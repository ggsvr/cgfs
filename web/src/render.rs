
use cgfs::RayTracer;
use yew::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, ImageData};
use wasm_bindgen::{Clamped, JsCast};


#[derive(PartialEq, Properties)]
pub struct RenderProps {
    pub scene_desc: String,
    pub width: u32,
    pub height: u32,
}

pub enum RenderMsg {
    Render,
}

pub struct Render {
    img_data: Result<ImageData, String>,
    canvas: NodeRef,
}

fn render(scene_desc: &str, canvas_width: u32, canvas_height: u32) -> Result<ImageData, String> {
    let mut rt = RayTracer::from_description(scene_desc)?;
    let img = image::imageops::resize(
        rt.render(),
        canvas_width,
        canvas_height,
        image::imageops::FilterType::Nearest);

    let data: Vec<u8> = img
        .pixels()
        .map(|p| {
            [p.0[0], p.0[1], p.0[2], 255]
        })
        .flatten()
        .collect();

    let width = img.width();

    let image_data = ImageData::new_with_u8_clamped_array(Clamped(&data), width);

    image_data.map_err(|e| {
        let mut error = "Error creating ImageData".to_string();
        if let Some(s) = e.as_string() {
            error += &format!(": {s}");
        }
        error
    })
}

impl Component for Render {
    type Properties = RenderProps;
    type Message = RenderMsg; 


    fn create(ctx: &Context<Self>) -> Self {
        let toml = &ctx.props().scene_desc;
        let (width, height) = (ctx.props().width, ctx.props().height);
        Self {
            img_data: render(toml, width, height),
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        //if !first_render {return}
        let data = match &self.img_data {
            Ok(i) => i,
            Err(_) => return,
        };
        let canvas: HtmlCanvasElement = match self.canvas.cast() {
            Some(c) => c,
            None => return
        };

        canvas.set_width(data.width());
        canvas.set_height(data.height());

        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        ctx.put_image_data(data, 0., 0.).unwrap();
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Err(e) = &self.img_data {
            html! {
                <div>{e}</div>
            }
        } else {
            html! {
                <canvas class="render" ref={self.canvas.clone()}></canvas>
            }
        }
    }
}