use crate::ametheed::Interactable;
use crate::ametheed::UiTransform;
use crate::ametheed::ui::button::UiButtonActionRetrigger;
use crate::ametheed::UiText;
use crate::ametheed::ui::colored_box::UiColorBox;
use specs::{World, WorldExt};
pub mod components;
mod systems;
use components::*;
pub use components::Color;
use crate::ametheed::UiButton;

#[derive(Default)]
pub struct State {
    pub inner: World,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("World").finish()
    }
}
impl State {
    pub fn init() -> State {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Renderable>();
        world.register::<UiColorBox>();
        world.register::<UiText>();
        world.register::<UiButtonActionRetrigger>();
        world.register::<UiTransform>();
        world.register::<Interactable>();
        // world.register::<Hovered>();
        State { inner: world }
    }
}
#[derive(Default)]
pub struct DTime(std::time::Duration);
