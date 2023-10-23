#[derive(Clone, Copy, Debug)]
pub struct RawLocation(pub *const u8, pub usize, pub u32, pub u32);
