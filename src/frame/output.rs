use crate::frame::*;

#[derive(Clone, Debug)]
pub struct FrameOutput {
    pub exit: bool,
    pub redraw: bool
}

impl FrameOutput {
    pub fn new_from_input(input: &FrameInput) -> Self {
        Self { redraw: input.first_frame, ..Default::default()}
    }
}

impl Default for FrameOutput {
    fn default() -> Self {
        Self {
            exit: false,
            redraw: true
        }
    }
}