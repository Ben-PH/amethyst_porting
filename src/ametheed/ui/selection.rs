use serde::{Deserialize, Serialize};
use derive_new::new;

use specs::{Component, DenseVecStorage, FlaggedStorage};


// TODO: If none selected and there is a Selectable in the World, select the lower ordered one automatically?

/// Component indicating that a Ui entity is selectable.
/// Generic Type:
/// - G: Selection Group. Used to determine which entities can be selected together at the same time.
#[derive(Debug, Serialize, Deserialize, new)]
pub struct Selectable<G> {
    /// The order in which entities are selected when pressing the `Tab` key or the "go to next" input action.
    pub order: u32,
    #[new(default)]
    /// A multi selection group. When multiple entities are in the same selection group, they can be selected at
    /// the same time by holding shift or control and clicking them.
    /// You can also select the first element, then hold shift and press the keyboard arrow keys.
    // TODO: Holding shift + arrow keys to select more.
    // TODO: Pressing the arrow keys could optionally be binded to change the selected ui element.
    pub multi_select_group: Option<G>,
    #[new(default)]
    /// Indicates if you can select multiple entities at once without having to press the shift or control key.
    pub auto_multi_select: bool,
    /// Indicates if this consumes the inputs. If enabled, all inputs (except Tab) will be ignored when the component is focused.
    /// For example, the arrow keys will not change the selected ui element.
    /// Example usage: Ui Editable Text.
    #[new(default)]
    pub consumes_inputs: bool,
}

impl<G: Send + Sync + 'static> Component for Selectable<G> {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

