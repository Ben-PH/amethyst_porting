pub mod ecs;
use crate::ametheed::ui::colored_box::UiColorBox;
use crate::ametheed::Interactable;
use crate::ametheed::UiText;
use crate::ametheed::ui::transform::UiTransform;
use crate::ametheed::ui::button::UiButtonBuilder;
use crate::ametheed::ui::button::builder::UiButtonBuilderResources;
use crate::ametheed::UiButton;
use crate::ametheed::ui::layout::Anchor;
use crate::pages::cg_graph::ecs::components::Position;
use crate::pages::cg_graph::ecs::components::Renderable;
use crate::pages::cg_graph::ecs::components::Color;
use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use seed::{prelude::*, *};
use shared::learning_trajectory;
use specs::prelude::*;
use std::collections::HashMap;
use web_sys::{HtmlCanvasElement};

const WIDTH: usize = 900;
const HEIGHT: usize = 600;
const RAD: u32 = 50;

#[derive(Debug)]
pub struct Model {
    pub pet: DiGraph<UiButton, f32>,
    fill_color: Color,
    canvas: ElRef<HtmlCanvasElement>,
    pub specs: ecs::State,
    tics: usize,
}

impl Model {
    fn render(&mut self) {
        let xform = self.specs.inner.read_storage::<UiTransform>();
        let text = self.specs.inner.read_storage::<UiText>();
        let canvas = self.canvas.get().expect("get canvas element");
        let ctx = seed::canvas_context_2d(&canvas);
        for (xform, text) in (&xform, &text).join() {
            ctx.begin_path();
            let x = xform.local_x;
            let y = xform.local_y;
            ctx.set_fill_style(&JsValue::from_str(&xform.));
            ctx.arc(
                x as f64,
                y as f64,
                RAD.into(),
                0.0,
                std::f64::consts::PI * 2.,
            );
            ctx.fill();
        }
    }

    fn detect_hover(&mut self, mouse_pos: (f32, f32)) {
        let positions = self.specs.inner.read_storage::<Position>();
        let rends = self.specs.inner.read_storage::<Renderable>();
        let mut cols = self.specs.inner.write_storage::<ecs::components::Color>();
        for (_rend, pos, mut col) in (&rends, &positions, &mut cols).join() {
            if (mouse_pos.0 - pos.x) * (mouse_pos.0 - pos.x) + (mouse_pos.1 - pos.y) * (mouse_pos.1 - pos.y) < (RAD * RAD) as f32
            {
                col.b = 0;
            }
        }
        // log!(mouse_pos);
    }
}

impl Default for Model {
    fn default() -> Self {
        let mut specs = ecs::State::init();
        specs
            .inner
            .create_entity()
            .with(Renderable)
            .with(ecs::components::Position { x: 50.0, y: 50.0 })
            .build();
        Self {
            tics: 0,
            pet: Default::default(),
            fill_color: Color { r: 0, g: 255, b: 0 },
            canvas: Default::default(),
            specs,
        }
    }
}

// #[derive(Debug, Default)]
// struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }

// impl Color {
//     fn html_str(&self) -> String {
//         format!("#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
//     }
// }

struct Myf32(f32);

impl Default for Myf32 {
    fn default() -> Self {
        Myf32(0.0)
    }
}

#[derive(Debug)]
pub enum Message {
    FetchCGGraph,
    CGGraph(fetch::Result<learning_trajectory::CGGraph>),
    OnTick(RenderInfo),
    CanvasMouse(web_sys::MouseEvent),
    DotFile,
    Rendered,
    ChangeColor,
}

#[derive(Debug)]
pub struct CGNode {
    color: Color,
    pos_x: f64,
    pos_y: f64,
    cg: shared::learning_trajectory::ConsensusGoal,
}
// fn draw(
//     model: &Model
//     // model: &petgraph::graph::DiGraph<CGNode, f32>,
//     // canvas: &ElRef<HtmlCanvasElement>,
//     // fill_color: &Color,
// ) {
//     let canvas = model.canvas.get().expect("get canvas element");
//     let ctx = seed::canvas_context_2d(&canvas);

//     ctx.rect(0., 0., (WIDTH as u32).into(), (HEIGHT as u32).into());
//     ctx.set_fill_style(&JsValue::from_str(&model.fill_color.html_str()));
//     ctx.fill();

//     let row_count: u32 = (WIDTH as u32) / (RAD*2);
//     for (i, node) in model.pet.node_references().enumerate() {
//         ctx.begin_path();
//         let x: f64 = (node.1).pos_x;
//         let y: f64 = (node.1).pos_y;
//         ctx.set_fill_style(&JsValue::from_str(&node.1.color.html_str()));
//         ctx.arc(x, y, RAD.into(), 0.0, std::f64::consts::PI * 2.);
//         ctx.fill();
//     }
// }
pub fn update(msg: Message, mdl: &mut Model, orders: &mut impl Orders<Message>) {
    use Message::*;
    match msg {
        Message::OnTick(rend_inf) => {
            mdl.tics += 1;
            mdl.render();
            orders.after_next_render(Message::OnTick);
        }
        Message::ChangeColor => std::mem::swap(&mut mdl.fill_color.b, &mut mdl.fill_color.g),
        // Message::Rendered => {
        //     draw(&mdl);
        //     // We want to call `.skip` to prevent infinite loop.
        //     // (However infinite loops are useful for animations.)
        //     orders.after_next_render(|_| Message::Rendered).skip();
        // }
        FetchCGGraph => {
            orders.perform_cmd(async { CGGraph(fetch_cg_graph().await) });
        }
        CGGraph(Ok(res)) => {
            let mut gr = DiGraph::<CGNode, f32>::new();
            let mut idx_map: HashMap<usize, NodeIndex> = HashMap::with_capacity(res.0.len());
            let row_count: u32 = (WIDTH as u32) / (RAD * 2);
            for (i, node) in res.0.into_iter().enumerate() {
                let x = (RAD + (i as u32 % (row_count as u32)) * (RAD * 2)) as f32;
                let y = (RAD + (i as u32 / (row_count as u32)) * (RAD * 2)) as f32;
                let xform = UiTransform::new(
                    i.to_string(),
                    Anchor::Middle,
                    Anchor::Middle,
                    x,
                    y,
                    0.,
                    (RAD * 2) as f32,
                    (RAD * 2) as f32,
                );
                let r =  255 / (i as u8 + 1);
                let g =  255 - (255 / (i as u8 + 1));
                let b =  255;
                let col = crate::ametheed::ui::colored_box::UiColorBox::SolidColor([r.into(),g.into(),b as f32,255.]);
                let text = UiText::new(
                    i.to_string(),
                    [(255 - r).into(), (255 - g).into(), (255 - b) as f32, 255.],
                    RAD as f32 / 2.,
                    crate::ametheed::ui::text::LineMode::Single,
                    Anchor::Middle

                );
                mdl.specs.inner.create_entity()
                               .with(xform)
                               .with(text)
                               .with(Interactable)
                    .build();
                // let idx = gr.add_node(g_node);
                // idx_map.insert(gr.raw_nodes()[idx.index()].weight.cg.id, idx);
            }
            for edge in res.1.into_iter() {
                gr.add_edge(
                    *idx_map.get(&edge.left).unwrap(),
                    *idx_map.get(&edge.right).unwrap(),
                    edge.weight,
                );
            }
            // mdl.pet = gr;
            // for node in mdl.pet.raw_nodes() {
            //     let node = &node.weight;
            //     mdl.specs
            //         .inner
            //         .create_entity()
            //         .with(Position {
            //             x: node.pos_x as f32,
            //             y: node.pos_y as f32,
            //         })
            //         .with(ecs::components::Color {
            //             r: node.color.r,
            //             g: node.color.g,
            //             b: node.color.b,
            //         })
            //         .with(Renderable)
            //         .build();
            // }
            log!(mdl.pet.raw_nodes());
            orders.after_next_render(Message::OnTick);
        }
        DotFile => log!(Dot::with_config(&mdl.pet, &[Config::EdgeNoLabel])),
        CanvasMouse(ev) => {
            let ox = mdl.canvas.get().unwrap().offset_left()
                - web_sys::window().unwrap().page_x_offset().unwrap() as i32;
            let oy = mdl.canvas.get().unwrap().offset_top()
                - web_sys::window().unwrap().page_y_offset().unwrap() as i32;
            let canv_pos = (ev.client_x() - ox, ev.client_y() - oy);
            let canvas = mdl.canvas.get().expect("get canvas element");
            let ctx = seed::canvas_context_2d(&canvas);
            let x = canv_pos.0;
            let y = canv_pos.1;
            mdl.detect_hover((x as f32, y as f32));
        }
        // Task(Ok((id, res))) => {
        //     mdl.subjects.get_mut(&id).unwrap().learning_objectives = res;
        // }
        _ => log!("impl me", msg),
    }
}

async fn fetch_cg_graph() -> fetch::Result<learning_trajectory::CGGraph> {
    let result = Request::new("api/graph/cg_graph")
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await?;
    Ok(result)
}

pub fn view(model: &Model) -> Node<Message> {
    ul![
        li![button![
            "get cg_graph",
            ev(Ev::Click, |_| Message::FetchCGGraph)
        ]],
        canvas![
            el_ref(&model.canvas),
            attrs![
                At::Width => px(WIDTH),
                At::Height => px(HEIGHT),
            ],
            style![
                St::Border => "1px solid black",
            ],
            mouse_ev(Ev::MouseEnter, |mouse_event| Message::CanvasMouse(
                mouse_event
            )),
            mouse_ev(Ev::MouseLeave, |mouse_event| Message::CanvasMouse(
                mouse_event
            )),
            mouse_ev(Ev::MouseMove, |mouse_event| Message::CanvasMouse(
                mouse_event.unchecked_into()
            ))
        ],
        button!["Change color", ev(Ev::Click, |_| Message::ChangeColor)],
        button!["get .dot file", ev(Ev::Click, |_| Message::DotFile)],
        li![format!("{:?}", model)]
    ]
}
