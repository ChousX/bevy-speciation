use rand::{SeedableRng, rngs::StdRng};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GenomeSeed(pub u64);

impl From<GenomeSeed> for StdRng {
    fn from(value: GenomeSeed) -> Self {
        StdRng::seed_from_u64(value.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Normalized(f32);

impl Normalized {
    pub fn new(value: f32) -> Option<Self> {
        (0.0..=1.0).contains(&value).then_some(Self(value))
    }
    pub fn value(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Length(f32);

impl Length {
    pub fn new(value: f32) -> Option<Self> {
        (value > 0.0).then_some(Self(value))
    }
    pub fn value(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Radians(f32);

impl Radians {
    pub fn new(value: f32) -> Self {
        Self(value)
    }
    pub fn value(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Count(u8);

impl Count {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct ValueRange<T> {
    pub min: T,
    pub max: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertebraIndex(pub u8);
