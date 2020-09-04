use crate::ametheed::ui::button::UiButton;
use crate::pages::cg_graph::ecs::components::Color;
use smallvec::{smallvec, SmallVec};
use specs::prelude::*;

use crate::ametheed::{
    // assets::loader::{Loader},
    core::transform::components::parent::{Parent},
    // ui::button::UiButton

};

use crate::ametheed::ui::{
    layout::{Anchor, Stretch}, event::{Interactable}, selection::{Selectable},
    text::LineMode,
    button::{UiButtonAction, UiButtonActionRetrigger, UiButtonActionType::{self, *}},
    text::UiText, transform::UiTransform, widgets::{WidgetId, Widgets},
};

use std::marker::PhantomData;

const DEFAULT_Z: f32 = 1.0;
const DEFAULT_WIDTH: f32 = 128.0;
const DEFAULT_HEIGHT: f32 = 64.0;
const DEFAULT_TAB_ORDER: u32 = 9;
const DEFAULT_BKGD_COLOR: [f32; 4] = [0.82, 0.83, 0.83, 1.0];
const DEFAULT_TXT_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// Container for all the resources the builder needs to make a new UiButton.
#[derive(SystemData)]
#[allow(missing_debug_implementations)]
pub struct UiButtonBuilderResources<'a, G: PartialEq + Send + Sync + 'static, I: WidgetId = u32> {
    // loader: ReadExpect<'a, Loader>,
    entities: Entities<'a>,
    background: WriteStorage<'a, Color>,
    mouse_reactive: WriteStorage<'a, Interactable>,
    parent: WriteStorage<'a, Parent>,
    text: WriteStorage<'a, UiText>,
    transform: WriteStorage<'a, UiTransform>,
    button_widgets: WriteExpect<'a, Widgets<UiButton, I>>,
    button_action_retrigger: WriteStorage<'a, UiButtonActionRetrigger>,
    selectables: WriteStorage<'a, Selectable<G>>,
}

/// Convenience structure for building a button
/// Note that since there can only be one "ui_loader" in use, and WidgetId of the UiBundle and
/// UiButtonBuilder should match, you can only use one type of WidgetId, e.g. you cant use both
/// UiButtonBuilder<(), u32> and UiButtonBuilder<(), String>.
#[derive(Debug, Clone)]
pub struct UiButtonBuilder<G, I: WidgetId> {
    id: Option<I>,
    x: f32,
    y: f32,
    z: f32,
    width: f32,
    height: f32,
    tab_order: u32,
    anchor: Anchor,
    stretch: Stretch,
    text: String,
    text_color: [f32; 4],
    font_size: f32,
    line_mode: LineMode,
    align: Anchor,
    parent: Option<Entity>,
    // SetTextColor and SetImage can occur on click/hover start,
    // Unset for both on click/hover stop, so we only need 2 max.
    on_click_start: SmallVec<[UiButtonActionType; 2]>,
    on_click_stop: SmallVec<[UiButtonActionType; 2]>,
    on_hover_start: SmallVec<[UiButtonActionType; 2]>,
    on_hover_stop: SmallVec<[UiButtonActionType; 2]>,
    _phantom: PhantomData<G>,
}

impl<G, I> Default for UiButtonBuilder<G, I>
where
    I: WidgetId,
{
    fn default() -> Self {
        UiButtonBuilder {
            id: None,
            x: 0.,
            y: 0.,
            z: DEFAULT_Z,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            tab_order: DEFAULT_TAB_ORDER,
            anchor: Anchor::TopLeft,
            stretch: Stretch::NoStretch,
            text: "".to_string(),
            text_color: DEFAULT_TXT_COLOR,
            // font: None,
            font_size: 32.,
            line_mode: LineMode::Single,
            align: Anchor::Middle,
            // image: None,
            parent: None,
            on_click_start: smallvec![],
            on_click_stop: smallvec![],
            on_hover_start: smallvec![],
            on_hover_stop: smallvec![],
            _phantom: PhantomData,
        }
    }
}

impl<'a, G: PartialEq + Send + Sync + 'static, I: WidgetId> UiButtonBuilder<G, I> {
    /// Construct a new UiButtonBuilder.
    /// This allows easy use of default values for text and button appearance and allows the user
    /// to easily set other UI-related options. It also allows easy retrieval and updating through
    /// the appropriate widgets resouce, see [`Widgets`](../../struct.Widgets.html).
    pub fn new<S: ToString>(text: S) -> UiButtonBuilder<G, I> {
        let mut builder = UiButtonBuilder::default();
        builder.text = text.to_string();
        builder
    }

    /// Sets an ID for this widget. The type of this ID will determine which `Widgets`
    /// resource this widget will be added to, see see [`Widgets`](../../struct.Widgets.html).
    pub fn with_id(mut self, id: I) -> Self {
        self.id = Some(id);
        self
    }

    /// Add a parent to the button.
    pub fn with_parent(mut self, parent: Entity) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Add an anchor to the button.
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Stretch the button.
    pub fn with_stretch(mut self, stretch: Stretch) -> Self {
        self.stretch = stretch;
        self
    }

    /// This will set the rendered characters within the button. Use this to just change what
    /// characters will appear. If you need to change the font size, color, etc., then you should
    /// use
    /// [`with_uitext`](#with_uitext) and provide a new `UiText` object.
    pub fn with_text<S>(mut self, text: S) -> Self
    where
        S: ToString,
    {
        self.text = text.to_string();
        self
    }

    /// Provide an X and Y position for the button.
    ///
    /// This will create a default UiTransform if one is not already attached.
    /// See `DEFAULT_Z`, `DEFAULT_WIDTH`, `DEFAULT_HEIGHT`, and `DEFAULT_TAB_ORDER` for
    /// the values that will be provided to the default UiTransform.
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Provide a Z position, i.e UI layer
    pub fn with_layer(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    /// Set button size
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set button tab order
    pub fn with_tab_order(mut self, tab_order: u32) -> Self {
        self.tab_order = tab_order;
        self
    }

    /// Set font size
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set text color
    pub fn with_text_color(mut self, text_color: [f32; 4]) -> Self {
        self.text_color = text_color;
        self
    }

    /// Set text line mode
    pub fn with_line_mode(mut self, line_mode: LineMode) -> Self {
        self.line_mode = line_mode;
        self
    }

    /// Set text align
    pub fn with_align(mut self, align: Anchor) -> Self {
        self.align = align;
        self
    }

    /// Text color to use when the mouse is hovering over this button
    pub fn with_hover_text_color(mut self, text_color: [f32; 4]) -> Self {
        self.on_hover_start.push(SetTextColor(text_color));
        self.on_hover_stop.push(UnsetTextColor(text_color));
        self
    }

    /// Set text color when the button is pressed
    pub fn with_press_text_color(mut self, text_color: [f32; 4]) -> Self {
        self.on_click_start.push(SetTextColor(text_color));
        self.on_click_stop.push(UnsetTextColor(text_color));
        self
    }

    /// Build this with the `UiButtonBuilderResources`.
    pub fn build(mut self, res: &mut UiButtonBuilderResources<'a, G, I>) -> (I, UiButton) {
        let color_entity = res.entities.create();
        let text_entity = res.entities.create();
        let widget = UiButton::new(text_entity, color_entity);

        let id = {
            let widget = widget.clone();

            if let Some(id) = self.id {
                let added_id = id.clone();
                res.button_widgets.add_with_id(id, widget);
                added_id
            } else {
                res.button_widgets.add(widget)
            }
        };

        if !self.on_click_start.is_empty()
            || !self.on_click_stop.is_empty()
            || !self.on_hover_start.is_empty()
            || !self.on_hover_stop.is_empty()
        {
            let retrigger = UiButtonActionRetrigger {
                on_click_start: actions_with_target(
                    &mut self.on_click_start.into_iter(),
                    color_entity,
                ),
                on_click_stop: actions_with_target(
                    &mut self.on_click_stop.into_iter(),
                    color_entity,
                ),
                on_hover_start: actions_with_target(
                    &mut self.on_hover_start.into_iter(),
                    color_entity,
                ),
                on_hover_stop: actions_with_target(
                    &mut self.on_hover_stop.into_iter(),
                    color_entity,
                ),
            };

            res.button_action_retrigger
                .insert(color_entity, retrigger)
                .expect("Unreachable: Inserting newly created entity");
        }


        res.transform
            .insert(
                color_entity,
                UiTransform::new(
                    format!("{}_btn", id),
                    self.anchor,
                    Anchor::Middle,
                    self.x,
                    self.y,
                    self.z,
                    self.width,
                    self.height,
                )
                .with_stretch(self.stretch),
            )
            .expect("Unreachable: Inserting newly created entity");
        res.selectables
            .insert(color_entity, Selectable::<G>::new(self.tab_order))
            .expect("Unreachable: Inserting newly created entity");

        res.mouse_reactive
            .insert(color_entity, Interactable)
            .expect("Unreachable: Inserting newly created entity");
        if let Some(parent) = self.parent.take() {
            res.parent
                .insert(color_entity, Parent { entity: parent })
                .expect("Unreachable: Inserting newly created entity");
        }

        res.transform
            .insert(
                text_entity,
                UiTransform::new(
                    format!("{}_btn_text", id),
                    Anchor::Middle,
                    Anchor::Middle,
                    0.,
                    0.,
                    0.01,
                    0.,
                    0.,
                )
                .into_transparent()
                .with_stretch(Stretch::XY {
                    x_margin: 0.,
                    y_margin: 0.,
                    keep_aspect_ratio: false,
                }),
            )
            .expect("Unreachable: Inserting newly created entity");
        res.text
            .insert(
                text_entity,
                UiText::new(
                    self.text,
                    self.text_color,
                    self.font_size,
                    self.line_mode,
                    self.align,
                ),
            )
            .expect("Unreachable: Inserting newly created entity");
        res.parent
            .insert(
                text_entity,
                Parent {
                    entity: color_entity,
                },
            )
            .expect("Unreachable: Inserting newly created entity");

        (id, widget)
    }

    /// Create the UiButton based on provided configuration parameters.
    pub fn build_from_world(self, world: &World) -> (I, UiButton) {
        self.build(&mut UiButtonBuilderResources::<G, I>::fetch(&world))
    }
}

fn actions_with_target<I>(actions: I, target: Entity) -> Vec<UiButtonAction>
where
    I: Iterator<Item = UiButtonActionType>,
{
    actions
        .map(|action| UiButtonAction {
            target,
            event_type: action,
        })
        .collect()
}
