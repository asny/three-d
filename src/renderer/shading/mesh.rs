use crate::renderer::shading::*;
use crate::renderer::*;

impl ShadedGeometry for Mesh {
    fn geometry_pass(
        &self,
        camera: &Camera,
        viewport: Viewport,
        material: &Material,
    ) -> Result<(), Error> {
        let fragment_shader_source = geometry_fragment_shader(material);
        let program = self.get_or_insert_program(&fragment_shader_source)?;
        material.bind(program)?;
        self.render(
            RenderStates {
                cull: self.cull,
                ..Default::default()
            },
            program,
            camera.uniform_buffer(),
            viewport,
        )
    }

    fn render_with_lighting(
        &self,
        camera: &Camera,
        material: &Material,
        lighting_model: LightingModel,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<(), Error> {
        let fragment_shader_source = shaded_fragment_shader(
            lighting_model,
            Some(material),
            directional_lights.len(),
            spot_lights.len(),
            point_lights.len(),
        );
        let program = self.get_or_insert_program(&fragment_shader_source)?;

        bind_lights(
            program,
            ambient_light,
            directional_lights,
            spot_lights,
            point_lights,
            camera.position(),
        )?;
        material.bind(program)?;
        self.render(
            self.render_states(
                material.albedo[3] != 1.0
                    || material
                        .albedo_texture
                        .as_ref()
                        .map(|t| t.format() == Format::RGBA)
                        .unwrap_or(false),
            ),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )?;
        Ok(())
    }
}
