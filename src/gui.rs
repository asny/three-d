//!
//! Graphical User Interface support.
//!

#[cfg(feature = "egui-gui")]
mod egui_gui;
#[doc(inline)]
#[cfg(feature = "egui-gui")]
pub use egui_gui::*;
