
use crate::ametheed::Color;
use specs::DenseVecStorage;
use specs::Component;

pub enum UiColorBox {
    SolidColor(Color),
}

impl Component for UiColorBox {
    type Storage = DenseVecStorage<Self>;
}
