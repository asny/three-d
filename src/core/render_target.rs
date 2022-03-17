//!
//! Functionality for rendering to the screen or into textures.
//!
//!
mod screen;
#[doc(inline)]
pub use screen::*;

mod render_target2d;
#[doc(inline)]
pub use render_target2d::*;

mod render_target2d_array;
#[doc(inline)]
pub use render_target2d_array::*;

mod render_target_cube_map;
#[doc(inline)]
pub use render_target_cube_map::*;

use crate::core::*;

///
/// Defines which channels (red, green, blue, alpha and depth) to clear when starting to write to a
/// [RenderTarget] or the [Screen].
/// If `None` then the channel is not cleared and if `Some(value)` the channel is cleared to that value (the value must be between 0 and 1).
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ClearState {
    /// Defines the clear value for the red channel.
    pub red: Option<f32>,
    /// Defines the clear value for the green channel.
    pub green: Option<f32>,
    /// Defines the clear value for the blue channel.
    pub blue: Option<f32>,
    /// Defines the clear value for the alpha channel.
    pub alpha: Option<f32>,
    /// Defines the clear value for the depth channel. A value of 1 means a depth value equal to the far plane and 0 means a depth value equal to the near plane.
    pub depth: Option<f32>,
}

impl ClearState {
    ///
    /// Nothing will be cleared.
    ///
    pub const fn none() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
            alpha: None,
            depth: None,
        }
    }

    ///
    /// The depth will be cleared to the given value.
    ///
    pub const fn depth(depth: f32) -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
            alpha: None,
            depth: Some(depth),
        }
    }

    ///
    /// The color channels (red, green, blue and alpha) will be cleared to the given values.
    ///
    pub const fn color(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red: Some(red),
            green: Some(green),
            blue: Some(blue),
            alpha: Some(alpha),
            depth: None,
        }
    }

    ///
    /// Both the color channels (red, green, blue and alpha) and depth will be cleared to the given values.
    ///
    pub const fn color_and_depth(red: f32, green: f32, blue: f32, alpha: f32, depth: f32) -> Self {
        Self {
            red: Some(red),
            green: Some(green),
            blue: Some(blue),
            alpha: Some(alpha),
            depth: Some(depth),
        }
    }
}

impl Default for ClearState {
    fn default() -> Self {
        Self::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0)
    }
}

///
/// The destination of applying a copy.
///
pub enum CopyDestination<'a, 'b, 'c, 'd, T: TextureDataType> {
    /// Copies to the [Screen].
    Screen,
    /// Copies to a [Texture2D].
    ColorTexture(&'d mut Texture2D<T>),
    /// Copies to a [DepthTargetTexture2D].
    DepthTexture(&'d mut DepthTargetTexture2D),
    /// Copies to a [RenderTarget].
    RenderTarget(&'c RenderTarget<'a, 'b, T>),
}

pub(in crate::core) fn new_framebuffer(
    context: &Context,
) -> ThreeDResult<crate::context::Framebuffer> {
    unsafe {
        Ok(context
            .create_framebuffer()
            .map_err(|e| CoreError::RenderTargetCreation(e))?)
    }
}

fn clear(context: &Context, clear_state: &ClearState) {
    Program::set_write_mask(
        context,
        WriteMask {
            red: clear_state.red.is_some(),
            green: clear_state.green.is_some(),
            blue: clear_state.blue.is_some(),
            alpha: clear_state.alpha.is_some(),
            depth: clear_state.depth.is_some(),
        },
    );
    let clear_color = clear_state.red.is_some()
        || clear_state.green.is_some()
        || clear_state.blue.is_some()
        || clear_state.alpha.is_some();
    unsafe {
        if clear_color {
            context.clear_color(
                clear_state.red.unwrap_or(0.0),
                clear_state.green.unwrap_or(0.0),
                clear_state.blue.unwrap_or(0.0),
                clear_state.alpha.unwrap_or(1.0),
            );
        }
        if let Some(depth) = clear_state.depth {
            context.clear_depth_f32(depth);
        }
        context.clear(if clear_color && clear_state.depth.is_some() {
            crate::context::COLOR_BUFFER_BIT | crate::context::DEPTH_BUFFER_BIT
        } else {
            if clear_color {
                crate::context::COLOR_BUFFER_BIT
            } else {
                crate::context::DEPTH_BUFFER_BIT
            }
        });
    }
}

fn copy_from(
    context: &Context,
    color_texture: Option<&Texture2D<impl TextureDataType>>,
    depth_texture: Option<&DepthTargetTexture2D>,
    viewport: Viewport,
    write_mask: WriteMask,
) -> ThreeDResult<()> {
    if color_texture.is_some() || depth_texture.is_some() {
        let fragment_shader_source = if color_texture.is_some() && depth_texture.is_some() {
            "
            uniform sampler2D colorMap;
            uniform sampler2D depthMap;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                color = texture(colorMap, uv);
                gl_FragDepth = texture(depthMap, uv).r;
            }"
        } else if color_texture.is_some() {
            "
            uniform sampler2D colorMap;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                color = texture(colorMap, uv);
            }"
        } else {
            "
            uniform sampler2D depthMap;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                gl_FragDepth = texture(depthMap, uv).r;
            }"
        };
        context.effect(fragment_shader_source, |effect| {
            if let Some(tex) = color_texture {
                effect.use_texture("colorMap", tex)?;
            }
            if let Some(tex) = depth_texture {
                effect.use_texture("depthMap", tex)?;
            }
            effect.apply(
                RenderStates {
                    depth_test: DepthTest::Always,
                    write_mask: WriteMask {
                        red: color_texture.is_some() && write_mask.red,
                        green: color_texture.is_some() && write_mask.green,
                        blue: color_texture.is_some() && write_mask.blue,
                        alpha: color_texture.is_some() && write_mask.alpha,
                        depth: depth_texture.is_some() && write_mask.depth,
                    },
                    ..Default::default()
                },
                viewport,
            )
        })
    } else {
        Ok(())
    }
}

fn copy_from_array(
    context: &Context,
    color_texture: Option<(&Texture2DArray<impl TextureDataType>, u32)>,
    depth_texture: Option<(&DepthTargetTexture2DArray, u32)>,
    viewport: Viewport,
    write_mask: WriteMask,
) -> ThreeDResult<()> {
    if color_texture.is_some() || depth_texture.is_some() {
        let fragment_shader_source = if color_texture.is_some() && depth_texture.is_some() {
            "
            uniform sampler2DArray colorMap;
            uniform sampler2DArray depthMap;
            uniform int colorLayer;
            uniform int depthLayer;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                color = texture(colorMap, vec3(uv, colorLayer));
                gl_FragDepth = texture(depthMap, vec3(uv, depthLayer)).r;
            }"
        } else if color_texture.is_some() {
            "
            uniform sampler2DArray colorMap;
            uniform int colorLayer;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                color = texture(colorMap, vec3(uv, colorLayer));
            }"
        } else {
            "
            uniform sampler2DArray depthMap;
            uniform int depthLayer;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                gl_FragDepth = texture(depthMap, vec3(uv, depthLayer)).r;
            }"
        };
        context.effect(fragment_shader_source, |effect| {
            if let Some((tex, layer)) = color_texture {
                effect.use_texture_array("colorMap", tex)?;
                effect.use_uniform("colorLayer", layer as i32)?;
            }
            if let Some((tex, layer)) = depth_texture {
                effect.use_texture_array("depthMap", tex)?;
                effect.use_uniform("depthLayer", layer as i32)?;
            }
            effect.apply(
                RenderStates {
                    depth_test: DepthTest::Always,
                    write_mask: WriteMask {
                        red: color_texture.is_some() && write_mask.red,
                        green: color_texture.is_some() && write_mask.green,
                        blue: color_texture.is_some() && write_mask.blue,
                        alpha: color_texture.is_some() && write_mask.alpha,
                        depth: depth_texture.is_some() && write_mask.depth,
                    },
                    ..Default::default()
                },
                viewport,
            )
        })
    } else {
        Ok(())
    }
}
