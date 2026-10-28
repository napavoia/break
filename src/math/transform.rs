use crate::math::Vector3;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Transform {
    position: Vector3<f64>,
    size: Vector3<f64>,
}
