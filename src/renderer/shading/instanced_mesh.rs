use crate::renderer::shading::*;
use crate::renderer::*;

impl ShadedGeometry for InstancedMesh {
    fn geometry_pass(
        &self,
        render_states: RenderStates,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
        material: &Material,
    ) -> Result<(), Error> {
        let fragment_shader_source = geometry_fragment_shader(material);
        let program = self.get_or_insert_program(&fragment_shader_source)?;
        material.bind(program)?;
        self.render(program, render_states, camera_buffer, viewport)
    }

    fn render_with_lighting(
        &self,
        render_states: RenderStates,
        camera: &Camera,
        material: &Material,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<(), Error> {
        let fragment_shader_source = shaded_fragment_shader(
            self.lighting_model,
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
            program,
            render_states,
            camera.uniform_buffer(),
            camera.viewport(),
        )?;
        Ok(())
    }
}
