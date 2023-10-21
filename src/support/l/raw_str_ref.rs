#[derive(Copy, Clone, Debug)]
pub(crate) struct RawStrRef(pub *const u8, pub usize);
