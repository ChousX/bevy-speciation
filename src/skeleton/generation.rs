use bevy::prelude::{Quat, Vec3};

use crate::{
    anatomical_features::MandibleStructure,
    appendage::{Appendage, AppendageClass, BranchPoint, LimbStructure, Terminus},
    organism::Organism,
    primitives::Length,
    sockets_symmetry::{Socket, SymmetricSocket},
};

use super::{
    bone_id::{BoneClass, BoneId, Side},
    node::{GeneratedSkeleton, RestTransform, SkeletonNode},
    traits::{Articulated, BoneSource, Terminable},
};

/// Configuration for skeleton generation
#[derive(Clone, Debug)]
pub struct SkeletonConfig {
    /// Primary axis along which bones extend (typically forward/back)
    pub bone_axis: Vec3,
    /// Axis for bilateral symmetry offsets (typically left/right)
    pub lateral_axis: Vec3,
}

impl Default for SkeletonConfig {
    fn default() -> Self {
        Self {
            bone_axis: Vec3::NEG_Z,
            lateral_axis: Vec3::X,
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
        let root_length = Length::new(0.001).unwrap();
        let mut root = SkeletonNode::new(BoneId::root(), root_length);

        // Generate spine first - head attaches to it via neck
        let mut spine_nodes = self.generate_spine(organism);

        // Attach head to the appropriate vertebra via neck
        let head_node = self.generate_head(organism);
        let neck = &organism.head().neck;

        if let Some(vertebra_node) = spine_nodes.get_mut(neck.vertebra_index as usize) {
            vertebra_node.add_child(head_node);
        } else if !spine_nodes.is_empty() {
            // Fallback: attach to first vertebra
            spine_nodes[0].add_child(head_node);
        } else {
            // No spine, attach directly to root
            root.add_child(head_node);
        }

        for spine_node in spine_nodes {
            root.add_child(spine_node);
        }

        GeneratedSkeleton::new(root)
    }

    fn generate_head(&self, organism: &Organism) -> SkeletonNode {
        let cranium = organism.head();
        let head_length = BoneSource::length(cranium);
        let neck = &cranium.neck;

        // Head position comes from neck attachment offset
        let rest = RestTransform {
            translation: neck.offset.as_vec3(),
            rotation: Quat::IDENTITY,
        };

        let mut head_node = SkeletonNode::new(BoneId::head(), head_length)
            .with_rest(rest)
            .with_articulation(neck.joint.articulation.clone());

        if let Some(mandible_socket) = &cranium.mandible_socket {
            if let Some(mandible) = &mandible_socket.attachment {
                let mandible_node = self.generate_mandible(mandible, mandible_socket);
                head_node.add_child(mandible_node);
            }
        }

        head_node
    }

    fn generate_mandible(
        &self,
        mandible: &MandibleStructure,
        socket: &Socket<MandibleStructure>,
    ) -> SkeletonNode {
        let first_segment = mandible.segments.first();

        let length = first_segment
            .map(|s| BoneSource::length(s))
            .unwrap_or_else(|| Length::new(0.1).unwrap());

        // Use socket position for mandible placement
        let rest = RestTransform {
            translation: socket.position.as_vec3(),
            rotation: Quat::from_rotation_arc(Vec3::NEG_Z, socket.normal.as_vec3()),
        };

        let mut current = SkeletonNode::new(BoneId::mandible(), length).with_rest(rest);

        // Add articulation from first segment if present
        if let Some(seg) = first_segment {
            if let Some(art) = Articulated::articulation(seg) {
                current = current.with_articulation(art.clone());
            }
        }

        // Chain additional mandible segments
        let mut parent = &mut current;
        for (i, segment) in mandible.segments.iter().skip(1).enumerate() {
            let seg_length = BoneSource::length(segment);

            let mut seg_node = SkeletonNode::new(
                BoneId {
                    class: BoneClass::Mandible,
                    side: None,
                    index: (i + 1) as u8,
                    branch_path: Vec::new(),
                },
                seg_length,
            )
            .with_rest(RestTransform::from_offset_along_parent(
                parent.length.value(),
                Vec3::NEG_Y,
            ));

            if let Some(art) = Articulated::articulation(segment) {
                seg_node = seg_node.with_articulation(art.clone());
            }

            parent.add_child(seg_node);
            let last_idx = parent.children.len() - 1;
            parent = &mut parent.children[last_idx];
        }

        current
    }

    fn generate_spine(&self, organism: &Organism) -> Vec<SkeletonNode> {
        let spine = &organism.torso().spine;
        let mut spine_nodes = Vec::new();
        let mut cumulative_offset = 0.0f32;

        for (i, vertebra) in spine.vertebrae.iter().enumerate() {
            let vert_length = BoneSource::length(vertebra);

            let mut vert_node = SkeletonNode::new(BoneId::spine(i as u8), vert_length).with_rest(
                RestTransform::from_offset_along_parent(cumulative_offset, self.config.bone_axis),
            );

            for attachment in &spine.appendages {
                if attachment.vertebra_index.0 as usize == i {
                    let appendage_nodes = self.generate_appendage_from_socket(&attachment.socket);
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
    ) -> Vec<SkeletonNode> {
        match socket {
            SymmetricSocket::Medial(s) => {
                if let Some(appendage) = &s.attachment {
                    let rest = self.rest_from_socket(s, None);
                    vec![self.generate_appendage(appendage, None, rest)]
                } else {
                    vec![]
                }
            }
            SymmetricSocket::Lateral(pair) => {
                let mut nodes = Vec::new();
                if let Some(left_app) = &pair.left.attachment {
                    let rest = self.rest_from_socket(&pair.left, Some(Side::Left));
                    nodes.push(self.generate_appendage(left_app, Some(Side::Left), rest));
                }
                if let Some(right_app) = &pair.right.attachment {
                    let rest = self.rest_from_socket(&pair.right, Some(Side::Right));
                    nodes.push(self.generate_appendage(right_app, Some(Side::Right), rest));
                }
                nodes
            }
        }
    }

    /// Create rest transform from socket position and normal
    fn rest_from_socket<T>(&self, socket: &Socket<T>, side: Option<Side>) -> RestTransform {
        let mut pos = socket.position.as_vec3();

        // Mirror X for right side
        if side == Some(Side::Right) {
            pos.x = -pos.x;
        }

        // Compute rotation to align bone axis with socket normal
        let rotation = Quat::from_rotation_arc(Vec3::NEG_Y, socket.normal.as_vec3());

        RestTransform {
            translation: pos,
            rotation,
        }
    }

    fn generate_appendage(
        &self,
        appendage: &Appendage,
        side: Option<Side>,
        root_rest: RestTransform,
    ) -> SkeletonNode {
        self.generate_limb(
            &appendage.structure,
            appendage.class,
            side,
            root_rest,
            Vec::new(),
        )
    }

    fn generate_limb(
        &self,
        limb: &LimbStructure,
        class: AppendageClass,
        side: Option<Side>,
        root_rest: RestTransform,
        branch_path: Vec<u8>,
    ) -> SkeletonNode {
        let first_seg = limb.segments.first();
        let first_length = first_seg
            .map(|s| BoneSource::length(s))
            .unwrap_or_else(|| Length::new(0.1).unwrap());

        let mut root_id = BoneId::limb(class, side, 0);
        root_id.branch_path = branch_path.clone();

        let mut root_node = SkeletonNode::new(root_id, first_length).with_rest(root_rest);

        if let Some(seg) = first_seg {
            if let Some(art) = Articulated::articulation(seg) {
                root_node = root_node.with_articulation(art.clone());
            }
        }

        let mut parent_node = &mut root_node;
        let limb_axis = self.limb_axis_for_class(class, side);

        for (i, segment) in limb.segments.iter().enumerate().skip(1) {
            let seg_length = BoneSource::length(segment);
            let parent_len = parent_node.length.value();

            let mut seg_id = BoneId::limb(class, side, i as u8);
            seg_id.branch_path = branch_path.clone();

            let mut seg_node = SkeletonNode::new(seg_id, seg_length).with_rest(
                RestTransform::from_offset_along_parent(parent_len, limb_axis),
            );

            if let Some(art) = Articulated::articulation(segment) {
                seg_node = seg_node.with_articulation(art.clone());
            }

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
            let last_idx = parent_node.children.len() - 1;
            parent_node = &mut parent_node.children[last_idx];
        }

        let terminus_nodes = self.generate_terminus(
            &limb.terminus,
            class,
            side,
            &branch_path,
            parent_node.length.value(),
        );
        for tn in terminus_nodes {
            parent_node.add_child(tn);
        }

        root_node
    }

    fn generate_branches(
        &self,
        branch_point: &BranchPoint,
        class: AppendageClass,
        side: Option<Side>,
        parent_path: &[u8],
    ) -> Vec<SkeletonNode> {
        let mut nodes = Vec::new();
        let count = branch_point.branch_count.value();

        for branch_idx in 0..count {
            let mut new_path = parent_path.to_vec();
            new_path.push(branch_idx);

            let spread = self.branch_spread_offset(branch_idx, count);
            let rest = RestTransform::from_translation(self.config.lateral_axis * spread);

            let branch_node = self.generate_limb(&branch_point.branch, class, side, rest, new_path);
            nodes.push(branch_node);
        }

        nodes
    }

    fn generate_terminus(
        &self,
        terminus: &Terminus,
        class: AppendageClass,
        side: Option<Side>,
        _branch_path: &[u8],
        parent_length: f32,
    ) -> Vec<SkeletonNode> {
        let bone_count = Terminable::terminal_bone_count(terminus);

        if bone_count == 0 {
            return vec![];
        }

        // Use actual digit length from terminus
        let digit_length =
            Terminable::terminal_bone_length(terminus).unwrap_or_else(|| Length::new(0.1).unwrap());

        let mut nodes = Vec::new();
        let limb_axis = self.limb_axis_for_class(class, side);

        for i in 0..bone_count {
            let spread = self.branch_spread_offset(i, bone_count);

            let rest = RestTransform {
                translation: limb_axis * parent_length + self.config.lateral_axis * spread,
                rotation: Quat::IDENTITY,
            };

            let node = SkeletonNode::new(BoneId::digit(side, i), digit_length).with_rest(rest);
            nodes.push(node);
        }

        nodes
    }

    /// Get the primary axis for limb extension based on appendage class
    fn limb_axis_for_class(&self, class: AppendageClass, side: Option<Side>) -> Vec3 {
        match class {
            AppendageClass::Forelimb | AppendageClass::Hindlimb => Vec3::NEG_Y,
            AppendageClass::Wing => {
                // Wings extend outward, mirrored by side
                let dir = if side == Some(Side::Right) { -1.0 } else { 1.0 };
                Vec3::X * dir
            }
            AppendageClass::Tail => self.config.bone_axis,
            AppendageClass::Tentacle => Vec3::NEG_Y,
            AppendageClass::Antenna => Vec3::Y,
        }
    }

    /// Calculate lateral spread for branching structures
    fn branch_spread_offset(&self, index: u8, total: u8) -> f32 {
        if total <= 1 {
            return 0.0;
        }
        let normalized = index as f32 / (total - 1) as f32;
        let centered = normalized - 0.5;
        centered * 0.3
    }
}
