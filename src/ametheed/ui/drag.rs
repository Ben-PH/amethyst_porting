use serde::{Deserialize, Serialize};

use specs::{
    Component, DenseVecStorage,
};
//     nalgebra::Vector2;
//     shrev::EventChannel,
//     Hidden, HiddenPropagate, ParentHierarchy,

/// Component that denotes whether a given ui widget is draggable.
/// Requires UiTransform to work, and its expected way of usage is
/// through UiTransformData prefab.
#[derive(Debug, Serialize, Deserialize)]
pub struct Draggable;

impl Component for Draggable {
    type Storage = DenseVecStorage<Self>;
}
