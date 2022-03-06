use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

mod world;
mod object;
mod rectangle;
mod geometry;

use world::World;

enum Msg {
    Render(f64),
}

struct App {
    world: World,
    last_tick: f64,

    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            world: World::new(),
            node_ref: NodeRef::default(),
            _render_loop: None,
            last_tick: 0.0,
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

                context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                self.world.tick((time - self.last_tick) / 1000.0);
                self.last_tick = time;
                self.world.draw(&context);

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
            <canvas ref={self.node_ref.clone()} width=1000 height=1000/>
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
