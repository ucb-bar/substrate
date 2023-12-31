//! Substrate to SCIR conversion.

use std::collections::{HashMap, HashSet};

use arcstr::ArcStr;
use scir::{
    Cell, CellId as ScirCellId, CellInner, IndexOwned, Instance, InstancePathTail, LibraryBuilder,
    SignalPathTail, TopKind,
};
use uniquify::Names;

use crate::io::{Node, NodePath, TerminalPath};
use crate::schematic::{InstancePath, PrimitiveNode};

use super::{BlackboxElement, CellId, InstanceId, RawCell, RawCellContent};

/// An SCIR library with associated conversion metadata.
#[derive(Debug, Clone)]
pub struct RawLib {
    /// The SCIR library.
    pub scir: scir::Library,
    /// Associated conversion metadata.
    ///
    /// Can be used to retrieve SCIR objects from their corresponding Substrate IDs.
    pub conv: ScirLibConversion,
}

/// Metadata associated with a conversion from a Substrate schematic to a SCIR library.
///
/// Provides helpers for retrieving SCIR objects from their Substrate IDs.
#[derive(Debug, Clone)]
pub struct ScirLibConversion {
    pub(crate) id_mapping: HashMap<CellId, ScirCellId>,
    /// Map from Substrate cell IDs to cell conversion metadata.
    pub(crate) cells: HashMap<CellId, ScirCellConversion>,
    pub(crate) top: scir::CellId,
}

#[derive(Debug, Clone, Default)]
struct ScirLibConversionBuilder {
    pub(crate) id_mapping: HashMap<CellId, ScirCellId>,
    /// Map from Substrate cell IDs to cell conversion metadata.
    pub(crate) cells: HashMap<CellId, ScirCellConversion>,
    pub(crate) top: Option<scir::CellId>,
}

impl ScirLibConversionBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> ScirLibConversion {
        ScirLibConversion {
            id_mapping: self.id_mapping,
            cells: self.cells,
            top: self.top.unwrap(),
        }
    }

    #[inline]
    pub(crate) fn set_top(&mut self, id: CellId, scir_id: scir::CellId) {
        self.cells.get_mut(&id).unwrap().top = true;
        self.top = Some(scir_id);
    }

    #[inline]
    pub(crate) fn add_cell(&mut self, id: CellId, conv: ScirCellConversion) {
        self.cells.insert(id, conv);
    }
}

impl RawLib {
    fn convert_instance_path_inner<'a>(
        &self,
        top: CellId,
        instances: impl IntoIterator<Item = &'a InstanceId>,
    ) -> Option<(Vec<scir::InstanceId>, &ScirCellConversion, scir::CellId)> {
        let mut cell = self.conv.cells.get(&top)?;
        assert!(cell.top);
        let mut scir_id = self.conv.top;

        let mut scir_instances = Vec::new();
        for inst in instances {
            let conv = cell.instances.get(inst).unwrap();
            match conv.instance.as_ref() {
                RawCellContent::Opaque(id) => {
                    scir_id = self.scir.cell(scir_id).instance(*id).cell();
                    scir_instances.push(*id);
                    cell = self.conv.cells.get(&conv.child)?;
                }
                RawCellContent::Clear(conv) => {
                    cell = conv;
                }
            }
        }
        Some((scir_instances, cell, scir_id))
    }
    /// Converts a Substrate [`NodePath`] to a SCIR [`scir::SignalPath`].
    pub fn convert_node_path(&self, path: &NodePath) -> Option<scir::SignalPath> {
        let (instances, cell, id) = self.convert_instance_path_inner(path.top, &path.instances)?;

        let slice = *cell.signals.get(&path.node)?;

        Some(scir::SignalPath {
            tail: SignalPathTail::Scir { cell: id, slice },
            instances,
            top: self.conv.top,
        })
    }

    /// Converts a Substrate [`InstancePath`] to a SCIR [`scir::InstancePath`].
    pub fn convert_instance_path(&self, path: &InstancePath) -> Option<scir::InstancePath> {
        let (instances, _, scir_id) = self.convert_instance_path_inner(path.top, &path.path)?;
        Some(scir::InstancePath {
            top: self.conv.top,
            instances,
            tail: InstancePathTail::Scir(scir_id),
        })
    }

    /// Converts a Substrate [`TerminalPath`] to a list SCIR [`scir::SignalPath`]s that are
    /// associated with the terminal at that path.
    ///
    /// Returns [`None`] if the path is invalid. Only flattened instances will
    /// return more than one [`scir::SignalPath`], and unconnected terminals will return
    /// `Some(vec![])`.
    pub fn convert_terminal_path(&self, path: &TerminalPath) -> Option<Vec<scir::SignalPath>> {
        let mut cell = self.conv.cells.get(&path.top)?;
        assert!(cell.top);

        let mut instances = Vec::new();
        let mut last_clear = false;
        let mut scir_id = self.conv.top;
        for inst in &path.instances {
            let conv = cell.instances.get(inst).unwrap();
            match conv.instance.as_ref() {
                RawCellContent::Opaque(id) => {
                    scir_id = self.scir.cell(scir_id).instance(*id).cell();
                    instances.push(*id);
                    cell = self.conv.cells.get(&conv.child)?;
                    last_clear = false;
                }
                RawCellContent::Clear(conv) => {
                    cell = conv;
                    last_clear = true;
                }
            }
        }

        // If the last cell in the conversion was `Opacity::Clear`, the provided terminal
        // virtual and thus may correspond to more than one `scir::SignalPath`.
        //
        // Run DFS to find all signal paths that are directly connected to this virtual
        // terminal.
        let slice = *cell.signals.get(&path.node)?;
        Some(if last_clear {
            let mut signals = Vec::new();
            self.find_connected_terminals(cell, slice, &mut instances, &mut signals);
            signals
        } else {
            vec![scir::SignalPath {
                tail: SignalPathTail::Scir {
                    cell: scir_id,
                    slice,
                },
                instances,
                top: self.conv.top,
            }]
        })
    }

    /// Must ensure that `instances` is returned to its original value by the end of the
    /// function call.
    fn find_connected_terminals_in_scir_instance(
        &self,
        parent_cell: &scir::Cell,
        id: scir::InstanceId,
        slice: scir::SliceOne,
        instances: &mut Vec<scir::InstanceId>,
        signals: &mut Vec<scir::SignalPath>,
    ) {
        // let (signal, index) = slice;
        instances.push(id);
        let inst = parent_cell.instance(id);
        for (name, conn) in inst.connections() {
            let mut port_index = 0;
            for part in conn.parts() {
                if slice.signal() == part.signal() {
                    let concat_index = match (slice.index(), part.range()) {
                        (None, None) => Some(port_index),
                        (Some(index), Some(range)) => {
                            if range.contains(index) {
                                Some(port_index + index - range.start())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    if let Some(concat_index) = concat_index {
                        let child_cell = self.scir.cell(inst.cell());
                        let port = child_cell.port(name);
                        let port_slice = child_cell.signal(port.signal()).slice();
                        let tail = port_slice
                            .slice_one()
                            .unwrap_or_else(|| port_slice.index(concat_index));
                        signals.push(scir::SignalPath {
                            tail: SignalPathTail::Scir {
                                cell: inst.cell(),
                                slice: tail,
                            },
                            instances: instances.clone(),
                            top: self.conv.top,
                        });
                    }
                }
                port_index += part.width();
            }
        }
        instances.pop().unwrap();
    }

    /// Must ensure that `instances` is returned to its original value by the end of the
    /// function call.
    fn find_connected_terminals(
        &self,
        conv: &ScirCellConversion,
        slice: scir::SliceOne,
        instances: &mut Vec<scir::InstanceId>,
        signals: &mut Vec<scir::SignalPath>,
    ) {
        let parent_cell = self.scir.cell(self.conv.id_mapping[&conv.id]);
        for primitive in &conv.primitives {
            match primitive {
                ScirPrimitiveDeviceConversion::Primitive { id, nodes } => {
                    for node in nodes {
                        if &slice == conv.signals.get(&node.node).unwrap() {
                            signals.push(scir::SignalPath {
                                tail: SignalPathTail::Primitive {
                                    id: *id,
                                    name_path: vec![node.port.clone()],
                                },
                                instances: instances.clone(),
                                top: self.conv.top,
                            })
                        }
                    }
                }
                ScirPrimitiveDeviceConversion::Instance(id) => {
                    self.find_connected_terminals_in_scir_instance(
                        parent_cell,
                        *id,
                        slice,
                        instances,
                        signals,
                    );
                }
            }
        }
        for (_, conv) in conv.instances.iter() {
            match conv.instance.as_ref() {
                RawCellContent::Opaque(id) => {
                    self.find_connected_terminals_in_scir_instance(
                        parent_cell,
                        *id,
                        slice,
                        instances,
                        signals,
                    );
                }
                RawCellContent::Clear(conv) => {
                    self.find_connected_terminals(conv, slice, instances, signals);
                }
            }
        }
    }
}

/// A converted SCIR instance.
type ConvertedScirInstance = RawCellContent<scir::InstanceId, ScirCellConversion>;

/// Data used to map between a Substrate cell and a SCIR cell.
///
/// Flattened cells do not have a conversion.
#[derive(Debug, Clone)]
pub(crate) struct ScirCellConversion {
    /// The Substrate cell ID that this conversion corresponds to.
    pub(crate) id: CellId,
    /// Whether or not this cell is the top cell.
    pub(crate) top: bool,
    /// Map Substrate nodes to SCIR signal IDs and indices.
    pub(crate) signals: HashMap<Node, scir::SliceOne>,
    /// Map Substrate instance IDs to SCIR instances and their underlying Substrate cell.
    pub(crate) instances: HashMap<InstanceId, ScirInstanceConversion>,
    pub(crate) primitives: Vec<ScirPrimitiveDeviceConversion>,
}

impl ScirCellConversion {
    #[inline]
    pub fn new(id: CellId) -> Self {
        Self {
            id,
            top: false,
            signals: HashMap::new(),
            instances: HashMap::new(),
            primitives: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ScirInstanceConversion {
    /// The Substrate cell ID of the child cell.
    child: CellId,
    /// The SCIR instance.
    ///
    /// If the instance is not inlined/flattened, this will be an opaque instance ID.
    /// If the instance is inlined, this will be a [`ScirCellConversion`].
    instance: ConvertedScirInstance,
}

#[derive(Debug, Clone)]
pub(crate) enum ScirPrimitiveDeviceConversion {
    /// A Substrate primitive that translates to a [`scir::PrimitiveDevice`].
    Primitive {
        /// The SCIR ID of the translated primitive device.
        id: scir::PrimitiveDeviceId,
        /// The nodes connected to this SCIR primitive.
        nodes: Vec<PrimitiveNode>,
    },
    /// A Substrate primitive that translates to a [`scir::Instance`].
    Instance(scir::InstanceId),
}

#[derive(Debug, Clone)]
struct ScirExportData {
    lib: LibraryBuilder,
    conv: ScirLibConversionBuilder,
    cell_names: Names<CellId>,
}

impl ScirExportData {
    fn new(name: impl Into<ArcStr>) -> Self {
        Self {
            lib: LibraryBuilder::new(name),
            conv: ScirLibConversionBuilder::new(),
            cell_names: Names::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
enum FlatExport {
    Yes(Vec<scir::SliceOne>),
    #[default]
    No,
}

impl FlatExport {
    #[inline]
    pub fn is_yes(&self) -> bool {
        matches!(self, FlatExport::Yes(_))
    }

    #[inline]
    pub fn is_no(&self) -> bool {
        !self.is_yes()
    }
}

struct ScirExportContext {
    id: CellId,
    inst_idx: u64,
    prim_idx: u64,
    cell: scir::Cell,
}

impl ScirExportContext {
    #[inline]
    pub fn new(id: CellId, cell: scir::Cell) -> Self {
        Self {
            id,
            inst_idx: 0,
            prim_idx: 0,
            cell,
        }
    }

    fn whitebox_contents_mut(&mut self) -> &mut CellInner {
        self.cell.contents_mut().as_mut().unwrap_clear()
    }
}

impl RawCell {
    /// Export this cell and all subcells as a SCIR library.
    ///
    /// Returns the SCIR library and metadata for converting between SCIR and Substrate formats.
    pub(crate) fn to_scir_lib(&self, kind: TopKind) -> Result<RawLib, scir::Issues> {
        let mut data = ScirExportData::new(self.name.clone());
        let scir_id = self.to_scir_cell(&mut data);
        data.lib.set_top(scir_id, kind);
        data.conv.set_top(self.id, scir_id);

        Ok(RawLib {
            scir: data.lib.build()?,
            conv: data.conv.build(),
        })
    }

    fn to_scir_cell(&self, data: &mut ScirExportData) -> ScirCellId {
        let name = data.cell_names.assign_name(self.id, &self.name);

        // Create the SCIR cell as a whitebox for now.
        // If this Substrate cell is actually a blackbox,
        // the contents of this SCIR cell will be made into a blackbox
        // by calling `cell.set_contents`.
        let cell = Cell::new_whitebox(name);

        let mut ctx = ScirExportContext::new(self.id, cell);
        let conv = self.to_scir_cell_inner(data, &mut ctx, FlatExport::No);
        let ScirExportContext { cell, .. } = ctx;

        let id = data.lib.add_cell(cell);
        data.conv.add_cell(self.id, conv);
        data.conv.id_mapping.insert(self.id, id);

        id
    }

    fn to_scir_cell_inner(
        &self,
        data: &mut ScirExportData,
        ctx: &mut ScirExportContext,
        flatten: FlatExport,
    ) -> ScirCellConversion {
        if flatten.is_yes() {
            assert!(
                self.contents.is_clear(),
                "cannot flat-export a blackbox cell"
            );
        }

        let mut conv = ScirCellConversion::new(ctx.id);
        let mut nodes = HashMap::new();
        let mut roots_added = HashSet::new();

        if let FlatExport::Yes(ref ports) = flatten {
            // Flattened cells need to add all non-IO nodes to the enclosing cell.
            assert_eq!(ports.len(), self.ports.len());
            for (port, s) in self.ports.iter().zip(ports) {
                let root = self.roots[&port.node()];
                roots_added.insert(root);
                nodes.insert(root, *s);
            }
        }

        for (&src, &root) in self.roots.iter() {
            let s = if !roots_added.contains(&root) {
                let s = ctx.cell.add_node(self.node_name(root));
                roots_added.insert(root);
                nodes.insert(root, s);
                s
            } else {
                nodes[&root]
            };
            nodes.insert(src, s);
            conv.signals.insert(src, s);
        }

        match self.contents.as_ref() {
            RawCellContent::Opaque(contents) => {
                assert!(flatten.is_no(), "cannot flat-export a blackbox cell");
                let transformed = contents
                    .elems
                    .iter()
                    .map(|e| match e {
                        BlackboxElement::RawString(s) => {
                            scir::BlackboxElement::RawString(s.clone())
                        }
                        BlackboxElement::Node(n) => scir::BlackboxElement::Slice(nodes[n].into()),
                    })
                    .collect();
                ctx.cell
                    .set_contents(scir::CellContent::Opaque(transformed));
            }
            RawCellContent::Clear(contents) => {
                let contents_mut = ctx.cell.contents_mut().as_mut();
                let clear = contents_mut.is_clear();
                assert!(clear, "cannot flatten a cell into a blackbox parent cell");
                for instance in contents.instances.iter() {
                    if instance.child.flatten {
                        let ports = instance.connections.iter().map(|c| nodes[c]).collect();
                        let inst_conv =
                            instance
                                .child
                                .to_scir_cell_inner(data, ctx, FlatExport::Yes(ports));
                        conv.instances.insert(
                            instance.id,
                            ScirInstanceConversion {
                                child: instance.child.id,
                                instance: RawCellContent::Clear(inst_conv),
                            },
                        );
                    } else {
                        if !data.conv.id_mapping.contains_key(&instance.child.id) {
                            instance.child.to_scir_cell(data);
                        }
                        let child: ScirCellId =
                            *data.conv.id_mapping.get(&instance.child.id).unwrap();

                        let mut sinst =
                            Instance::new(arcstr::format!("inst{}", ctx.inst_idx), child);
                        ctx.inst_idx += 1;
                        assert_eq!(instance.child.ports.len(), instance.connections.len());
                        for (port, &conn) in instance.child.ports.iter().zip(&instance.connections)
                        {
                            let scir_port_name = instance.child.node_name(port.node());
                            sinst.connect(scir_port_name, nodes[&conn]);
                        }
                        let id = ctx.whitebox_contents_mut().add_instance(sinst);
                        conv.instances.insert(
                            instance.id,
                            ScirInstanceConversion {
                                child: instance.child.id,
                                instance: RawCellContent::Opaque(id),
                            },
                        );
                    }
                }
                for p in contents.primitives.iter() {
                    let i = ctx.prim_idx;
                    ctx.prim_idx += 1;
                    match &p.kind {
                        super::PrimitiveDeviceKind::Res2 { pos, neg, value } => {
                            let id = ctx.whitebox_contents_mut().add_primitive(
                                scir::PrimitiveDevice::from_params(
                                    arcstr::format!("res{i}"),
                                    scir::PrimitiveDeviceKind::Res2 {
                                        pos: nodes[&pos.node],
                                        neg: nodes[&neg.node],
                                        value: scir::Expr::NumericLiteral(*value),
                                    },
                                    p.params.clone(),
                                ),
                            );
                            conv.primitives
                                .push(ScirPrimitiveDeviceConversion::Primitive {
                                    id,
                                    nodes: vec![pos.clone(), neg.clone()],
                                });
                        }
                        super::PrimitiveDeviceKind::Cap2 { pos, neg, value } => {
                            let id = ctx.whitebox_contents_mut().add_primitive(
                                scir::PrimitiveDevice::from_params(
                                    arcstr::format!("cap{i}"),
                                    scir::PrimitiveDeviceKind::Cap2 {
                                        pos: nodes[&pos.node],
                                        neg: nodes[&neg.node],
                                        value: scir::Expr::NumericLiteral(*value),
                                    },
                                    p.params.clone(),
                                ),
                            );
                            conv.primitives
                                .push(ScirPrimitiveDeviceConversion::Primitive {
                                    id,
                                    nodes: vec![pos.clone(), neg.clone()],
                                });
                        }
                        super::PrimitiveDeviceKind::RawInstance { ports, cell } => {
                            let id = ctx.whitebox_contents_mut().add_primitive(
                                scir::PrimitiveDevice::from_params(
                                    arcstr::format!("rawinst{i}"),
                                    scir::PrimitiveDeviceKind::RawInstance {
                                        ports: ports.iter().map(|p| nodes[&p.node]).collect(),
                                        cell: cell.clone(),
                                    },
                                    p.params.clone(),
                                ),
                            );
                            conv.primitives
                                .push(ScirPrimitiveDeviceConversion::Primitive {
                                    id,
                                    nodes: ports.clone(),
                                });
                        }
                        super::PrimitiveDeviceKind::ScirInstance {
                            lib,
                            cell,
                            connections,
                        } => {
                            let mapping = data.lib.merge(lib);
                            let cell = mapping.new_cell_id(*cell);
                            let mut inst =
                                scir::Instance::new(arcstr::format!("scirinst{i}"), cell);

                            for (port, elems) in connections {
                                let concat: scir::Concat = elems.iter().map(|n| nodes[n]).collect();
                                inst.connect(port, concat);
                            }

                            for (key, value) in p.params.iter() {
                                inst.set_param(key, value.clone());
                            }
                            let id = ctx.whitebox_contents_mut().add_instance(inst);
                            conv.primitives
                                .push(ScirPrimitiveDeviceConversion::Instance(id));
                        }
                    };
                }
            }
        }

        if flatten.is_no() {
            for port in self.ports.iter() {
                ctx.cell.expose_port(nodes[&port.node()], port.direction());
            }
        }

        conv
    }

    /// The name associated with the given node.
    ///
    /// # Panics
    ///
    /// Panics if the node does not exist within this cell.
    fn node_name(&self, node: Node) -> String {
        let node = self.roots[&node];
        self.node_names[&node].to_string()
    }
}
