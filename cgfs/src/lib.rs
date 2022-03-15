pub mod color;
pub mod canvas;
pub mod scene;
pub mod camera;
pub mod raytracer;
pub mod rasterizer;

#[cfg(feature = "scene")]
pub mod toml;



pub mod math {
    use cgmath::{Vector3, Point3};

    pub use cgmath::{point3 as point, vec3 as vector};
    pub type Point = Point3<f64>;
    pub type Vector = Vector3<f64>;
}

pub use color::{Color, color};
pub use canvas::Canvas;
pub use scene::{Light, Sphere};
pub use camera::{Camera, Viewport};
pub use raytracer::RayTracer;
pub use math::*;