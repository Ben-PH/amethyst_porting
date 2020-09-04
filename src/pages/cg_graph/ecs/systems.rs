use crate::pages::cg_graph::ecs::components::{Position, Velocity};
use crate::pages::cg_graph::ecs::DTime;
use specs::{Join, Read, ReadStorage, System, WriteStorage,};

pub struct MousePos {
    x: f64,
    y: f64,
}

pub struct UpdatePos;
impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;
        let delta = delta.0;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x = pos.x + (delta.as_millis() as f32 / 1000.) * vel.x;
            pos.y = pos.y + (delta.as_millis() as f32 / 1000.) * vel.y;
        }
    }
}

// pub struct CheckHover;
// impl<'a> System<'a> for CheckHover {
//     type SystemData = (
//         WriteStorage<'a, Hovered>,
//         Read<'a, MousePos>,
//     );
//     fn run(&mut self, data: Self::SystemData) {
//     }
// }
