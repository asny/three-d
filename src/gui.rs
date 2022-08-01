//!
//! Graphical User Interface support.
//!

#[cfg(feature = "gui")]
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
mod egui_gui;
#[doc(inline)]
#[cfg(feature = "gui")]
pub use egui_gui::*;
