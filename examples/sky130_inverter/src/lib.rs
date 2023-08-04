// begin-code-snippet imports
use serde::{Deserialize, Serialize};
use sky130pdk::mos::{Nfet01v8, Pfet01v8};
use sky130pdk::{Sky130CommercialPdk, Sky130OpenPdk};
use substrate::block::Block;
use substrate::io::{InOut, Input, Output, Signal};
use substrate::schematic::{HasSchematic, HasSchematicData};
use substrate::type_dispatch::impl_dispatch;
use substrate::Block;
use substrate::Io;
// end-code-snippet imports

pub mod tb;

// begin-code-snippet inverter-io
#[derive(Io, Clone, Default, Debug)]
pub struct InverterIo {
    pub vdd: InOut<Signal>,
    pub vss: InOut<Signal>,
    pub din: Input<Signal>,
    pub dout: Output<Signal>,
}
// end-code-snippet inverter-io

// begin-code-snippet inverter-struct
#[derive(Serialize, Deserialize, Block, Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[substrate(io = "InverterIo")]
pub struct Inverter {
    /// NMOS width.
    pub nw: i64,
    /// PMOS width.
    pub pw: i64,
    /// Channel length.
    pub lch: i64,
}
// end-code-snippet inverter-struct

// begin-code-snippet inverter-schematic
impl HasSchematicData for Inverter {
    type Data = ();
}

#[impl_dispatch({Sky130CommercialPdk; Sky130OpenPdk})]
impl<Pdk> HasSchematic<Pdk> for Inverter {
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as substrate::io::SchematicType>::Data,
        cell: &mut substrate::schematic::CellBuilder<Pdk, Self>,
    ) -> substrate::error::Result<Self::Data> {
        let nmos = cell.instantiate(Nfet01v8::new((self.nw, self.lch)));
        cell.connect(io.dout, nmos.io().d);
        cell.connect(io.din, nmos.io().g);
        cell.connect(io.vss, nmos.io().s);
        cell.connect(io.vss, nmos.io().b);

        let pmos = cell.instantiate(Pfet01v8::new((self.pw, self.lch)));
        cell.connect(io.dout, pmos.io().d);
        cell.connect(io.din, pmos.io().g);
        cell.connect(io.vdd, pmos.io().s);
        cell.connect(io.vdd, pmos.io().b);

        Ok(())
    }
}
// end-code-snippet inverter-schematic
