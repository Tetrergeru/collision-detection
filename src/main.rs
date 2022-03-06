use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

enum Msg {
    Render(f64),
}

struct App {
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            _render_loop: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(time) => {
                let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                context.set_fill_style(&("#f00".to_string()).into());
                context.begin_path();
                context.move_to(150.0 + time / 200.0, 100.0);
                context.line_to(100.0 + time / 200.0, 150.0);
                context.line_to(100.0 + time / 200.0, 100.0);
                context.close_path();
                context.fill();

                let handle = {
                    let link = ctx.link().clone();
                    request_animation_frame(move |time| link.send_message(Msg::Render(time)))
                };

                self._render_loop = Some(handle);
            }
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.node_ref.clone()} width=300 height=300/>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };

            self._render_loop = Some(handle);
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
