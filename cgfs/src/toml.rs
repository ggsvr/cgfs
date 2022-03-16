use toml::Value;
use toml::value::Table;

use crate::{
    Camera, Viewport, Canvas,
    RayTracer, Point, Vector,
    Color,
    Sphere, Light,
    color, point,
};

fn get_table<'a>(toml: &'a Value) -> Result<&'a Table, String> {
    match toml.as_table() {
        Some(t) => Ok(t),
        None => Err("expected `table`".into())
    }
}

fn table_get<T: FromToml>(table: &Table, key: &str) -> Result<T, String> {
    match table.get(key) {
        None => Err(format!("missing field `{key}`")),
        Some(v) => FromToml::from_toml(v).map_err(|e| format!("{e} at `{key}`"))
    }
}

fn table_get_default<T: FromToml>(table: &Table, key: &str, default: T) -> Result<T, String> {
    match table.get(key) {
        None => Ok(default),
        Some(v) => FromToml::from_toml(v).map_err(|e| format!("{e} at `{key}`"))
    }
}

fn ret_obj<T, F: FnOnce() -> Result<T, String>>(err: &str, f: F) -> Result<T, String> {
    f().map_err(|e| format!("{err}:\n{e}"))
}


pub trait FromToml where Self: Sized {
    fn from_toml(toml: &Value) -> Result<Self, String>;
}

impl FromToml for RayTracer {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        let table = match toml {
            Value::Table(t) => t,
            _ => panic!("raytracer must be the root table")
        };
        let canvas: Canvas = table_get(table, "canvas")?;
        let camera: Camera = table_get(table, "camera")?;
        let viewport = table_get_default(table, "viewport", {
            let ratio = canvas.width() as f64 / canvas.height() as f64;
            let width = 1.;
            let height = width / ratio;
            Viewport {
                width, height,
                distance: 1.,
            }
        })?;
        let background = table_get_default(table, "background", color(0,0,0))?;
        let spheres = table_get_default(table, "spheres", Vec::new())?;
        let lights = table_get_default(table, "lights", Vec::new())?;

        let recursion_depth = table_get_default(table, "recursion", 3)?;
        
        Ok(RayTracer {
            canvas,
            camera,
            viewport,
            background,
            spheres,
            lights,
            recursion_depth
        })
    }
}

impl FromToml for Canvas {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        let err = "error in `canvas` definition";
        
        ret_obj(err, || {
            let table = get_table(toml)?;

            let width = table_get(table, "width")?;
            let height = table_get(table, "height")?;

            Ok(Canvas::new(width, height))
        })
    }
}

impl FromToml for Camera {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        let err = "error in `camera` definition";

        ret_obj(err, || {
            let table = get_table(toml)?;

            let position = table_get_default(table, "position", point(0., 0., 0.))?;
            let rotation = table_get_default(table, "rotation", [0., 0., 0.])?;
            Ok(Camera {
                position,
                rot_x: rotation[0],
                rot_y: rotation[1],
                rot_z: rotation[2],
            })
        })
    }
}

impl FromToml for Viewport {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        let err = "error in `viewport` definition";

        ret_obj(err, || {
            let table = get_table(toml)?;

            let width = table_get_default(table, "width", 1.)?;
            let height = table_get_default(table, "height", 1.)?;
            let distance = table_get_default(table, "distance", 1.)?;
            Ok(Viewport {
                width, height, distance
            })
        })
    }
}

impl FromToml for Sphere {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        let err = "error in `sphere` definition";

        ret_obj(err, || {
            let table = get_table(toml)?;

            let pos = table_get(table, "position")?;
            let radius = table_get(table, "radius")?;
            let color = table_get(table, "color")?;

            let specular = table_get_default(table, "specular", 0.)?;
            let reflective = table_get_default(table, "reflective", 0.)?;

            Ok(Sphere {
                pos, radius, color,
                specular, reflective
            })
        })
    }
}

impl FromToml for Light {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        let err = "error in `light` definition";

        ret_obj(err, || {
            let table = get_table(toml)?;

            let light_type: String = table_get(table, "type")?;
            let intensity = table_get(table, "intensity")?;

            match light_type.as_str() {
                "ambient" => Ok(Light::Ambient {
                    intensity
                }),
                "point" => {
                    let pos = table_get(table, "position")?;
                    Ok(Light::Point {
                        intensity,
                        pos
                    })
                },
                "directional" => {
                    let direction = table_get(table, "direction")?;
                    Ok(Light::Directional {
                        intensity,
                        direction
                    })
                }
                _ => Err("unknown type".into())
            }
        })
    }
}

impl FromToml for Point {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        <[f64; 3]>::from_toml(toml)
            .map(|a| Point::from(a))
            .map_err(|e| format!("error in `point` definition:\n{e}"))
    }
}
impl FromToml for Vector {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        <[f64; 3]>::from_toml(toml)
            .map(|a| Vector::from(a))
            .map_err(|e| format!("error in `vector` definition:\n{e}"))
    }
}
impl FromToml for Color {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        <[u8; 3]>::from_toml(toml)
            .map(|a| Color::from(a))
            .map_err(|e| format!("error in `color` definition:\n{e}"))
    }
}

impl<T: FromToml> FromToml for Vec<T> {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        let err = "error in array definition";
        match toml {
            Value::Array(v) => {
                let mut out = Vec::with_capacity(v.len());
                for (i, value) in v.iter().enumerate() {
                    let value: T = FromToml::from_toml(value)
                        .map_err(|e| format!("{err}:\n{e} at index `{i}`"))?;
                    out.push(value);
                }
                Ok(out)
            }
            _ => Err("expected array".into())
        }
    }
}

impl<T: FromToml, const N: usize> FromToml for [T; N] {

    fn from_toml(toml: &Value) -> Result<Self, String> {
        use std::mem::MaybeUninit;
        let err = "error in array definition";
        let sized_array_err = format!("expected `{N}` sized array");

        match toml {
            Value::Array(ar) => {
                if ar.len() != N {
                    return Err(sized_array_err);
                }
                let mut out: [T; N] = unsafe {
                    MaybeUninit::uninit().assume_init()
                };
                for (i, v) in ar.iter().enumerate() {
                    out[i] = T::from_toml(v).map_err(|e| format!("{err}:\n{e} at index `{i}`"))?;
                }
                Ok(out)
            },
            _ => Err(sized_array_err),
        }
    }
}

impl FromToml for String {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        match toml {
            Value::String(s) => Ok(s.clone()),
            _ => Err("expected string".into())
        }
    }
}

impl FromToml for f64 {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        match *toml {
            Value::Float(f) => Ok(f),
            Value::Integer(i) => Ok(i as f64),
            _ => Err("expected float".into())
        }
    }
}
impl FromToml for f32 {
    fn from_toml(toml: &Value) -> Result<Self, String> {
        match *toml {
            Value::Float(f) => Ok(f as f32),
            Value::Integer(i) => Ok(i as f32),
            _ => Err("expected float".into())
        }
    }
}


macro_rules! from_toml_int {
    ($t:ty) => {
        impl FromToml for $t {
            fn from_toml(toml: &Value) -> Result<Self, String> {
                let inttype = stringify!($t);
                match *toml {
                    Value::Integer(int) => int.try_into().map_err(|_| format!("integer out of {inttype} bounds")),
                    _ => Err(format!("expected {inttype}"))
                }
            }
        }
    };
    ($x:ty, $($y:ty),+) => {
        from_toml_int!($x);
        from_toml_int!($($y),+);
    }
}
from_toml_int!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn array() {
        let _: [i64; 5] = FromToml::from_toml(&toml::Value::Array(vec![toml::Value::Integer(0); 5])).unwrap();
        let _: [u32; 5] = FromToml::from_toml(&toml::Value::Array(vec![toml::Value::Integer(1); 5])).unwrap();
        <[u32; 3]>::from_toml(&toml::Value::Array(vec![toml::Value::Integer(2); 5])).unwrap_err();
        <[u32; 3]>::from_toml(&toml::Value::Array(vec![toml::Value::Integer(-1); 3])).unwrap_err();
    }
}