
use specs::DenseVecStorage;
use specs::Component;

pub enum UiColorBox {
    SolidColor([f32; 4]),
}

impl Component for UiColorBox {
    type Storage = DenseVecStorage<Self>;
}
