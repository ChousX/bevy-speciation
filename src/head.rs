use crate::{
    anatomical_features::{AnatomicalFeature, MandibleStructure, SensoryOrgan},
    primitives::LocalPosition,
    skeletal::{Bone, Joint},
    sockets_symmetry::{Socket, SymmetricSocket},
    surface::Integument,
};

/// Defines how the head connects to the spine
#[derive(Clone, Debug)]
pub struct NeckAttachment {
    /// Which vertebra the head attaches to (typically 0)
    pub vertebra_index: u8,
    /// Offset from vertebra to head origin
    pub offset: LocalPosition,
    /// Joint allowing head movement
    pub joint: Joint,
}

#[derive(Clone, Debug)]
pub struct Cranium {
    pub bone: Bone,
    pub neck: NeckAttachment,
    pub sensory_sockets: Vec<SymmetricSocket<SensoryOrgan>>,
    pub mandible_socket: Option<Socket<MandibleStructure>>,
    pub feature_sockets: Vec<SymmetricSocket<AnatomicalFeature>>,
    pub integument: Integument,
}
