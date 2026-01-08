use bevy::prelude::{Quat, Vec3};

use crate::{primitives::Length, skeletal::JointArticulation};

use super::BoneId;

/// Rest pose transform relative to parent bone
#[derive(Clone, Copy, Debug)]
pub struct RestTransform {
    pub translation: Vec3,
    pub rotation: Quat,
}

impl Default for RestTransform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }
}

impl RestTransform {
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
        }
    }

    pub fn from_offset_along_parent(length: f32, axis: Vec3) -> Self {
        Self {
            translation: axis * length,
            rotation: Quat::IDENTITY,
        }
    }

    /// Create transform with a lateral offset (for bilateral symmetry)
    pub fn with_lateral_offset(mut self, offset: f32) -> Self {
        self.translation.x += offset;
        self
    }

    /// Mirror across the YZ plane (negate X)
    pub fn mirrored_x(mut self) -> Self {
        self.translation.x = -self.translation.x;
        self
    }
}

/// A node in the skeleton hierarchy
#[derive(Clone, Debug)]
pub struct SkeletonNode {
    pub id: BoneId,
    pub rest: RestTransform,
    pub length: Length,
    pub articulation: Option<JointArticulation>,
    pub children: Vec<SkeletonNode>,
}

impl SkeletonNode {
    pub fn new(id: BoneId, length: Length) -> Self {
        Self {
            id,
            rest: RestTransform::default(),
            length,
            articulation: None,
            children: Vec::new(),
        }
    }

    pub fn with_rest(mut self, rest: RestTransform) -> Self {
        self.rest = rest;
        self
    }

    pub fn with_articulation(mut self, articulation: JointArticulation) -> Self {
        self.articulation = Some(articulation);
        self
    }

    pub fn with_child(mut self, child: SkeletonNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn add_child(&mut self, child: SkeletonNode) {
        self.children.push(child);
    }

    /// Recursively count all bones in this subtree (including self)
    pub fn bone_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.bone_count()).sum::<usize>()
    }

    /// Iterate depth-first over all nodes
    pub fn iter_depth_first(&self) -> DepthFirstIter<'_> {
        DepthFirstIter { stack: vec![self] }
    }

    /// Find a node by its ID
    pub fn find(&self, id: &BoneId) -> Option<&SkeletonNode> {
        if &self.id == id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find(id) {
                return Some(found);
            }
        }
        None
    }

    /// Find a node by its ID (mutable)
    pub fn find_mut(&mut self, id: &BoneId) -> Option<&mut SkeletonNode> {
        if &self.id == id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(found) = child.find_mut(id) {
                return Some(found);
            }
        }
        None
    }
}

pub struct DepthFirstIter<'a> {
    stack: Vec<&'a SkeletonNode>,
}

impl<'a> Iterator for DepthFirstIter<'a> {
    type Item = &'a SkeletonNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        // Push children in reverse so we visit left-to-right
        self.stack.extend(node.children.iter().rev());
        Some(node)
    }
}

/// The complete generated skeleton
#[derive(Clone, Debug)]
pub struct GeneratedSkeleton {
    pub root: SkeletonNode,
}

impl GeneratedSkeleton {
    pub fn new(root: SkeletonNode) -> Self {
        Self { root }
    }

    pub fn bone_count(&self) -> usize {
        self.root.bone_count()
    }

    pub fn find(&self, id: &BoneId) -> Option<&SkeletonNode> {
        self.root.find(id)
    }

    pub fn iter(&self) -> DepthFirstIter<'_> {
        self.root.iter_depth_first()
    }
}
