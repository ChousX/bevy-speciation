use bevy::math::Dir3;

use crate::primitives::{Count, LocalPosition};

#[derive(Clone, Debug)]
pub struct Socket<T> {
    pub position: LocalPosition,
    pub normal: Dir3,
    pub attachment: Option<T>,
}

#[derive(Clone, Debug)]
pub struct BilateralPair<T> {
    pub left: T,
    pub right: T,
}

#[derive(Clone, Debug)]
pub enum SymmetricSocket<T> {
    Medial(Socket<T>),
    Lateral(BilateralPair<Socket<T>>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BodySymmetry {
    #[default]
    Bilateral,
    Radial {
        fold_count: Count,
    },
    Asymmetric,
}
