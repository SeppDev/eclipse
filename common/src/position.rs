use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PositionRange {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub character: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, character: usize) -> Self {
        Self {
            line,
            column,
            character,
        }
    }
    pub fn to_range(self) -> PositionRange {
        PositionRange::new(self.clone(), self)
    }
    pub fn extend(self, second: Self) -> PositionRange {
        PositionRange::new(self, second)
    }
}

impl PositionRange {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    pub fn from(start: Self, end: Self) -> Self {
        Self {
            start: start.start,
            end: end.end,
        }
    }
    pub fn set_start(&mut self, position: Position) {
        self.start = position;
    }
    pub fn set_end(&mut self, position: Position) {
        self.end = position;
    }
}

impl std::fmt::Display for PositionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "col {}-{}, ln {}-{}",
            self.start.column, self.end.column, self.start.line, self.end.line
        )
    }
}

#[derive(Default, Clone)]
pub struct LocatedAt<T = ()> {
    pub position: PositionRange,
    pub raw: T,
}
impl<T> LocatedAt<T> {
    pub fn value(value: T) -> LocatedAt<T> {
        LocatedAt {
            raw: value,
            position: PositionRange::default(),
        }
    }
}
impl<T: Hash> Hash for LocatedAt<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}
impl<T> From<T> for LocatedAt<T> {
    fn from(value: T) -> Self {
        LocatedAt::new(value, PositionRange::default())
    }
}
impl<T> Into<Option<T>> for LocatedAt<T> {
    fn into(self) -> Option<T> {
        Some(self.raw)
    }
}
impl<T: PartialEq> PartialEq for LocatedAt<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
impl<T> LocatedAt<T> {
    pub fn new(raw: T, position: PositionRange) -> Self {
        Self { raw, position }
    }
}
impl<T: Debug> Debug for LocatedAt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?} ({})", self.raw, self.position)
    }
}
impl<T: Display> Display for LocatedAt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.raw, self.position)
    }
}
