use crate::App;
use yew::prelude::*;
use web_sys::HtmlTextAreaElement;

#[derive(PartialEq, Properties)]
pub struct DescProps {
    pub rows: u32,
    pub cols: u32,
    pub default_scene: String,
    pub oncompile: Callback<String>,
}


pub struct Description {
    textarea: NodeRef,
}

impl Component for Description {
    type Message = ();
    type Properties = DescProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            textarea: NodeRef::default(),
        }
    }

    //fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    //    let textarea: HtmlTextAreaElement = self.textarea.cast().unwrap();
    //    false
    //}

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let oncompile = props.oncompile.clone();
        let textarea = self.textarea.clone();

        let onclick = Callback::from(move |_| {
            let textarea: HtmlTextAreaElement = textarea.cast().unwrap();
            oncompile.emit(textarea.value());
        });

        html! {
            <div>
                <textarea ref={self.textarea.clone()} rows={props.rows.to_string()} cols={props.cols.to_string()} value={props.default_scene.clone()}/>
                <br/><br/>
                <input type="button" value="Compile" {onclick}/>
            </div>
        }
    }

}