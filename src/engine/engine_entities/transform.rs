use crate::engine_math::quaternion::Quaternion;
use crate::engine_math::vector::Vec3;

#[derive(Clone, Default)]
pub struct Transform {
    position: Vec3,
    rotation: Quaternion,
    scale: Vec3,
}

impl Transform {
    pub(crate) fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}
