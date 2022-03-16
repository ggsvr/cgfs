use cgfs::{RayTracer, Canvas};
use minifb::{Window, WindowOptions};
use std::time;


#[derive(Debug)]
pub struct Rays {
    raytracer: RayTracer,
    buffer: Vec<u32>,
    window: Window,
}

impl Rays {
    pub fn new(mut rt: RayTracer) -> Self {
        let window_opts = WindowOptions {
            resize: true,
            scale: window_scale(800, (rt.canvas.width(), rt.canvas.height())),
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..Default::default()
        };

        let mut window = Window::new(
            "Rays",
            rt.canvas.width() as usize,
            rt.canvas.height() as usize,
            window_opts
        ).unwrap();

        window.limit_update_rate(Some(std::time::Duration::from_millis(1000 / 60)));

        let mut buffer = Vec::new();

        rt.render();
        update_buffer(&mut buffer, &rt.canvas);

        Self {
            raytracer: rt,
            window,
            buffer
        }
    }

    pub fn start<S>(&mut self, setup: impl FnOnce(&mut RayTracer) -> S, mut update: impl FnMut(Ctx, &mut S, time::Duration) -> Msg) {
        let mut state = setup(&mut self.raytracer);

        let mut last_time = time::Instant::now();
        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape) {

            let time = time::Instant::now();
            let delta = time - last_time;
            last_time = time;

            let ctx = Ctx {
                rt: &mut self.raytracer,
                keys: self.window.get_keys()
            };

            match update(ctx, &mut state, delta) {
                Msg::Nil => (),
                Msg::ShouldClose => break
            };
            
            self.raytracer.render();
            update_buffer(&mut self.buffer, &self.raytracer.canvas);

            self.window
                .update_with_buffer(&self.buffer, self.raytracer.canvas.width() as usize, self.raytracer.canvas.height() as usize)
                .unwrap();
        }
    }
}

pub enum Msg {
    Nil,
    ShouldClose
}

pub struct Ctx<'a> {
    pub keys: Vec<minifb::Key>,
    pub rt: &'a mut RayTracer,
}


fn window_scale(reference: u32, size: (u32, u32)) -> minifb::Scale {
    let scale = reference / size.0.max(size.1);

    if scale <= 1 {
        minifb::Scale::X1
    } else if scale <= 2 {
        minifb::Scale::X2
    } else if scale <= 4 {
        minifb::Scale::X4
    } else if scale <= 8 {
        minifb::Scale::X8
    } else if scale <= 16 {
        minifb::Scale::X16
    } else {
        minifb::Scale::X32
    }
}

fn update_buffer(buffer: &mut Vec<u32>, canvas: &Canvas) {
    *buffer = canvas
        .image
        .pixels()
        .map(|p| (p.0[0] as u32) << 16 | (p.0[1] as u32) << 8 | p.0[2] as u32)
        .collect();
}

