use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

mod circle;
mod collisions;
mod geometry;
mod object;
mod polyhedron;
mod quad_tree;
mod rectangle;
mod world;

#[cfg(test)]
mod geometry_test;

use world::World;

enum Msg {
    Render(f64),
    Stop,
}

static DEBUG: bool = false;

struct App {
    world: World,
    last_tick: f64,
    sum_time: f64,
    ticks: u64,
    width: u64,
    height: u64,

    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let width = 1200;
        let height = 800;
        Self {
            world: World::new(width as f64, height as f64),
            node_ref: NodeRef::default(),
            _render_loop: None,
            last_tick: 0.0,
            sum_time: 0.0,
            ticks: 0,
            width,
            height,
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
                let delta_time = (time - self.last_tick) / 1000.0;

                self.sum_time += delta_time;
                self.ticks += 1;
                log::info!("{} fps", 1.0 / (self.sum_time / self.ticks as f64));

                self.world.tick(delta_time);
                self.last_tick = time;

                draw(&context, self.world.export());
                if DEBUG {
                    draw_quad_tree(
                        &context,
                        self.world.export_quad_tree(),
                        self.width as f64,
                        self.height as f64,
                    );
                }
                let handle = {
                    let link = ctx.link().clone();
                    request_animation_frame(move |time| link.send_message(Msg::Render(time)))
                };

                self._render_loop = Some(handle);
            }
            Msg::Stop => {
                self._render_loop = None;
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas ref={self.node_ref.clone()} width={format!("{}", self.width)} height={format!("{}", self.height)}/>
                <button onclick={ctx.link().callback(|_| Msg::Stop)}>
                    { "Stop" }
                </button>
            </div>
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

#[wasm_bindgen(module = "/src/draw.js")]
extern "C" {
    #[wasm_bindgen(js_name = "draw")]
    pub fn draw(context: &CanvasRenderingContext2d, objects: Box<[f64]>);

    #[wasm_bindgen(js_name = "draw_quad_tree")]
    pub fn draw_quad_tree(context: &CanvasRenderingContext2d, tree: Box<[f64]>, x1: f64, y1: f64);
}
