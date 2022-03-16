pub mod render;
pub mod desc;

use render::Render;
use desc::Description;

use yew::prelude::*;

const DEFAULT_SCENE: &str = include_str!("../scene.toml");


pub struct App {
    scene: String,
}

impl Component for App {
    type Message = String;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            scene: DEFAULT_SCENE.to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.scene = msg;
        web_sys::console::log_1(&format!("new scene: {}", self.scene).into());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oncompile = ctx.link().callback(|s| {
            //web_sys::console::log_1(&"COIMPIEL".into());
            s
        });
        web_sys::console::log_1(&"RERENDERED".into());
        html! {
            <>
               <div class="mainbox">
                    <Description rows=40 cols=30 default_scene={self.scene.clone()} oncompile={oncompile}/>
                    <Render width=800 height=800 scene_desc={self.scene.clone()} />
               </div> 
            </>
        }
    }
}