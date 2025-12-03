use crate::primitives::{Count, VertebraIndex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InclusiveRange<T> {
    pub start: T,
    pub end: T,
}

impl<T: Copy> InclusiveRange<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> T {
        self.start
    }
    pub fn end(&self) -> T {
        self.end
    }
}

impl<T: Copy + PartialOrd> InclusiveRange<T> {
    pub fn contains(&self, value: T) -> bool {
        value >= self.start && value <= self.end
    }
}

impl InclusiveRange<Count> {
    pub fn iter(&self) -> impl Iterator<Item = Count> {
        (self.start.value()..=self.end.value()).map(Count::new)
    }
}

impl InclusiveRange<VertebraIndex> {
    pub fn iter(&self) -> impl Iterator<Item = VertebraIndex> {
        (self.start.0..=self.end.0).map(VertebraIndex)
    }
}
