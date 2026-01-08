use bevy::prelude::Vec3;

use crate::{
    appendage::{Appendage, AppendageClass, LimbStructure, Terminus},
    organism::Organism,
    primitives::Length,
    sockets_symmetry::SymmetricSocket,
};

use super::{
    BoneClass, BoneId, Side,
    node::{GeneratedSkeleton, RestTransform, SkeletonNode},
    traits::{Articulated, BoneChain, BoneSource, Terminable},
};

/// Configuration for skeleton generation
#[derive(Clone, Debug)]
pub struct SkeletonConfig {
    /// Primary axis along which bones extend (typically forward/back)
    pub bone_axis: Vec3,
    /// Axis for bilateral symmetry offsets (typically left/right)  
    pub lateral_axis: Vec3,
    /// Default lateral offset for bilateral appendages
    pub default_lateral_offset: f32,
}

impl Default for SkeletonConfig {
    fn default() -> Self {
        Self {
            bone_axis: Vec3::NEG_Z, // bones extend backward
            lateral_axis: Vec3::X,  // bilateral = left/right
            default_lateral_offset: 0.5,
        }
    }
}

/// Generates skeleton hierarchies from organisms
pub struct SkeletonGenerator {
    config: SkeletonConfig,
}

impl SkeletonGenerator {
    pub fn new(config: SkeletonConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(SkeletonConfig::default())
    }

    /// Generate a complete skeleton from an organism
    pub fn generate(&self, organism: &Organism) -> GeneratedSkeleton {
        // Create root node (synthetic, zero-length)
        let root_length = Length::new(0.001).unwrap(); // near-zero
        let mut root = SkeletonNode::new(BoneId::root(), root_length);

        // Add head
        let head_node = self.generate_head(organism);
        root.add_child(head_node);

        // Add spine and collect attachment points
        let spine_nodes = self.generate_spine(organism);
        for spine_node in spine_nodes {
            root.add_child(spine_node);
        }

        GeneratedSkeleton::new(root)
    }

    fn generate_head(&self, organism: &Organism) -> SkeletonNode {
        let cranium = organism.head();
        let head_length = cranium.length();

        let mut head_node = SkeletonNode::new(BoneId::head(), head_length).with_rest(
            RestTransform::from_offset_along_parent(
                0.0, // Head at origin, spine extends backward
                self.config.bone_axis,
            ),
        );

        // Add mandible if present
        if let Some(mandible_socket) = &cranium.mandible_socket {
            if let Some(mandible) = &mandible_socket.attachment {
                let mandible_node = self.generate_mandible(mandible);
                head_node.add_child(mandible_node);
            }
        }

        head_node
    }

    fn generate_mandible(
        &self,
        mandible: &crate::anatomical_features::MandibleStructure,
    ) -> SkeletonNode {
        // Mandible segments form a chain
        let first_segment = mandible.segments.first();

        let length = first_segment
            .map(|s| s.length())
            .unwrap_or_else(|| Length::new(0.1).unwrap());

        let mut current = SkeletonNode::new(BoneId::mandible(), length).with_rest(
            RestTransform::from_translation(
                Vec3::new(0.0, -0.2, 0.1), // Below and slightly forward
            ),
        );

        // Chain additional mandible segments
        for (i, segment) in mandible.segments.iter().skip(1).enumerate() {
            let seg_node = SkeletonNode::new(
                BoneId {
                    class: BoneClass::Mandible,
                    side: None,
                    index: (i + 1) as u8,
                    branch_path: Vec::new(),
                },
                segment.length(),
            )
            .with_rest(RestTransform::from_offset_along_parent(
                current.length.value(),
                Vec3::NEG_Y,
            ));

            if let Some(art) = segment.articulation() {
                current = current.with_articulation(art.clone());
            }
            current.add_child(seg_node);
            // Move reference to the new node for next iteration
            // (simplified - in practice we'd need to track the chain end)
        }

        current
    }

    fn generate_spine(&self, organism: &Organism) -> Vec<SkeletonNode> {
        let spine = &organism.torso().spine;
        let mut spine_nodes = Vec::new();
        let mut cumulative_offset = 0.0f32;

        for (i, vertebra) in spine.vertebrae.iter().enumerate() {
            let vert_length = vertebra.length();

            let mut vert_node = SkeletonNode::new(BoneId::spine(i as u8), vert_length).with_rest(
                RestTransform::from_offset_along_parent(cumulative_offset, self.config.bone_axis),
            );

            // Find appendages attached to this vertebra
            for attachment in &spine.appendages {
                if attachment.vertebra_index.0 as usize == i {
                    let appendage_nodes =
                        self.generate_appendage_from_socket(&attachment.socket, i as u8);
                    for app_node in appendage_nodes {
                        vert_node.add_child(app_node);
                    }
                }
            }

            cumulative_offset += vert_length.value();
            spine_nodes.push(vert_node);
        }

        spine_nodes
    }

    fn generate_appendage_from_socket(
        &self,
        socket: &SymmetricSocket<Appendage>,
        _vertebra_index: u8,
    ) -> Vec<SkeletonNode> {
        match socket {
            SymmetricSocket::Medial(s) => {
                if let Some(appendage) = &s.attachment {
                    vec![self.generate_appendage(appendage, None)]
                } else {
                    vec![]
                }
            }
            SymmetricSocket::Lateral(pair) => {
                let mut nodes = Vec::new();

                if let Some(left_app) = &pair.left.attachment {
                    nodes.push(self.generate_appendage(left_app, Some(Side::Left)));
                }
                if let Some(right_app) = &pair.right.attachment {
                    nodes.push(self.generate_appendage(right_app, Some(Side::Right)));
                }

                nodes
            }
        }
    }

    fn generate_appendage(&self, appendage: &Appendage, side: Option<Side>) -> SkeletonNode {
        let class = appendage.class;

        // Calculate lateral offset for bilateral appendages
        let lateral_offset = side
            .map(|s| s.mirror_x() * self.config.default_lateral_offset)
            .unwrap_or(0.0);

        // Generate the limb structure
        self.generate_limb(
            &appendage.structure,
            class,
            side,
            lateral_offset,
            Vec::new(),
        )
    }

    fn generate_limb(
        &self,
        limb: &LimbStructure,
        class: AppendageClass,
        side: Option<Side>,
        lateral_offset: f32,
        branch_path: Vec<u8>,
    ) -> SkeletonNode {
        // Start with first segment
        let first_seg = limb.segments.first();
        let first_length = first_seg
            .map(|s| s.length())
            .unwrap_or_else(|| Length::new(0.1).unwrap());

        let mut root_id = BoneId::limb(class, side, 0);
        root_id.branch_path = branch_path.clone();

        let initial_rest =
            RestTransform::from_translation(self.config.lateral_axis * lateral_offset);

        let mut root_node = SkeletonNode::new(root_id, first_length).with_rest(initial_rest);

        if let Some(seg) = first_seg {
            if let Some(art) = seg.articulation() {
                root_node = root_node.with_articulation(art.clone());
            }
        }

        // Chain remaining segments
        let mut parent_node = &mut root_node;
        let mut cumulative_length = first_length.value();

        for (i, segment) in limb.segments.iter().enumerate().skip(1) {
            let seg_length = segment.length();

            let mut seg_id = BoneId::limb(class, side, i as u8);
            seg_id.branch_path = branch_path.clone();

            let mut seg_node = SkeletonNode::new(seg_id, seg_length).with_rest(
                RestTransform::from_offset_along_parent(
                    cumulative_length,
                    self.limb_axis_for_class(class),
                ),
            );

            if let Some(art) = segment.articulation() {
                seg_node = seg_node.with_articulation(art.clone());
            }

            // Check for branching at this segment
            if let Some(branch_point) = &limb.branching {
                if branch_point.parent_segment == i {
                    let branch_nodes =
                        self.generate_branches(branch_point, class, side, &branch_path);
                    for bn in branch_nodes {
                        seg_node.add_child(bn);
                    }
                }
            }

            parent_node.add_child(seg_node);

            // Navigate to the child we just added
            let last_idx = parent_node.children.len() - 1;
            parent_node = &mut parent_node.children[last_idx];
            cumulative_length = seg_length.value();
        }

        // Add terminus bones
        let terminus_nodes = self.generate_terminus(&limb.terminus, side, &branch_path);
        for tn in terminus_nodes {
            parent_node.add_child(tn);
        }

        root_node
    }

    fn generate_branches(
        &self,
        branch_point: &crate::appendage::BranchPoint,
        class: AppendageClass,
        side: Option<Side>,
        parent_path: &[u8],
    ) -> Vec<SkeletonNode> {
        let mut nodes = Vec::new();

        for branch_idx in 0..branch_point.branch_count.value() {
            let mut new_path = parent_path.to_vec();
            new_path.push(branch_idx);

            // Calculate spread angle for this branch
            let spread = self.branch_spread_offset(branch_idx, branch_point.branch_count.value());

            let branch_node =
                self.generate_limb(&branch_point.branch, class, side, spread, new_path);
            nodes.push(branch_node);
        }

        nodes
    }

    fn generate_terminus(
        &self,
        terminus: &Terminus,
        side: Option<Side>,
        _branch_path: &[u8],
    ) -> Vec<SkeletonNode> {
        let bone_count = terminus.terminal_bone_count();

        if bone_count == 0 {
            return vec![];
        }

        let mut nodes = Vec::new();

        // TODO: Get actual digit length from somewhere (needs type extension)
        let digit_length = Length::new(0.1).unwrap();

        for i in 0..bone_count {
            let spread = self.branch_spread_offset(i, bone_count);

            let node = SkeletonNode::new(BoneId::digit(side, i), digit_length).with_rest(
                RestTransform::from_translation(self.config.lateral_axis * spread),
            );

            nodes.push(node);
        }

        nodes
    }

    /// Get the primary axis for limb extension based on appendage class
    fn limb_axis_for_class(&self, class: AppendageClass) -> Vec3 {
        match class {
            AppendageClass::Forelimb | AppendageClass::Hindlimb => Vec3::NEG_Y, // down
            AppendageClass::Wing => Vec3::X, // outward (will be mirrored)
            AppendageClass::Tail => self.config.bone_axis, // backward
            AppendageClass::Tentacle => Vec3::NEG_Y, // down/out
            AppendageClass::Antenna => Vec3::Y, // up
        }
    }

    /// Calculate lateral spread for branching structures
    fn branch_spread_offset(&self, index: u8, total: u8) -> f32 {
        if total <= 1 {
            return 0.0;
        }
        let normalized = index as f32 / (total - 1) as f32; // 0 to 1
        let centered = normalized - 0.5; // -0.5 to 0.5
        centered * 0.3 // scale factor
    }
}
