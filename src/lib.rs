pub mod implicit;
pub mod primitives;
pub mod csg;
pub mod transform;
pub mod dsl;
pub mod evaluator;
pub mod renderer;

pub use glam::{Vec2, Vec3, Mat3, Mat4};

pub mod prelude {
    pub use crate::implicit::*;
    pub use crate::primitives::*;
    pub use crate::csg::*;
    pub use crate::transform::*;
    pub use crate::dsl::*;
    pub use crate::evaluator::*;
    pub use glam::{Vec2, Vec3, Mat3, Mat4};
}
