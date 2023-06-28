use arcstr::ArcStr;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use substrate::io::*;
use substrate::schematic::*;

use substrate::pdk::Pdk;
use substrate::Io;
use substrate::{block::Block, schematic::HasSchematic};

#[derive(Debug, Default, Clone, Io)]
pub struct ResistorIo {
    pub p: InOut<Signal>,
    pub n: InOut<Signal>,
}

// BEGIN AUTOGENERATED

impl LayoutType for ResistorIo {
    type Data = ResistorIoLayout;
    type Builder = ResistorIoLayoutBuilder;

    fn builder(&self) -> Self::Builder {
        Self::Builder {
            p: self.p.builder(),
            n: self.n.builder(),
        }
    }
}

pub struct ResistorIoLayout {
    pub p: InOut<PortGeometry>,
    pub n: InOut<PortGeometry>,
}

pub struct ResistorIoLayoutBuilder {
    pub p: InOut<PortGeometryBuilder>,
    pub n: InOut<PortGeometryBuilder>,
}

impl FlatLen for ResistorIoLayout {
    fn len(&self) -> usize {
        2
    }
}

impl Flatten<PortGeometry> for ResistorIoLayout {
    fn flatten<E>(&self, output: &mut E)
    where
        E: Extend<PortGeometry>,
    {
        self.p.flatten(output);
        self.n.flatten(output);
    }
}

impl FlatLen for ResistorIoLayoutBuilder {
    fn len(&self) -> usize {
        2
    }
}

impl LayoutDataBuilder<ResistorIoLayout> for ResistorIoLayoutBuilder {
    fn build(self) -> substrate::error::Result<ResistorIoLayout> {
        Ok(ResistorIoLayout {
            p: self.p.build()?,
            n: self.n.build()?,
        })
    }
}

// END AUTOGENERATED

#[derive(Debug, Default, Clone, Io)]
pub struct PowerIo {
    vdd: InOut<Signal>,
    vss: InOut<Signal>,
}

// BEGIN AUTOGENERATED

impl LayoutType for PowerIo {
    type Data = PowerIoLayout;
    type Builder = PowerIoLayoutBuilder;

    fn builder(&self) -> Self::Builder {
        Self::Builder {
            vdd: self.vdd.builder(),
            vss: self.vss.builder(),
        }
    }
}

pub struct PowerIoLayout {
    pub vdd: InOut<PortGeometry>,
    pub vss: InOut<PortGeometry>,
}

pub struct PowerIoLayoutBuilder {
    pub vdd: InOut<PortGeometryBuilder>,
    pub vss: InOut<PortGeometryBuilder>,
}

impl FlatLen for PowerIoLayout {
    fn len(&self) -> usize {
        2
    }
}

impl Flatten<PortGeometry> for PowerIoLayout {
    fn flatten<E>(&self, output: &mut E)
    where
        E: Extend<PortGeometry>,
    {
        self.vdd.flatten(output);
        self.vss.flatten(output);
    }
}

impl FlatLen for PowerIoLayoutBuilder {
    fn len(&self) -> usize {
        2
    }
}

impl LayoutDataBuilder<PowerIoLayout> for PowerIoLayoutBuilder {
    fn build(self) -> substrate::error::Result<PowerIoLayout> {
        Ok(PowerIoLayout {
            vdd: self.vdd.build()?,
            vss: self.vss.build()?,
        })
    }
}

// END AUTOGENERATED

#[derive(Debug, Default, Clone, Io)]
pub struct VdividerIo {
    pub pwr: PowerIo,
    pub out: Output<Signal>,
}

// BEGIN AUTOGENERATED

impl LayoutType for VdividerIo {
    type Data = VdividerIoLayout;
    type Builder = VdividerIoLayoutBuilder;

    fn builder(&self) -> Self::Builder {
        Self::Builder {
            pwr: self.pwr.builder(),
            out: self.out.builder(),
        }
    }
}

pub struct VdividerIoLayout {
    pub pwr: PowerIoLayout,
    pub out: Output<PortGeometry>,
}

pub struct VdividerIoLayoutBuilder {
    pub pwr: PowerIoLayoutBuilder,
    pub out: Output<PortGeometryBuilder>,
}

impl FlatLen for VdividerIoLayout {
    fn len(&self) -> usize {
        2
    }
}

impl Flatten<PortGeometry> for VdividerIoLayout {
    fn flatten<E>(&self, output: &mut E)
    where
        E: Extend<PortGeometry>,
    {
        self.pwr.flatten(output);
        self.out.flatten(output);
    }
}

impl FlatLen for VdividerIoLayoutBuilder {
    fn len(&self) -> usize {
        2
    }
}

impl LayoutDataBuilder<VdividerIoLayout> for VdividerIoLayoutBuilder {
    fn build(self) -> substrate::error::Result<VdividerIoLayout> {
        Ok(VdividerIoLayout {
            pwr: self.pwr.build()?,
            out: self.out.build()?,
        })
    }
}

// END AUTOGENERATED

#[derive(Debug, Default, Clone, Io)]
pub struct ArrayIo {
    pub inputs: Input<Array<Signal>>,
    pub out: Output<Signal>,
}

// BEGIN AUTOGENERATED

impl LayoutType for ArrayIo {
    type Data = ArrayIoLayout;
    type Builder = ArrayIoLayoutBuilder;

    fn builder(&self) -> Self::Builder {
        Self::Builder {
            inputs: self.inputs.builder(),
            out: self.out.builder(),
        }
    }
}

pub struct ArrayIoLayout {
    pub inputs: Input<ArrayData<PortGeometry>>,
    pub out: Output<PortGeometry>,
}

pub struct ArrayIoLayoutBuilder {
    pub inputs: Input<ArrayData<PortGeometryBuilder>>,
    pub out: Output<PortGeometryBuilder>,
}

impl FlatLen for ArrayIoLayout {
    fn len(&self) -> usize {
        self.inputs.len() + self.out.len()
    }
}

impl Flatten<PortGeometry> for ArrayIoLayout {
    fn flatten<E>(&self, output: &mut E)
    where
        E: Extend<PortGeometry>,
    {
        self.inputs.flatten(output);
        self.out.flatten(output);
    }
}

impl FlatLen for ArrayIoLayoutBuilder {
    fn len(&self) -> usize {
        self.inputs.len() + self.out.len()
    }
}

impl LayoutDataBuilder<ArrayIoLayout> for ArrayIoLayoutBuilder {
    fn build(self) -> substrate::error::Result<ArrayIoLayout> {
        Ok(ArrayIoLayout {
            inputs: self.inputs.build()?,
            out: self.out.build()?,
        })
    }
}

// END AUTOGENERATED

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resistor {
    pub r: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vdivider {
    pub r1: Resistor,
    pub r2: Resistor,
}

/// Shorts all input signals to an output node.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayShorter {
    width: usize,
}

impl Block for Resistor {
    type Io = ResistorIo;

    fn id() -> ArcStr {
        arcstr::literal!("resistor")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("resistor_{}", self.r)
    }

    fn io(&self) -> Self::Io {
        Default::default()
    }
}

impl Block for Vdivider {
    type Io = VdividerIo;

    fn id() -> ArcStr {
        arcstr::literal!("vdivider")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("vdivider_{}_{}", self.r1.name(), self.r2.name())
    }

    fn io(&self) -> Self::Io {
        Default::default()
    }
}

impl Block for ArrayShorter {
    type Io = ArrayIo;

    fn id() -> ArcStr {
        arcstr::literal!("array_shorter")
    }
    fn name(&self) -> ArcStr {
        arcstr::format!("array_shorter_{}", self.width)
    }
    fn io(&self) -> Self::Io {
        Self::Io {
            inputs: Input(Array::new(self.width, Signal)),
            out: Output(Signal),
        }
    }
}

impl HasSchematic for Resistor {
    type Data = ();
}

impl HasSchematic for Vdivider {
    type Data = ();
}

impl HasSchematic for ArrayShorter {
    type Data = ();
}

impl<PDK: Pdk> HasSchematicImpl<PDK> for Resistor {
    fn schematic(
        &self,
        io: ResistorIoData,
        cell: &mut CellBuilder<PDK, Self>,
    ) -> substrate::error::Result<Self::Data> {
        cell.add_primitive(PrimitiveDevice::Res2 {
            pos: *io.p,
            neg: *io.n,
            value: dec!(1000),
        });
        Ok(())
    }
}

impl<PDK: Pdk> HasSchematicImpl<PDK> for Vdivider {
    fn schematic(
        &self,
        io: VdividerIoData,
        cell: &mut CellBuilder<PDK, Self>,
    ) -> substrate::error::Result<Self::Data> {
        let r1 = cell.instantiate(self.r1);
        let r2 = cell.instantiate(self.r2);

        cell.connect(io.pwr.vdd, r1.io.p);
        cell.connect(io.out, r1.io.n);
        cell.connect(io.out, r2.io.p);
        cell.connect(io.pwr.vss, r2.io.n);
        Ok(())
    }
}

impl<PDK: Pdk> HasSchematicImpl<PDK> for ArrayShorter {
    fn schematic(
        &self,
        io: ArrayIoData,
        cell: &mut CellBuilder<PDK, Self>,
    ) -> substrate::error::Result<Self::Data> {
        for i in 0..self.width {
            cell.connect(io.inputs[i], io.out)
        }
        Ok(())
    }
}