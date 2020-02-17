
pub mod event;

#[cfg(feature="desktop")]
pub mod glutin_window;
#[cfg(feature="desktop")]
pub use crate::glutin_window::*;

#[cfg(feature="web")]
pub mod canvas;
#[cfg(feature="web")]
pub use crate::canvas::*;

pub struct FrameInput {
    pub events: Vec<event::Event>,
    pub elapsed_time: f64,
    pub screen_width: usize,
    pub screen_height: usize
}