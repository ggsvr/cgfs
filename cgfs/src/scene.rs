use cgmath::{prelude::*, Point3, Vector3};
use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub pos: Point3<f64>,
    pub radius: f64,
    pub color: Color,
    pub specular: f64,

    pub reflective: f64,
}

impl Sphere {
    pub fn intersect_ray(&self, origin: Point3<f64>, ray: Vector3<f64>) -> Option<(f64, f64)> {
        let co = origin - self.pos;
        let a = ray.dot(ray);
        let b = co.dot(ray) * 2.;
        let c = co.dot(co) - self.radius * self.radius;

        let delta = b * b - 4. * a * c;

        if delta < 0. {
            return None;
        } else {
            let sqrt = delta.sqrt();
            let t1 = (-b + sqrt) / (2. * a);
            let t2 = (-b - sqrt) / (2. * a);

            Some((t1, t2))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Light {
    Ambient {
        intensity: f64,
    },
    Directional {
        intensity: f64,
        direction: Vector3<f64>,
    },
    Point {
        intensity: f64,
        pos: Point3<f64>,
    },
}

