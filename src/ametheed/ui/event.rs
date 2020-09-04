
use specs::prelude::*;
use nalgebra::Vector2;

/// An event that pertains to a specific `Entity`, for example a `UiEvent` for clicking on a widget
/// entity.
pub trait TargetedEvent {
    /// The `Entity` targeted by the event.
    fn get_target(&self) -> Entity;
}

/// The type of ui event.
/// Click happens if you start and stop clicking on the same ui element.
#[derive(Debug, Clone, PartialEq)]
pub enum UiEventType {
    /// When an element is clicked normally.
    /// Includes touch events.
    Click,
    /// When the element starts being clicked (On left mouse down).
    /// Includes touch events.
    ClickStart,
    /// When the element stops being clicked (On left mouse up).
    /// Includes touch events.
    ClickStop,
    /// When the cursor gets over an element.
    HoverStart,
    /// When the cursor stops being over an element.
    HoverStop,
    /// When dragging a `Draggable` Ui element.
    Dragging {
        /// The position of the mouse relative to the center of the transform when the drag started.
        offset_from_mouse: Vector2<f32>,
        /// Position at which the mouse is currently. Absolute value; not relative to the parent of the dragged entity.
        new_position: Vector2<f32>,
    },
    /// When stopping to drag a `Draggable` Ui element.
    Dropped {
        /// The entity on which the dragged object was dropped.
        dropped_on: Option<Entity>,
    },
    /// When the value of a UiText element has been changed by user input.
    ValueChange,
    /// When the value of a UiText element has been committed by user action.
    ValueCommit,
    /// When an editable UiText element has gained focus.
    Focus,
    /// When an editable UiText element has lost focus.
    Blur,
}
/// A ui event instance.
#[derive(Debug, Clone)]
pub struct UiEvent {
    /// The type of ui event.
    pub event_type: UiEventType,
    /// The entity on which the event happened.
    pub target: Entity,
}

impl UiEvent {
    /// Creates a new UiEvent.
    pub fn new(event_type: UiEventType, target: Entity) -> Self {
        UiEvent { event_type, target }
    }
}

impl TargetedEvent for UiEvent {
    fn get_target(&self) -> Entity {
        self.target
    }
}
/// A component that tags an entity as reactive to ui events.
/// Will only work if the entity has a UiTransform component attached to it.
/// Without this, the ui element will not generate events.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Interactable;

impl Component for Interactable {
    type Storage = NullStorage<Interactable>;
}
