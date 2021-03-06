//! Rendering and window management for the `arcs` CAD library.
//!
//! # A Note on Conventions
//!
//! The rendering system needs to work with two coordinate systems at the same
//! time. To avoid confusion,
//!
//! > **In *Drawing Space* we'll use [`crate::Vector`] and types from
//! > [`crate::primitives`], and when in *Canvas Space* we'll use types from
//! > the [`kurbo`] crate**

mod utils;
mod window;

pub use utils::{
    to_canvas_coordinates, to_drawing_coordinates, transform_to_canvas_space,
    transform_to_drawing_space,
};
pub use window::Window;
