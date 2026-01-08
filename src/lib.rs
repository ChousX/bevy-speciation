mod anatomical_features;
mod appendage;
mod body;
mod head;
mod organism;
mod primitives;
mod skeletal;
pub mod skeleton;
mod sockets_symmetry;
mod species;
mod surface;
mod tissue_muscle;
mod validation_errors;

// Re-export key types for skeleton generation
pub use organism::Organism;
pub use skeleton::{GeneratedSkeleton, SkeletonGenerator};
