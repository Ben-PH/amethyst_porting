#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum Anchor {
    /// Anchors the entity at the top left of the parent.
    TopLeft,
    /// Anchors the entity at the top middle of the parent.
    TopMiddle,
    /// Anchors the entity at the top right of the parent.
    TopRight,
    /// Anchors the entity at the middle left of the parent.
    MiddleLeft,
    /// Anchors the entity at the center of the parent.
    Middle,
    /// Anchors the entity at the middle right of the parent.
    MiddleRight,
    /// Anchors the entity at the bottom left of the parent.
    BottomLeft,
    /// Anchors the entity at the bottom middle of the parent.
    BottomMiddle,
    /// Anchors the entity at the bottom right of the parent.
    BottomRight,
}

impl Anchor {
    /// Returns the normalized offset using the `Anchor` setting.
    /// The normalized offset is a [-0.5,0.5] value
    /// indicating the relative offset multiplier from the parent's position (centered).
    pub fn norm_offset(self) -> (f32, f32) {
        match self {
            Anchor::TopLeft => (-0.5, 0.5),
            Anchor::TopMiddle => (0.0, 0.5),
            Anchor::TopRight => (0.5, 0.5),
            Anchor::MiddleLeft => (-0.5, 0.0),
            Anchor::Middle => (0.0, 0.0),
            Anchor::MiddleRight => (0.5, 0.0),
            Anchor::BottomLeft => (-0.5, -0.5),
            Anchor::BottomMiddle => (0.0, -0.5),
            Anchor::BottomRight => (0.5, -0.5),
        }
    }

    /// Vertical align. Used by the `UiGlyphsSystem`.
    pub(crate) fn vertical_align(self) -> VerticalAlign {
        match self {
            Anchor::TopLeft => VerticalAlign::Top,
            Anchor::TopMiddle => VerticalAlign::Top,
            Anchor::TopRight => VerticalAlign::Top,
            Anchor::MiddleLeft => VerticalAlign::Center,
            Anchor::Middle => VerticalAlign::Center,
            Anchor::MiddleRight => VerticalAlign::Center,
            Anchor::BottomLeft => VerticalAlign::Bottom,
            Anchor::BottomMiddle => VerticalAlign::Bottom,
            Anchor::BottomRight => VerticalAlign::Bottom,
        }
    }

    /// Horizontal align. Used by the `UiGlyphsSystem`.
    pub(crate) fn horizontal_align(self) -> HorizontalAlign {
        match self {
            Anchor::TopLeft => HorizontalAlign::Left,
            Anchor::TopMiddle => HorizontalAlign::Center,
            Anchor::TopRight => HorizontalAlign::Right,
            Anchor::MiddleLeft => HorizontalAlign::Left,
            Anchor::Middle => HorizontalAlign::Center,
            Anchor::MiddleRight => HorizontalAlign::Right,
            Anchor::BottomLeft => HorizontalAlign::Left,
            Anchor::BottomMiddle => HorizontalAlign::Center,
            Anchor::BottomRight => HorizontalAlign::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Stretch {
    /// No stretching occurs
    NoStretch,
    /// Stretches on the X axis.
    X {
        /// The margin length for the width
        x_margin: f32,
    },
    /// Stretches on the Y axis.
    Y {
        /// The margin length for the height
        y_margin: f32,
    },
    /// Stretches on both axes.
    XY {
        /// The margin length for the width
        x_margin: f32,
        /// The margin length for the height
        y_margin: f32,
        /// Keep the aspect ratio by adding more margin to one axis when necessary
        keep_aspect_ratio: bool,
    },
}

/// Indicates if the position and margins should be calculated in pixel or
/// relative to their parent size.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum ScaleMode {
    /// Use directly the pixel value.
    Pixel,
    /// Use a proportion (%) of the parent's dimensions (or screen, if there is no parent).
    Percent,
}
