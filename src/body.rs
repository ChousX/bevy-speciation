use crate::{
    anatomical_features::AnatomicalFeature, appendage::Appendage, primitives::*, skeletal::Bone,
    sockets_symmetry::SymmetricSocket, surface::Integument, tissue_muscle::TissueEnvelope,
};

#[derive(Clone, Debug)]
pub struct Vertebra {
    pub bone: Bone,
}

#[derive(Clone, Debug)]
pub struct SpinalAttachment<T> {
    pub vertebra_index: VertebraIndex,
    pub socket: SymmetricSocket<T>,
}

#[derive(Clone, Debug)]
pub struct Spine {
    pub vertebrae: NonEmpty<Vertebra>,
    pub appendages: Vec<SpinalAttachment<Appendage>>,
    pub features: Vec<SpinalAttachment<AnatomicalFeature>>,
}

#[derive(Clone, Debug)]
pub struct Torso {
    pub spine: Spine,
    pub base_tissue: TissueEnvelope,
    pub integument: Integument,
}
