use cgmath::prelude::*;
use cgmath::{Vector3, Matrix3};
use image::RgbImage;
use toml::Value;
use crate::{
    color::*,
    scene::*,
    canvas::*,
    camera::*,
    math::*,
};
#[cfg(feature = "scene")]
use crate::toml::FromToml;


#[derive(Debug, Clone)]
pub struct RayTracer {
    pub canvas: Canvas,
    pub camera: Camera,
    pub viewport: Viewport,
    pub background: Color,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub recursion_depth: u32,
}



impl RayTracer {
    #[cfg(feature = "scene")]
    pub fn from_description(toml: &str) -> Result<Self, String> {
        let toml: Value = toml::from_str(toml).map_err(|e| e.to_string())?;
        Self::from_toml(&toml)
    }
    pub fn render(&mut self) -> &RgbImage {
        for x in self.canvas.min_x()..self.canvas.max_x() {
            for y in self.canvas.min_y()..self.canvas.max_y() {
                let ray = self.canvas_to_viewport(x, y);
                let color = self.trace_ray(self.camera.position, ray, (self.viewport.distance, f64::INFINITY), self.recursion_depth);
                self.canvas.put_pixel(x, y, color);
            }
        }
        self.image()
    }

    pub fn image(&self) -> &RgbImage {
        &self.canvas.image
    }

    fn canvas_to_viewport(&self, x: i32, y: i32) -> Vector {
        let v = Vector3::new(
            x as f64 * (self.viewport.width / self.canvas.width() as f64),
            y as f64 * (self.viewport.height as f64 / self.canvas.height() as f64),
            self.viewport.distance
        );

        rotate_cam_ray(v, self.camera.rot_x, self.camera.rot_y, self.camera.rot_z)
    }

    fn trace_ray(&self, origin: Point, ray: Vector, t_bounds: (f64, f64), recursion_depth: u32) -> Color {
        let closest_intersection = self.closest_intersection(origin, ray, t_bounds);

        if let Some((sphere, t)) = closest_intersection {
            let p = origin + t * ray;
            let n = (p - sphere.pos).normalize();

            let local_color = sphere.color * self.compute_lighting(p, n, -ray, sphere.specular);

            if recursion_depth <= 0 || sphere.reflective <= 0. {
                return local_color;
            }

            let r = reflect_ray(-ray, n);
            let reflected_color = self.trace_ray(p, r, (0.001, f64::INFINITY), recursion_depth-1);

            local_color * (1.0 - sphere.reflective) + reflected_color * sphere.reflective
        } else {
            self.background
        }

    }
    fn closest_intersection(
        &self,
        origin: Point,
        ray: Vector,
        t_bounds: (f64, f64),
    ) -> Option<(&Sphere, f64)> {
        let mut closest_t = f64::INFINITY;
        let mut closest_sphere = None;

        for sphere in &self.spheres {
            let (t1, t2) = match sphere.intersect_ray(origin, ray) {
                Some(t) => t,
                None => continue,
            };

            if t1 > t_bounds.0 && t1 < t_bounds.1 && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }
            if t2 > t_bounds.0 && t2 < t_bounds.1 && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }
        closest_sphere.map(|s| (s, closest_t))
    }
    fn compute_lighting(
        &self,
        point: Point,
        normal: Vector,
        v: Vector,
        s: f64,
    ) -> f64 {
        let mut i = 0.;

        for light in &self.lights {
            let curr_i: f64;
            let l: Vector;
            let t_max: f64;

            match *light {
                Light::Ambient { intensity } => {
                    i += intensity;
                    continue;
                }
                Light::Directional {
                    intensity,
                    direction,
                } => {
                    curr_i = intensity;
                    l = direction * -1.;
                    t_max = f64::INFINITY;
                }
                Light::Point { intensity, pos } => {
                    curr_i = intensity;
                    l = pos - point;
                    t_max = 1.;
                }
            }

            if let Some(_) = self.closest_intersection(point, l, (0.001, t_max)) {
                continue;
            }

            let n_dot_l = normal.dot(l);
            if n_dot_l > 0. {
                i += curr_i * n_dot_l / (normal.magnitude() * l.magnitude());
            }

            if s > 0. {
                let r = reflect_ray(l, normal);
                let r_dot_v = r.dot(v);
                if r_dot_v > 0. {
                    i += curr_i * (r_dot_v / (r.magnitude() * v.magnitude())).powf(s);
                }
            }
        }
        i
    }

}



fn reflect_ray(ray: Vector, n: Vector) -> Vector {
    n * n.dot(ray) * 2. - ray
}


fn rotate_cam_ray(v: Vector, x: f64, y: f64, z: f64) -> Vector {
    let (x, y, z) = (x.to_radians(), y.to_radians(), z.to_radians());
    let x_matrix = |x: f64| Matrix3::new(
        1., 0., 0.,
        0., x.cos(), -x.sin(),
        0., x.sin(), x.cos());
    let y_matrix = |y: f64| Matrix3::new(
        y.cos(), 0., y.sin(),
        0., 1., 0.,
        -y.sin(), 0., y.cos());
    let z_matrix = |z: f64| Matrix3::new(
        z.cos(), -z.sin(), 0.,
        z.sin(), z.cos(), 0.,
        0., 0., 1.);

    let v0 = z_matrix(z) * v;
    let v1 = x_matrix(x) * v0;
    let v2 = y_matrix(y) * v1;

    v2
}