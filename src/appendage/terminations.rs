use crate::primitives::*;
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
pub enum Terminus {
    Tapered,
    Claw { digits: DigitCount },
    Paw { digits: DigitCount },
    Hoof,
    Sucker,
    Pincer,
}
