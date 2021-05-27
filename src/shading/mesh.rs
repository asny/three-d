use crate::camera::*;
use crate::core::*;
use crate::light::*;
use crate::math::*;
use crate::object::*;
use crate::shading::*;

impl ShadedGeometry for Mesh {
    fn geometry_pass(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let fragment_shader_source = geometry_fragment_shader(&self.material);
        let program = self.get_or_insert_program(&fragment_shader_source)?;
        self.material.bind(program)?;
        self.render(program, render_states, viewport, camera)
    }

    fn render_with_lighting(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<(), Error> {
        let fragment_shader_source = shaded_fragment_shader(
            Some(&self.material),
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
        )?;
        program.use_uniform_vec3("eyePosition", &camera.position())?;
        self.material.bind(program)?;
        self.render(program, render_states, viewport, camera)?;
        Ok(())
    }
}
