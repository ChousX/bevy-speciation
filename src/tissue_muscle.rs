use crate::primitives::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MuscleAttachment {
    pub position: Normalized,
    pub radial_angle: Radians,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MuscleIntensity(f32);

impl MuscleIntensity {
    pub fn new(value: f32) -> Option<Self> {
        (value >= 0.0).then_some(Self(value))
    }
    pub fn value(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MuscleSpread(f32);

impl MuscleSpread {
    pub fn new(value: f32) -> Option<Self> {
        (value > 0.0).then_some(Self(value))
    }
    pub fn value(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct MuscleBulge {
    pub attachment: MuscleAttachment,
    pub intensity: MuscleIntensity,
    pub spread: MuscleSpread,
}

#[derive(Clone, Debug)]
pub struct TissueEnvelope {
    pub profile: CrossSectionProfile,
    pub radius_curve: Curve,
    pub musculature: Vec<MuscleBulge>,
}
