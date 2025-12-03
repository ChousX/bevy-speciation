use crate::primitives::*;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Roughness(pub Normalized);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metallic(pub Normalized);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SurfacePattern {
    Smooth,
    Scaled,
    Feathered,
    Furred,
    Chitinous,
    Warty,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Integument {
    pub base_color: LinearRgba,
    pub pattern: SurfacePattern,
    pub roughness: Roughness,
    pub metallic: Metallic,
}
