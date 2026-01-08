use crate::appendage::AppendageClass;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn mirror_x(&self) -> f32 {
        match self {
            Side::Left => 1.0,
            Side::Right => -1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoneClass {
    Root,
    Head,
    Mandible,
    Spine,
    Limb(AppendageClass),
    Digit,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoneId {
    pub class: BoneClass,
    pub side: Option<Side>,
    pub index: u8,
    /// Path through branch points for nested structures (e.g., fingers)
    pub branch_path: Vec<u8>,
}

impl BoneId {
    pub fn root() -> Self {
        Self {
            class: BoneClass::Root,
            side: None,
            index: 0,
            branch_path: Vec::new(),
        }
    }

    pub fn head() -> Self {
        Self {
            class: BoneClass::Head,
            side: None,
            index: 0,
            branch_path: Vec::new(),
        }
    }

    pub fn mandible() -> Self {
        Self {
            class: BoneClass::Mandible,
            side: None,
            index: 0,
            branch_path: Vec::new(),
        }
    }

    pub fn spine(index: u8) -> Self {
        Self {
            class: BoneClass::Spine,
            side: None,
            index,
            branch_path: Vec::new(),
        }
    }

    pub fn limb(class: AppendageClass, side: Option<Side>, index: u8) -> Self {
        Self {
            class: BoneClass::Limb(class),
            side,
            index,
            branch_path: Vec::new(),
        }
    }

    pub fn with_branch(mut self, branch_index: u8) -> Self {
        self.branch_path.push(branch_index);
        self
    }

    pub fn digit(side: Option<Side>, digit_index: u8) -> Self {
        Self {
            class: BoneClass::Digit,
            side,
            index: digit_index,
            branch_path: Vec::new(),
        }
    }

    /// Generate a deterministic name for animation targeting
    pub fn name(&self) -> String {
        let mut name = match self.class {
            BoneClass::Root => return "root".to_string(),
            BoneClass::Head => "head".to_string(),
            BoneClass::Mandible => "mandible".to_string(),
            BoneClass::Spine => format!("spine_{}", self.index),
            BoneClass::Limb(class) => {
                let class_name = match class {
                    AppendageClass::Forelimb => "forelimb",
                    AppendageClass::Hindlimb => "hindlimb",
                    AppendageClass::Wing => "wing",
                    AppendageClass::Tentacle => "tentacle",
                    AppendageClass::Tail => "tail",
                    AppendageClass::Antenna => "antenna",
                };
                format!("{}_{}", class_name, self.index)
            }
            BoneClass::Digit => format!("digit_{}", self.index),
        };

        if let Some(side) = self.side {
            let suffix = match side {
                Side::Left => "_L",
                Side::Right => "_R",
            };
            name.push_str(suffix);
        }

        for (i, branch) in self.branch_path.iter().enumerate() {
            name.push_str(&format!("_b{}_{}", i, branch));
        }

        name
    }
}
