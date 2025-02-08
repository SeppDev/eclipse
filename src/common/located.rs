use super::position::PositionRange;

#[derive(Debug, Default, Clone)]
pub struct Located<T> {
    pub raw: T,
    pub position: PositionRange,
}
impl<T> Located<T> {
    pub fn new(raw: T, position: PositionRange) -> Self {
        Self { raw, position }
    }
}
