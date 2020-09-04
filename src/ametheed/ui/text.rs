use specs::{Component, DenseVecStorage};
use derivative::Derivative;
use super::layout::Anchor;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize, Derivative)]
pub enum LineMode {
    Single,
    Wrap,
}
/// A component used to display text in this entity's UiTransform
#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct UiText {
    /// The string rendered by this.
    pub text: String,
    /// The height of a line of text in pixels.
    pub font_size: f32,
    /// The color of the rendered text, using a range of 0.0 to 1.0 per channel.
    pub color: [f32; 4],
    // /// The font used for rendering.
    // #[serde(skip)]
    // pub font: FontHandle,
    /// If true this will be rendered as dots instead of the text.
    pub password: bool,
    /// How the text should handle new lines.
    pub line_mode: LineMode,
    /// How to align the text within its `UiTransform`.
    pub align: Anchor,
    // /// Cached glyph positions including invisible characters, used to process mouse highlighting.
    // #[serde(skip)]
    // pub(crate) cached_glyphs: Vec<CachedGlyph>,
}

impl UiText {
    /// Initializes a new UiText
    ///
    /// # Parameters
    ///
    /// * `font`: A handle to a `Font` asset
    /// * `text`: The glyphs to render
    /// * `color`: RGBA color with a maximum of 1.0 and a minimum of 0.0 for each channel
    /// * `font_size`: A uniform scale applied to the glyphs
    /// * `line_mode`: Text mode allowing single line or multiple lines
    /// * `align`: Text alignment within its `UiTransform`
    pub fn new(
        text: String,
        color: [f32; 4],
        font_size: f32,
        line_mode: LineMode,
        align: Anchor,
    ) -> UiText {
        UiText {
            text,
            color,
            font_size,
            password: false,
            line_mode,
            align,
        }
    }
}


impl Component for UiText {
    type Storage = DenseVecStorage<Self>;
}
// If this component is attached to an entity with a UiText then that UiText is editable.
/// This component also controls how that editing works.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextEditing {
    /// The current editing cursor position, specified in terms of glyphs, not characters.
    pub cursor_position: isize,
    /// The maximum graphemes permitted in this string.
    pub max_length: usize,
    /// The amount and direction of glyphs highlighted relative to the cursor.
    pub highlight_vector: isize,
    /// The color of the text itself when highlighted.
    pub selected_text_color: [f32; 4],
    /// The text background color when highlighted.
    pub selected_background_color: [f32; 4],
    /// If this is true the text will use a block cursor for editing.  Otherwise this uses a
    /// standard line cursor.  This is not recommended if your font is not monospace.
    pub use_block_cursor: bool,

    /// This value is used to control cursor blinking.
    ///
    /// When it is greater than 0.5 / CURSOR_BLINK_RATE the cursor should not display, when it
    /// is greater than or equal to 1.0 / CURSOR_BLINK_RATE it should be reset to 0.  When the
    /// player types it should be reset to 0.
    pub(crate) cursor_blink_timer: f32,
}

impl TextEditing {
    /// Create a new TextEditing Component
    pub fn new(
        max_length: usize,
        selected_text_color: [f32; 4],
        selected_background_color: [f32; 4],
        use_block_cursor: bool,
    ) -> TextEditing {
        TextEditing {
            cursor_position: 0,
            max_length,
            highlight_vector: 0,
            selected_text_color,
            selected_background_color,
            use_block_cursor,
            cursor_blink_timer: 0.0,
        }
    }
}

impl Component for TextEditing {
    type Storage = DenseVecStorage<Self>;
}
