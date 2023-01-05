use crate::core::*;

pub struct ColorTargetMultisample<C: TextureDataType>(RenderTargetMultisample<C, f32>);

impl<C: TextureDataType + Default> ColorTargetMultisample<C> {
    pub fn new(context: &Context, width: u32, height: u32, number_of_samples: u32) -> Self {
        Self(RenderTargetMultisample::new_color(
            context,
            width,
            height,
            number_of_samples,
        ))
    }
}

impl<C: TextureDataType> std::ops::Deref for ColorTargetMultisample<C> {
    type Target = RenderTargetMultisample<C, f32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C: TextureDataType> std::ops::DerefMut for ColorTargetMultisample<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
