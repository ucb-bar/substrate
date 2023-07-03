let mut ctx = Context::new(ExamplePdk);
let handle = ctx.generate_layout(Buffer::new(5));
let cell: &Cell<Buffer> = handle.wait().as_ref().unwrap();

assert_eq!(cell.block, Buffer::new(5));
assert_eq!(cell.data.inv1.cell().block, &Inverter::new(5));
assert_eq!(cell.data.inv2.cell().block, &Inverter::new(5));

assert_eq!(
    cell.data.inv1.bbox(),
    Some(Rect::from_sides(0, 0, 100, 200))
);

assert_eq!(
    cell.data.inv2.bbox(),
    Some(Rect::from_sides(110, 0, 210, 200))
);

assert_eq!(cell.bbox(), Some(Rect::from_sides(0, 0, 210, 200)));
