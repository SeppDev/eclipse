#[derive(Debug)]
pub enum Type {
    Void,
    Bytes(usize),
    Boolean,
    Int(u8),
}
