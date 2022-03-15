use crate::math::*;
use cgmath::prelude::*;
use cgmath::Quaternion;

const ORIGIN_VEC: Vector = vector(0., 0., 1.);

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Point,
    pub rot_x: f64,
    pub rot_y: f64,
    pub rot_z: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point::new(0., 0., 0.),
            rot_x: 0., rot_y: 0., rot_z: 0.
        }
    }
}

#[derive(Debug, Clone)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub distance: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 1.,
            height: 1.,
            distance: 1.,
        }
    }
}
