use arcstr::ArcStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use substrate::block::Block;
use substrate::io::Io;
use substrate::io::{Array, InOut, Output, Signal};
use substrate::pdk::Pdk;
use substrate::schematic::primitives::Resistor;
use substrate::schematic::{CellBuilder, ExportsSchematicData, Instance, Schematic, SchematicData};

pub mod flattened;
pub mod tb;

#[derive(Debug, Default, Clone, Io)]
pub struct PowerIo {
    pub vdd: InOut<Signal>,
    pub vss: InOut<Signal>,
}

#[derive(Debug, Default, Clone, Io)]
pub struct VdividerIo {
    pub pwr: PowerIo,
    pub out: Output<Signal>,
}

#[derive(Debug, Default, Clone, Io)]
pub struct VdividerFlatIo {
    pub vdd: InOut<Signal>,
    pub vss: InOut<Signal>,
    pub out: Output<Signal>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vdivider {
    pub r1: Resistor,
    pub r2: Resistor,
}

impl Vdivider {
    #[inline]
    fn new(r1: impl Into<Decimal>, r2: impl Into<Decimal>) -> Self {
        Self {
            r1: Resistor::new(r1),
            r2: Resistor::new(r2),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct VdividerArray {
    pub vdividers: Vec<Vdivider>,
}

impl Block for Vdivider {
    type Io = VdividerIo;
    const FLATTEN: bool = false;

    fn id() -> ArcStr {
        arcstr::literal!("vdivider")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("vdivider_{}_{}", self.r1.value(), self.r2.value())
    }

    fn io(&self) -> Self::Io {
        Default::default()
    }
}

#[derive(Debug, Clone, Io)]
pub struct VdividerArrayIo {
    pub elements: Array<PowerIo>,
}

impl Block for VdividerArray {
    type Io = VdividerArrayIo;

    fn id() -> ArcStr {
        arcstr::literal!("vdivider_array")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("vdivider_array_{}", self.vdividers.len())
    }

    fn io(&self) -> Self::Io {
        VdividerArrayIo {
            elements: Array::new(self.vdividers.len(), Default::default()),
        }
    }
}

#[derive(SchematicData)]
pub struct VdividerData {
    #[substrate(nested)]
    r1: Instance<Resistor>,
    #[substrate(nested)]
    r2: Instance<Resistor>,
}

impl ExportsSchematicData for Vdivider {
    type Data = VdividerData;
}

impl ExportsSchematicData for VdividerArray {
    type Data = Vec<Instance<Vdivider>>;
}

impl<PDK: Pdk> Schematic<PDK> for Vdivider {
    fn schematic(
        &self,
        io: &VdividerIoSchematic,
        cell: &mut CellBuilder<PDK, Self>,
    ) -> substrate::error::Result<Self::Data> {
        let r1 = cell.instantiate(self.r1);
        let r2 = cell.instantiate(self.r2);

        cell.connect(io.pwr.vdd, r1.io().p);
        cell.connect(io.out, r1.io().n);
        cell.connect(io.out, r2.io().p);
        cell.connect(io.pwr.vss, r2.io().n);
        Ok(VdividerData { r1, r2 })
    }
}

impl<PDK: Pdk> Schematic<PDK> for VdividerArray {
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as substrate::io::SchematicType>::Bundle,
        cell: &mut CellBuilder<PDK, Self>,
    ) -> substrate::error::Result<Self::Data> {
        let mut vdividers = Vec::new();

        for (i, vdivider) in self.vdividers.iter().enumerate() {
            let vdiv = cell.instantiate(*vdivider);

            cell.connect(&vdiv.io().pwr, &io.elements[i]);

            vdividers.push(vdiv);
        }

        Ok(vdividers)
    }
}
