use specs::{Component, VecStorage, WorldExt};

#[derive(Component, Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn html_str(&self) -> String {
        format!("#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Hovered(bool);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
