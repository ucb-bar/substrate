#[derive(Io, Clone, Default)]
pub struct BufferIo {
    vdd: InOut<Signal>,
    vss: InOut<Signal>,
    din: Input<Signal>,
    dout: Output<Signal>,
}