//! Based on the amethyst project. Scratch-that, it started as a 99%
//! copy/paste.
//!
//! This doc comment was written while in the process of importing code
//! from amethyst to make the the API used in one of its UI examples
//! implemented within this project.
//!
//! this is what I'm bringing in:
//! ```rust,ignore
//! use amethyst::{
//!     assets::{PrefabLoader, PrefabLoaderSystemDesc, RonFormat},
//!     core::transform::TransformBundle,
//!     ecs::prelude::WorldExt, // ecs is alias for specs
//!     input::{InputBundle, StringBindings},
//!     prelude::*,
//!     // renderer::{
//!     //     plugins::RenderToWindow,
//!     //     rendy::mesh::{Normal, Position, TexCoord},
//!     //     types::DefaultBackend,
//!     //     RenderingBundle,
//!     // },
//!     ui::{RenderUi, ToNativeWidget, UiBundle, UiCreator, UiTransformData, UiWidget},
//!     utils::{application_root_dir, scene::BasicScenePrefab},
//! };
//! ```
//! however, I don't want renderer (ising web-sys will be simpler)
//!
//! I've copied the file in its entirety into eg_reference_main.rs, and
//! it's not to be imported - for reference ONLY.

mod error;
// mod assets;
pub mod ui;
mod core;
pub use specs::Entity;
pub use specs_hierarchy::Parent;
pub use ui::text::UiText;
pub use ui::event::Interactable;
use crate::pages::cg_graph::ecs::Color;
pub use ui::button::UiButton;
pub use ui::button::UiButtonAction;

// use assets::storage::Handle;
use ui::transform::UiTransform;

