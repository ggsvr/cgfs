use cgfs::RayTracer;
use winit::{
    window::{Window, WindowBuilder},
};

pub struct Rays<State, Setup, Update>
    where
        Setup: FnOnce(&mut RayTracer) -> State,
        Update: FnMut(Ctx, &mut State, std::time::Duration) -> Msg
{
    rt: RayTracer,
    setup: Option<Setup>,
    update: Option<Update>,
}

impl<State, Setup, Update> Rays<State, Setup, Update>
    where
        Setup: FnOnce(&mut RayTracer) -> State,
        Update: FnMut(Ctx, &mut State, std::time::Duration) -> Msg
{
    pub fn new(rt: RayTracer) -> Self {
        Self {
            rt,
            setup: None,
            update: None,
        }
    }

    pub fn setup(mut self, setup: Setup) -> Self {
        self.setup = Some(setup);
        self
    }
    pub fn update(mut self, update: Update) -> Self {
        self.update = Some(update);
        self
    }

    pub fn start(self) {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(self.rt.image().width(), self.rt.image().height()))
            .with_resizable(true)
            .with_title("Rays")
            .build(&event_loop)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {

        });
    }

}

#[derive(Debug)]
pub struct Ctx<'a> {
    rt: &'a mut RayTracer,
}


pub enum Msg {
    Nil,
    Close
}