
use crate::definition::*;
use crate::core::*;
use crate::phong::*;
use crate::light::*;
use crate::math::*;
use crate::camera::*;
use crate::object::*;
use std::collections::HashMap;

///
/// Forward pipeline based on the phong reflection model supporting a very limited amount of lights with shadows.
/// Supports colored, transparent, textured and instanced meshes.
///
/// *NOTE*: Forward rendering does not require a pipeline, so this is only necessary if you want a depth pre-pass.
///
pub struct PhongForwardPipeline {
    context: Context,
    program_map: HashMap<String, MeshProgram>,
    depth_texture: Option<DepthTargetTexture2D>
}

impl PhongForwardPipeline {

    pub fn new(context: &Context) -> Result<Self, Error>
    {
        Ok(Self {
            context: context.clone(),
            program_map: HashMap::new(),
            depth_texture: None
        })
    }

    pub fn render_mesh(&mut self, mesh: &PhongMesh, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &Camera,
                       ambient_light: Option<&AmbientLight>, directional_lights: &[&DirectionalLight],
                       spot_lights: &[&SpotLight], point_lights: &[&PointLight]) -> Result<(), Error>
    {
        let key = format!("{},{},{},{}", ambient_light.is_some(), directional_lights.len(), spot_lights.len(), point_lights.len());
        if !self.program_map.contains_key(&key) {
            self.program_map.insert(key.clone(),
                                    MeshProgram::new(
                                        &self.context,
                                        &crate::phong::phong_fragment_shader(
                                            &format!("
                                        {}
                                        uniform float diffuse_intensity;
                                        uniform float specular_intensity;
                                        uniform float specular_power;

                                        in vec3 pos;
                                        in vec3 nor;

                                        Surface get_surface()
                                        {{
                                            return Surface(pos, nor, get_surface_color(), diffuse_intensity, specular_intensity, specular_power);
                                        }}", match mesh.material.color_source {
                                                ColorSource::Color(_) => {"
                                                uniform vec4 surfaceColor;
                                                vec4 get_surface_color()
                                                {{
                                                    return surfaceColor;
                                                }}"},
                                                ColorSource::Texture(_) => { "
                                                uniform sampler2D tex;
                                                in vec2 uvs;
                                                vec4 get_surface_color()
                                                {{
                                                    return texture(tex, vec2(uvs.x, 1.0 - uvs.y));
                                                }}"
                                                }
                                            }),
                                            directional_lights.len(),
                                            spot_lights.len(),
                                            point_lights.len()))?);
        };
        let program = self.program_map.get(&key).unwrap();

        crate::phong::bind_lights(program, ambient_light, directional_lights, spot_lights, point_lights)?;

        if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty() {
            program.use_uniform_vec3("eyePosition", &camera.position())?;
            mesh.material.bind(program)?;
        } else {
            match mesh.material.color_source {
                ColorSource::Color(ref color) => {
                    program.use_uniform_vec4("surfaceColor", color)?;
                },
                ColorSource::Texture(ref texture) => {
                    program.use_texture(texture.as_ref(),"tex")?;
                }
            }
        }
        mesh.render(program, render_states, viewport, transformation, camera)?;
        Ok(())
    }

    pub fn depth_pass<F: FnOnce() -> Result<(), Error>>(&mut self, width: usize, height: usize, render: F) -> Result<(), Error>
    {
        self.depth_texture = Some(DepthTargetTexture2D::new(&self.context, width, height,Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, DepthFormat::Depth32F)?);
        RenderTarget::new_depth(&self.context,self.depth_texture.as_ref().unwrap())?
            .write(&ClearState::depth(1.0), render)?;
        Ok(())
    }

    pub fn depth_texture(&self) -> &dyn Texture
    {
        self.depth_texture.as_ref().unwrap()
    }
}