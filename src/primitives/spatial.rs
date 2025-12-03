use bevy::prelude::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalPosition(pub Vec3);

impl LocalPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
    pub fn as_vec3(self) -> Vec3 {
        self.0
    }
}

impl From<Vec3> for LocalPosition {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}
