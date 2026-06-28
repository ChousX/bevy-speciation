mod bone_id;
mod generation;
mod node;
mod traits;

pub use bone_id::{BoneClass, BoneId, Side};
pub use generation::{SkeletonConfig, SkeletonGenerator};
pub use node::{GeneratedSkeleton, RestTransform, SkeletonNode};
pub use traits::{Articulated, BoneChain, BoneSource, Terminable};
