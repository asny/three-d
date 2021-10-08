use crate::core::*;
use crate::renderer::light::*;
use crate::renderer::*;

///
/// A light which shines in the given direction.
/// The light will cast shadows if you [generate a shadow map](DirectionalLight::generate_shadow_map).
///
pub struct DirectionalLight {
    context: Context,
    light_buffer: UniformBuffer,
    shadow_texture: Option<DepthTargetTexture2D>,
}

impl DirectionalLight {
    pub fn new(
        context: &Context,
        intensity: f32,
        color: Color,
        direction: &Vec3,
    ) -> Result<DirectionalLight> {
        let mut light = DirectionalLight {
            context: context.clone(),
            light_buffer: UniformBuffer::new(context, &[3u32, 1, 3, 1, 16])?,
            shadow_texture: None,
        };

        light.set_intensity(intensity);
        light.set_color(color);
        light.set_direction(direction);
        Ok(light)
    }

    pub fn set_color(&mut self, color: Color) {
        self.light_buffer.update(0, &color.to_rgb_slice()).unwrap();
    }

    pub fn color(&self) -> Color {
        let c = self.light_buffer.get(0).unwrap();
        Color::new_from_rgb_slice(&[c[0], c[1], c[2]])
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn intensity(&self) -> f32 {
        self.light_buffer.get(1).unwrap()[0]
    }

    pub fn set_direction(&mut self, direction: &Vec3) {
        self.light_buffer
            .update(2, &direction.normalize().to_slice())
            .unwrap();
    }

    pub fn direction(&self) -> Vec3 {
        let d = self.light_buffer.get(2).unwrap();
        vec3(d[0], d[1], d[2])
    }

    pub fn clear_shadow_map(&mut self) {
        self.shadow_texture = None;
        self.light_buffer.update(3, &[0.0]).unwrap();
    }

    pub fn generate_shadow_map<T: Shadable>(
        &mut self,
        target: &Vec3,
        frustrum_height: f32,
        frustrum_depth: f32,
        texture_width: u32,
        texture_height: u32,
        geometries: &[T],
    ) -> Result<()> {
        let direction = self.direction();
        let up = compute_up_direction(direction);

        let viewport = Viewport::new_at_origo(texture_width, texture_height);
        let shadow_camera = Camera::new_orthographic(
            &self.context,
            viewport,
            target - direction.normalize() * 0.5 * frustrum_depth,
            *target,
            up,
            frustrum_height,
            0.0,
            frustrum_depth,
        )?;
        self.light_buffer
            .update(4, &shadow_matrix(&shadow_camera).to_slice())?;

        let shadow_texture = DepthTargetTexture2D::new(
            &self.context,
            texture_width,
            texture_height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?;
        let depth_material = DepthMaterial {
            render_states: RenderStates {
                write_mask: WriteMask::DEPTH,
                ..Default::default()
            },
            ..Default::default()
        };
        shadow_texture.write(Some(1.0), || {
            for geometry in geometries {
                geometry.render_forward(&depth_material, &shadow_camera)?;
            }
            Ok(())
        })?;
        self.shadow_texture = Some(shadow_texture);
        self.light_buffer.update(3, &[1.0])?;
        Ok(())
    }

    pub fn shadow_map(&self) -> Option<&DepthTargetTexture2D> {
        self.shadow_texture.as_ref()
    }

    pub fn buffer(&self) -> &UniformBuffer {
        &self.light_buffer
    }
}

impl Light for DirectionalLight {
    fn shader_source(&self, i: u32) -> String {
        format!(
        "
            uniform sampler2D shadowMap{};
            layout (std140) uniform LightUniform{}
            {{
                BaseLight base{};
                vec3 direction{};
                float shadowEnabled{};
                mat4 shadowMVP{};
            }};
            vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
            {{
                if(base{}.intensity > 0.001) {{
                    vec3 light_color = base{}.intensity * base{}.color;
                    vec3 result = calculate_light(light_color, -direction{}, surface_color, position, normal, metallic, roughness, occlusion);
                    if(shadowEnabled{} > 0.5) {{
                        result *= calculate_shadow(shadowMap{}, shadowMVP{}, position);
                    }}
                    return result;
                }}
                else {{
                    return vec3(0.0, 0.0, 0.0);
                }}
            }}
        
        ", i, i, i, i, i, i, i, i, i, i, i, i, i, i)
    }
    fn bind(&self, program: &Program, camera: &Camera, i: u32) -> Result<()> {
        if let Some(tex) = self.shadow_map() {
            program.use_texture(&format!("shadowMap{}", i), tex)?;
        } else {
            self.context
                .use_texture_dummy(&program, &format!("shadowMap{}", i))?;
        }
        program.use_uniform_vec3("eyePosition", camera.position())?;
        program.use_uniform_block(&format!("LightUniform{}", i), self.buffer());
        Ok(())
    }
}
