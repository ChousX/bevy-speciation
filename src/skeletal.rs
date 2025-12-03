use crate::{primitives::*, tissue_muscle::TissueEnvelope};
#[derive(Clone, Copy, Debug)]
pub struct ArticulationRange {
    pub min: Radians,
    pub max: Radians,
}

#[derive(Clone, Copy, Debug)]
pub struct JointArticulation {
    pub flexion: ArticulationRange,
    pub rotation: ArticulationRange,
    pub abduction: ArticulationRange,
}

#[derive(Clone, Debug)]
pub struct Bone {
    pub length: Length,
    pub tissue: TissueEnvelope,
}

#[derive(Clone, Debug)]
pub struct Joint {
    pub articulation: JointArticulation,
}

#[derive(Clone, Debug)]
pub struct BoneSegment {
    pub bone: Bone,
    pub distal_joint: Option<Joint>,
}
