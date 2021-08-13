use crate::core::*;
use crate::renderer::Geometry;

impl Mesh {
    ///
    /// Render the mesh with a color per triangle vertex. The colors are defined when constructing the mesh and are assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no colors.
    ///
    pub fn render_color(&self, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program(&format!(
            "{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh_vertex_color.frag")
        ))?;
        self.render(
            self.render_states(self.transparent),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    ///
    /// Render the mesh with the given color. The color is assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn render_with_color(&self, color: &Color, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_color.frag"))?;
        program.use_uniform_vec4("color", &color.to_vec4())?;
        self.render(
            self.render_states(color.a != 255),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    ///
    /// Render the uv coordinates of the mesh in red (u) and green (v) for debug purposes.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no uv coordinates.
    ///
    pub fn render_uvs(&self, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_uvs.frag"))?;
        self.render(
            self.render_states(false),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    ///
    /// Render the normals of the mesh for debug purposes.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no normals.
    ///
    pub fn render_normals(&self, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_normals.frag"))?;
        self.render(
            self.render_states(false),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    ///
    /// Render the mesh with the given texture which is assumed to be in sRGB color space with or without an alpha channel.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    /// # Errors
    /// Will return an error if the mesh has no uv coordinates.
    ///
    pub fn render_with_texture(
        &self,
        texture: &impl Texture,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_texture.frag"))?;
        program.use_texture("tex", texture)?;
        self.render(
            self.render_states(texture.format() == Format::RGBA),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    pub(crate) fn render_states(&self, transparent: bool) -> RenderStates {
        if transparent {
            RenderStates {
                cull: self.cull,
                write_mask: WriteMask::COLOR,
                blend: Some(BlendParameters::TRANSPARENCY),
                ..Default::default()
            }
        } else {
            RenderStates {
                cull: self.cull,
                ..Default::default()
            }
        }
    }
}

impl Geometry for Mesh {
    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_pick.frag"))?;
        program.use_uniform_float("maxDistance", &max_depth)?;
        self.render(
            RenderStates {
                write_mask: WriteMask {
                    red: true,
                    depth: true,
                    ..WriteMask::NONE
                },
                cull: self.cull,
                ..Default::default()
            },
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    fn render_depth(&self, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program("void main() {}")?;
        self.render(
            RenderStates {
                write_mask: WriteMask::DEPTH,
                cull: self.cull,
                ..Default::default()
            },
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    fn aabb(&self) -> Option<AxisAlignedBoundingBox> {
        self.aabb()
    }
}
