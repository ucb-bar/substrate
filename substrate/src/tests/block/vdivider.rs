use std::collections::HashMap;

use arcstr::ArcStr;
use serde::{Deserialize, Serialize};

use crate::{
    block::Block,
    schematic::{
        cell::SchematicCell,
        context::SchematicCtx,
        interface::{AnalogInterface, Port, SignalMap},
        HasSchematic,
    },
};

use super::resistor::Resistor;

#[derive(Debug, Clone, Copy)]
pub struct VDividerIntf {
    pub vdd: Port,
    pub vss: Port,
    pub vout: Port,
}

// AUTOGENERATED CODE BEGIN
#[derive(Debug, Clone, Copy)]
pub struct VDividerIntfUninitialized;

#[allow(clippy::new_ret_no_self)]
impl VDividerIntf {
    fn new() -> VDividerIntfUninitialized {
        VDividerIntfUninitialized
    }
}

impl AnalogInterface<VDivider> for VDividerIntf {
    type Uninitialized = VDividerIntfUninitialized;

    fn initialize(_intf: Self::Uninitialized, map: &mut SignalMap) -> Self {
        Self {
            vdd: map.register_port(),
            vss: map.register_port(),
            vout: map.register_port(),
        }
    }

    fn uninitialized(self) -> VDividerIntfUninitialized {
        VDividerIntfUninitialized
    }

    fn ports(&self) -> HashMap<ArcStr, Port> {
        HashMap::from_iter([
            ("vdd".into(), self.vdd),
            ("vss".into(), self.vss),
            ("vout".into(), self.vout),
        ])
    }
}
// AUTOGENERATED CODE END

#[derive(Serialize, Deserialize, Debug)]
pub struct VDivider {
    pub r1: usize,
    pub r2: usize,
}

impl Block for VDivider {
    fn id() -> ArcStr {
        arcstr::literal!("vdivider")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("vdivider_{}_{}", self.r1, self.r2)
    }
}

impl HasSchematic for VDivider {
    type Interface = VDividerIntf;

    fn schematic(&self, ctx: &mut SchematicCtx) -> SchematicCell<VDivider> {
        let mut cell = SchematicCell::<VDivider>::new(VDividerIntf::new());

        let res1 = ctx.generate(Resistor { r: self.r1 });
        let res2 = ctx.generate(Resistor { r: self.r2 });

        let res1 = cell.add_instance("Xr1", res1);
        let res2 = cell.add_instance("Xr2", res2);

        cell.connect(res1.intf().p, cell.intf().vdd);
        cell.connect(res1.intf().n, res2.intf().p);
        cell.connect(res2.intf().n, cell.intf().vss);

        cell
    }
}