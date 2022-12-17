use crate::core::*;
use crate::renderer::*;
use std::collections::HashMap;

///
/// A triangle mesh [Geometry].
///
pub struct Mesh {
    vertex_buffers: HashMap<String, VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    context: Context,
    aabb: AxisAlignedBoundingBox,
    aabb_local: AxisAlignedBoundingBox,
    transformation: Mat4,
    texture_transform: Mat3,
}

impl Mesh {
    ///
    /// Creates a new triangle mesh from the given [CpuMesh].
    /// All data in the [CpuMesh] is transfered to the GPU, so make sure to remove all unnecessary data from the [CpuMesh] before calling this method.
    ///
    pub fn new(context: &Context, cpu_mesh: &CpuMesh) -> Self {
        let aabb = cpu_mesh.compute_aabb();
        Self {
            context: context.clone(),
            index_buffer: super::index_buffer_from_mesh(context, cpu_mesh),
            vertex_buffers: super::vertex_buffers_from_mesh(context, cpu_mesh),
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            texture_transform: Mat3::identity(),
        }
    }

    pub(in crate::renderer) fn set_transformation_2d(&mut self, transformation: Mat3) {
        self.set_transformation(Mat4::new(
            transformation.x.x,
            transformation.x.y,
            0.0,
            transformation.x.z,
            transformation.y.x,
            transformation.y.y,
            0.0,
            transformation.y.z,
            0.0,
            0.0,
            1.0,
            0.0,
            transformation.z.x,
            transformation.z.y,
            0.0,
            transformation.z.z,
        ));
    }

    ///
    /// Returns the local to world transformation applied to this mesh.
    ///
    pub fn transformation(&self) -> Mat4 {
        self.transformation
    }

    ///
    /// Set the local to world transformation applied to this mesh.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        let mut aabb = self.aabb_local.clone();
        aabb.transform(&self.transformation);
        self.aabb = aabb;
    }

    ///
    /// Get the texture transform applied to the uv coordinates of the model.
    ///
    #[deprecated]
    pub fn texture_transform(&mut self) -> &Mat3 {
        &self.texture_transform
    }

    ///
    /// Set the texture transform applied to the uv coordinates of the model.
    ///
    #[deprecated = "Set the texture transformation of Texture2DRef for a material instead"]
    pub fn set_texture_transform(&mut self, texture_transform: Mat3) {
        self.texture_transform = texture_transform;
    }

    fn draw(&self, program: &Program, render_states: RenderStates, camera: &Camera) {
        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform("modelMatrix", &self.transformation);
        program.use_uniform_if_required("textureTransform", &self.texture_transform);
        program.use_uniform_if_required(
            "normalMatrix",
            &self.transformation.invert().unwrap().transpose(),
        );

        for attribute_name in ["position", "normal", "tangent", "color", "uv_coordinates"] {
            if program.requires_attribute(attribute_name) {
                program.use_vertex_attribute(
                    attribute_name,
                    self.vertex_buffers
                        .get(attribute_name).expect(&format!("the render call requires the {} vertex buffer which is missing on the given geometry", attribute_name))
                );
            }
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements(render_states, camera.viewport(), index_buffer)
        } else {
            program.draw_arrays(
                render_states,
                camera.viewport(),
                self.vertex_buffers.get("position").unwrap().vertex_count() as u32,
            )
        }
    }

    fn vertex_shader_source(fragment_shader_source: &str) -> String {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_tangents = fragment_shader_source.find("in vec3 tang;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        let use_colors = fragment_shader_source.find("in vec4 col;").is_some();
        format!(
            "{}{}{}{}{}{}{}",
            if use_positions {
                "#define USE_POSITIONS\n"
            } else {
                ""
            },
            if use_normals {
                "#define USE_NORMALS\n"
            } else {
                ""
            },
            if use_tangents {
                if fragment_shader_source.find("in vec3 bitang;").is_none() {
                    panic!("if the fragment shader defined 'in vec3 tang' it also needs to define 'in vec3 bitang'");
                }
                "#define USE_TANGENTS\n"
            } else {
                ""
            },
            if use_uvs { "#define USE_UVS\n" } else { "" },
            if use_colors {
                "#define USE_COLORS\n#define USE_VERTEX_COLORS\n"
            } else {
                ""
            },
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
        )
    }
}

impl<'a> IntoIterator for &'a Mesh {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Geometry for Mesh {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        let fragment_shader_source =
            material.fragment_shader_source(self.vertex_buffers.contains_key("color"), lights);
        self.context
            .program(
                &Self::vertex_shader_source(&fragment_shader_source),
                &fragment_shader_source,
                |program| {
                    material.use_uniforms(program, camera, lights);
                    self.draw(program, material.render_states(), camera);
                },
            )
            .expect("Failed compiling shader")
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        let fragment_shader_source =
            material.fragment_shader_source(lights, color_texture, depth_texture);
        self.context
            .program(
                &Self::vertex_shader_source(&fragment_shader_source),
                &fragment_shader_source,
                |program| {
                    material.use_uniforms(program, camera, lights, color_texture, depth_texture);
                    self.draw(program, material.render_states(), camera);
                },
            )
            .expect("Failed compiling shader")
    }
}
