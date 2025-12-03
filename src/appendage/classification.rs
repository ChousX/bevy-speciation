use crate::{appendage::Terminus, primitives::*, skeletal::BoneSegment, surface::Integument};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppendageClass {
    Forelimb,
    Hindlimb,
    Wing,
    Tentacle,
    Tail,
    Antenna,
}

#[derive(Clone, Debug)]
pub struct MembraneSpan {
    pub proximal_bone: usize,
    pub distal_bone: usize,
    pub thickness: Length,
    pub scallop_depth: Normalized,
}

#[derive(Clone, Debug)]
pub struct Patagium {
    pub spans: Vec<MembraneSpan>,
}

#[derive(Clone, Debug)]
pub struct BranchPoint {
    pub parent_segment: usize,
    pub branch_count: Count,
    pub branch: Box<LimbStructure>,
}

#[derive(Clone, Debug)]
pub struct LimbStructure {
    pub segments: Vec<BoneSegment>,
    pub branching: Option<BranchPoint>,
    pub terminus: Terminus,
}

#[derive(Clone, Debug)]
pub struct Appendage {
    pub class: AppendageClass,
    pub structure: LimbStructure,
    pub patagium: Option<Patagium>,
    pub integument: Integument,
}
