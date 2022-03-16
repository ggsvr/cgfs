use std::time::Duration;
use cgfs::*;
use minifb::{Window, WindowOptions, Key};
use rays::*;

const WINDOW_NAME: &str = "RayTracer";


fn main() {
    let toml = std::fs::read_to_string("../scene.toml").unwrap();
    let raytracer = RayTracer::from_description(&toml).unwrap();

    let mut rays = Rays::new(raytracer);

    rays.start(setup, update);

}

struct State {
}

fn setup(_: &mut RayTracer) -> State {
    State {}
}

fn update(ctx: Ctx, state: &mut State, delta: Duration) -> Msg {
    let delta = delta.as_secs_f64();
    for key in ctx.keys {
        match key {
            Key::Left => ctx.rt.camera.rot_y += 30. * delta,
            Key::Right => ctx.rt.camera.rot_y -= 30. * delta,
            Key::Up => ctx.rt.camera.rot_x += 30. * delta,
            Key::Down => ctx.rt.camera.rot_x -= 30. * delta,

            _ => (),
        }
    }
    Msg::Nil
}

//fn get_rotation(mag: u32, frames: u32, frame: u32) -> f64 {
//    let mag = mag as f64;
//    let frame = frame as f64;
//    let frames = frames as f64;
//    let arc = 360. / frames;
//    
//    (arc.to_radians() * frame).sin() * mag
//}
