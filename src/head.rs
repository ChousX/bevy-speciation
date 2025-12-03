use crate::{
    anatomical_features::{AnatomicalFeature, MandibleStructure, SensoryOrgan},
    skeletal::Bone,
    sockets_symmetry::{Socket, SymmetricSocket},
    surface::Integument,
};

#[derive(Clone, Debug)]
pub struct Cranium {
    pub bone: Bone,
    pub sensory_sockets: Vec<SymmetricSocket<SensoryOrgan>>,
    pub mandible_socket: Option<Socket<MandibleStructure>>,
    pub feature_sockets: Vec<SymmetricSocket<AnatomicalFeature>>,
    pub integument: Integument,
}
