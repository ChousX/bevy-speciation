use crate::primitives::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurvePoint {
    pub t: Normalized,
    pub value: f32,
}

#[derive(Clone, Debug)]
pub struct Curve {
    pub points: Vec<CurvePoint>,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AspectRatio(f32);

impl AspectRatio {
    pub fn new(value: f32) -> Option<Self> {
        (value > 0.0).then_some(Self(value))
    }
    pub fn value(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Debug)]
pub enum CrossSectionProfile {
    Circular,
    Elliptical(AspectRatio),
    Radial(Curve),
}
