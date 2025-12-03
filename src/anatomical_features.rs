use crate::{primitives::*, skeletal::BoneSegment, tissue_muscle::TissueEnvelope};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SensoryType {
    Ocular,
    Auditory,
    Olfactory,
    Tactile,
}

#[derive(Clone, Debug)]
pub struct SensoryOrgan {
    pub kind: SensoryType,
    pub size: Length,
}

#[derive(Clone, Debug)]
pub struct Protrusion {
    pub length: Length,
    pub curvature: Curve,
    pub tissue: TissueEnvelope,
}

#[derive(Clone, Debug)]
pub struct SpineRow {
    pub count: Count,
    pub spine_length: Length,
    pub spacing: Length,
}

#[derive(Clone, Debug)]
pub struct FinStructure {
    pub height: Length,
    pub length: Length,
    pub membrane_thickness: Length,
}

#[derive(Clone, Debug)]
pub struct MandibleStructure {
    pub segments: Vec<BoneSegment>,
}

#[derive(Clone, Debug)]
pub enum AnatomicalFeature {
    Sensory(SensoryOrgan),
    Horn(Protrusion),
    Spines(SpineRow),
    Fin(FinStructure),
    Mandible(MandibleStructure),
}
