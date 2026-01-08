use crate::{
    appendage::{BranchPoint, LimbStructure, Terminus},
    body::Vertebra,
    head::Cranium,
    primitives::Length,
    skeletal::{Bone, BoneSegment, Joint, JointArticulation},
};

/// Types that contain bone geometry data
pub trait BoneSource {
    fn bone(&self) -> &Bone;

    fn length(&self) -> Length {
        self.bone().length
    }
}

impl BoneSource for Bone {
    fn bone(&self) -> &Bone {
        self
    }
}

impl BoneSource for Vertebra {
    fn bone(&self) -> &Bone {
        &self.bone
    }
}

impl BoneSource for BoneSegment {
    fn bone(&self) -> &Bone {
        &self.bone
    }
}

impl BoneSource for Cranium {
    fn bone(&self) -> &Bone {
        &self.bone
    }
}

/// Types with joint articulation constraints
pub trait Articulated {
    fn joint(&self) -> Option<&Joint>;

    fn articulation(&self) -> Option<&JointArticulation> {
        self.joint().map(|j| &j.articulation)
    }
}

impl Articulated for BoneSegment {
    fn joint(&self) -> Option<&Joint> {
        self.distal_joint.as_ref()
    }
}

// Vertebrae could have spinal articulation - for now return None
impl Articulated for Vertebra {
    fn joint(&self) -> Option<&Joint> {
        None
    }
}

/// A traversable chain of bone segments
pub trait BoneChain {
    fn segment_count(&self) -> usize;
    fn segment(&self, index: usize) -> Option<&BoneSegment>;
    fn branch_points(&self) -> &[BranchPoint];
    fn terminus(&self) -> Option<&Terminus>;

    fn segments(&self) -> BoneChainIter<'_, Self>
    where
        Self: Sized,
    {
        BoneChainIter {
            chain: self,
            index: 0,
        }
    }
}

pub struct BoneChainIter<'a, C: BoneChain> {
    chain: &'a C,
    index: usize,
}

impl<'a, C: BoneChain> Iterator for BoneChainIter<'a, C> {
    type Item = &'a BoneSegment;

    fn next(&mut self) -> Option<Self::Item> {
        let seg = self.chain.segment(self.index)?;
        self.index += 1;
        Some(seg)
    }
}

impl BoneChain for LimbStructure {
    fn segment_count(&self) -> usize {
        self.segments.len()
    }

    fn segment(&self, index: usize) -> Option<&BoneSegment> {
        self.segments.get(index)
    }

    fn branch_points(&self) -> &[BranchPoint] {
        match &self.branching {
            Some(bp) => std::slice::from_ref(bp),
            None => &[],
        }
    }

    fn terminus(&self) -> Option<&Terminus> {
        Some(&self.terminus)
    }
}

/// Types that terminate a limb and may spawn additional bones
pub trait Terminable {
    /// Number of additional leaf bones this terminus creates
    fn terminal_bone_count(&self) -> u8;

    /// Whether this terminus branches into multiple endpoints
    fn is_branching(&self) -> bool;
}

impl Terminable for Terminus {
    fn terminal_bone_count(&self) -> u8 {
        match self {
            Terminus::Tapered => 0,
            Terminus::Hoof => 0,
            Terminus::Sucker => 0,
            Terminus::Pincer => 2,
            Terminus::Claw { digits } => digits.value(),
            Terminus::Paw { digits } => digits.value(),
        }
    }

    fn is_branching(&self) -> bool {
        match self {
            Terminus::Tapered | Terminus::Hoof | Terminus::Sucker => false,
            Terminus::Pincer | Terminus::Claw { .. } | Terminus::Paw { .. } => true,
        }
    }
}
