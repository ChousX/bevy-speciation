use crate::{appendage::AppendageClass, primitives::*};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpeciesValidationError {
    EmptyVertebraRange,
    SocketIndexOutOfBounds {
        socket_index: VertebraIndex,
        max_vertebrae: Count,
    },
    EmptyAllowedList {
        context: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganismValidationError {
    VertebraIndexOutOfBounds {
        index: VertebraIndex,
        vertebra_count: usize,
    },
    AppendageClassMismatch {
        expected: AppendageClass,
        found: AppendageClass,
    },
    RequiredSocketEmpty {
        vertebra_index: VertebraIndex,
    },
    MissingHead,
}

#[derive(Clone, Debug)]
pub enum GenerationError {
    InvalidSeed,
    ConstraintUnsatisfiable { context: String },
}
