use specs::prelude::*;
pub use self::{
    actions::{UiButtonAction, UiButtonActionType},
    builder::{UiButtonBuilder, UiButtonBuilderResources},
    retrigger::{
        UiButtonActionRetrigger, UiButtonActionRetriggerSystem, UiButtonActionRetriggerSystemDesc,
    },
    system::{UiButtonSystem, UiButtonSystemDesc},
};
use crate::ametheed::ui::widgets::Widget;
use crate::ametheed::{UiText, Interactable, Color, UiTransform};
pub use crate::ametheed::core::transform::components::parent::*;
mod actions;
pub mod builder;
mod retrigger;
pub mod system;

#[derive(Debug, Clone)]
pub struct UiButton {

    color_entity: Entity,
    text_entity: Entity,
}
impl Widget for UiButton {}
impl UiButton {
    /// Create a new $t widget from its associated entities.
    pub fn new(text_entity: Entity, color_entity: Entity) -> Self {
        Self {
            text_entity,
            color_entity,
        }
    }
    /// Get a reference to the $t component for this widget.
    pub fn get_position<'a>(
        &self,
        storage: &'a ReadStorage<'a, UiTransform>,
    ) -> &'a UiTransform {
        storage
            .get(self.color_entity)
            .expect("Component should exist on entity")
    }
    /// Get a mutable reference to the $t component for this widget.
    pub fn get_position_mut<'a>(
        &self,
        storage: &'a mut WriteStorage<'a, UiTransform>,
    ) -> &'a mut UiTransform {
        storage
            .get_mut(self.color_entity)
            .expect("Component should exist on entity")
    }
    /// Get a reference to the $t component for this widget.
    pub fn get_text_position<'a>(
        &self,
        storage: &'a ReadStorage<'a, UiTransform>,
    ) -> &'a UiTransform {
        storage
            .get(self.text_entity)
            .expect("Component should exist on entity")
    }
    /// Get a mutable reference to the $t component for this widget.
    pub fn get_text_position_mut<'a>(
        &self,
        storage: &'a mut WriteStorage<'a, UiTransform>,
    ) -> &'a mut UiTransform {
        storage
            .get_mut(self.text_entity)
            .expect("Component should exist on entity")
    }
    /// Get a reference to the $t component for this widget.
    pub fn get_color<'a>(
        &self,
        storage: &'a ReadStorage<'a, Color>,
    ) -> &'a Color {
        storage
            .get(self.color_entity)
            .expect("Component should exist on entity")
    }
    /// Get a mutable reference to the $t component for this widget.
    pub fn get_texture_mut<'a>(
        &self,
        storage: &'a mut WriteStorage<'a, Color>,
    ) -> &'a mut Color {
        storage
            .get_mut(self.color_entity)
            .expect("Component should exist on entity")
    }
    /// Get a reference to the $t component for this widget.
    pub fn get_mouse_reactive<'a>(
        &self,
        storage: &'a ReadStorage<'a, Interactable>,
    ) -> &'a Interactable {
        storage
            .get(self.color_entity)
            .expect("Component should exist on entity")
    }
    /// Get a mutable reference to the $t component for this widget.
    pub fn get_mouse_reactive_mut<'a>(
        &self,
        storage: &'a mut WriteStorage<'a, Interactable>,
    ) -> &'a mut Interactable {
        storage
            .get_mut(self.color_entity)
            .expect("Component should exist on entity")
    }
    /// Get a reference to the $t component for this widget.
    pub fn get_text<'a>(
        &self,
        storage: &'a ReadStorage<'a, UiText>,
    ) -> &'a UiText {
        storage
            .get(self.text_entity)
            .expect("Component should exist on entity")
    }
    /// Get a mutable reference to the $t component for this widget.
    pub fn get_text_mut<'a>(
        &self,
        storage: &'a mut WriteStorage<'a, UiText>,
    ) -> &'a mut UiText {
        storage
            .get_mut(self.text_entity)
            .expect("Component should exist on entity")
    }
    /// Get a reference to the $t component for this widget if it exists,
    /// `None` otherwise.
    pub fn get_parent_maybe<'a>(
        &self,
        storage: &'a ReadStorage<'a, Parent>,
    ) -> Option<&'a Parent> {
        storage.get(self.color_entity)
    }
    /// Get a mutable reference to the $t component for this widget
    /// if it exists, `None` otherwise.
    pub fn get_parent_mut_maybe<'a>(
        &self,
        storage: &'a mut WriteStorage<'a, Parent>,
    ) -> Option<&'a mut Parent> {
        storage.get_mut(self.color_entity)
    }
    /// Get a reference to the $t component for this widget if it exists,
    /// `None` otherwise.
    pub fn get_action_retrigger_maybe<'a>(
        &self,
        storage: &'a ReadStorage<'a, UiButtonActionRetrigger>,
    ) -> Option<&'a UiButtonActionRetrigger> {
        storage.get(self.color_entity)
    }
    /// Get a mutable reference to the $t component for this widget
    /// if it exists, `None` otherwise.
    pub fn get_action_retrigger_mut_maybe<'a>(
        &self,
        storage: &'a mut WriteStorage<'a, UiButtonActionRetrigger>,
    ) -> Option<&'a mut UiButtonActionRetrigger> {
        storage.get_mut(self.color_entity)
    }
}
// crate::define_widget!(UiButton =>
//     entities: [text_entity, color_entity]
//     components: [
//         (has UiTransform as position on color_entity),
//         (has UiTransform as text_position on text_entity),
//         (has Handle<Color> as color on color_entity),
//         (has Interactable as mouse_reactive on color_entity),
//         (has UiText as text on text_entity),

//         (maybe_has Parent as parent on color_entity),
//         (maybe_has UiButtonActionRetrigger as action_retrigger on color_entity)
//     ]
// );
