use crate::primitives::Length;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DigitCount(u8);

impl DigitCount {
    pub fn new(value: u8) -> Option<Self> {
        (value > 0).then_some(Self(value))
    }
    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct DigitConfig {
    pub count: DigitCount,
    pub length: Length,
}

#[derive(Clone, Debug)]
pub struct PincerConfig {
    pub length: Length,
}

#[derive(Clone, Debug)]
pub enum Terminus {
    Tapered,
    Claw(DigitConfig),
    Paw(DigitConfig),
    Hoof,
    Sucker,
    Pincer(PincerConfig),
}

impl Terminus {
    /// Convenience constructor for claws
    pub fn claw(count: u8, length: Length) -> Option<Self> {
        Some(Self::Claw(DigitConfig {
            count: DigitCount::new(count)?,
            length,
        }))
    }

    /// Convenience constructor for paws
    pub fn paw(count: u8, length: Length) -> Option<Self> {
        Some(Self::Paw(DigitConfig {
            count: DigitCount::new(count)?,
            length,
        }))
    }

    /// Convenience constructor for pincers
    pub fn pincer(length: Length) -> Self {
        Self::Pincer(PincerConfig { length })
    }
}
