use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

const VERTICES_PER_SIDE: usize = 33;

#[derive(Clone, Copy, Debug)]
pub struct WaterParameters {
    pub min_wavelength: f32,
    pub max_wavelength: f32,
}

impl Default for WaterParameters {
    fn default() -> Self {
        Self {
            min_wavelength: 0.5,
            max_wavelength: 2.0,
        }
    }
}

///
/// A water geometry with an applied material.
///
pub struct Water<M: Material> {
    patches: Vec<Gm<WaterPatch, M>>,
}
impl<M: Material + Clone> Water<M> {
    ///
    /// Constructs a new [Water] object.
    ///
    pub fn new(
        context: &Context,
        material: M,
        side_length: f32,
        vertex_distance: f32,
        center: Vec2,
        parameters: WaterParameters,
    ) -> Self {
        let patch_size = vertex_distance * (VERTICES_PER_SIDE - 1) as f32;
        let patches_per_side = ((side_length / patch_size).ceil() as u32).max(1);
        let half_side_length = 0.5 * patches_per_side as f32 * patch_size;
        let index_buffer = Self::indices(context);
        let position_buffer = Self::positions(context, vertex_distance);
        let mut patches = Vec::new();
        for ix in 0..patches_per_side {
            for iy in 0..patches_per_side {
                let offset = vec2(
                    (ix as f32) * patch_size - half_side_length,
                    (iy as f32) * patch_size - half_side_length,
                );
                let patch = WaterPatch::new(
                    context,
                    center,
                    parameters,
                    offset,
                    vec2(patch_size, patch_size),
                    position_buffer.clone(),
                    index_buffer.clone(),
                );
                patches.push(Gm::new(patch, material.clone()));
            }
        }

        Self { patches }
    }

    ///
    /// Set the center of the water.
    /// To be able to move the water with the camera, thereby simulating infinite water.
    ///
    pub fn set_center(&mut self, center: Vec2) {
        self.patches.iter_mut().for_each(|m| m.center = center);
    }

    pub fn parameters(&self) -> WaterParameters {
        self.patches[0].parameters
    }

    pub fn set_parameters(&mut self, parameters: WaterParameters) {
        self.patches
            .iter_mut()
            .for_each(|p| p.parameters = parameters);
    }

    ///
    /// For updating the animation. The time parameter should be some continious time, for example the time since start.
    ///
    pub fn update_animation(&mut self, time: f64) {
        self.patches.iter_mut().for_each(|m| m.time = time);
    }

    ///
    /// Returns an iterator over the reference to the objects which can be used as input to a render function, for example [RenderTarget::render].
    ///
    pub fn obj_iter(&self) -> impl Iterator<Item = &dyn Object> + Clone {
        self.patches.iter().map(|m| m as &dyn Object)
    }

    ///
    /// Returns an iterator over the reference to the geometries which can be used as input to for example [pick], [RenderTarget::render_with_material] or [DirectionalLight::generate_shadow_map].
    ///
    pub fn geo_iter(&self) -> impl Iterator<Item = &dyn Geometry> + Clone {
        self.patches.iter().map(|m| m as &dyn Geometry)
    }

    fn indices(context: &Context) -> Rc<ElementBuffer> {
        let mut indices: Vec<u32> = Vec::new();
        let stride = VERTICES_PER_SIDE as u32;
        let max = stride - 1;
        for r in 0..max {
            for c in 0..max {
                indices.push(r + c * stride);
                indices.push(r + 1 + c * stride);
                indices.push(r + (c + 1) * stride);
                indices.push(r + (c + 1) * stride);
                indices.push(r + 1 + c * stride);
                indices.push(r + 1 + (c + 1) * stride);
            }
        }
        Rc::new(ElementBuffer::new_with_data(context, &indices))
    }

    fn positions(context: &Context, vertex_distance: f32) -> Rc<VertexBuffer> {
        let mut data = vec![vec3(0.0, 0.0, 0.0); (VERTICES_PER_SIDE * VERTICES_PER_SIDE) as usize];
        for r in 0..VERTICES_PER_SIDE {
            for c in 0..VERTICES_PER_SIDE {
                let vertex_id = r * VERTICES_PER_SIDE + c;
                let x = r as f32 * vertex_distance;
                let z = c as f32 * vertex_distance;
                data[vertex_id as usize] = vec3(x, 0.0, z);
            }
        }
        Rc::new(VertexBuffer::new_with_data(context, &data))
    }
}

struct WaterPatch {
    context: Context,
    time: f64,
    center: Vec2,
    parameters: WaterParameters,
    offset: Vec2,
    size: Vec2,
    position_buffer: Rc<VertexBuffer>,
    index_buffer: Rc<ElementBuffer>,
}

impl WaterPatch {
    pub fn new(
        context: &Context,
        center: Vec2,
        parameters: WaterParameters,
        offset: Vec2,
        size: Vec2,
        position_buffer: Rc<VertexBuffer>,
        index_buffer: Rc<ElementBuffer>,
    ) -> Self {
        Self {
            context: context.clone(),
            time: 0.0,
            center,
            parameters,
            offset,
            size,
            position_buffer,
            index_buffer,
        }
    }
}

impl Geometry for WaterPatch {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        let fragment_shader_source = material.fragment_shader_source(false, lights);
        self.context
            .program(
                &include_str!("shaders/water.vert"),
                &fragment_shader_source,
                |program| {
                    material.use_uniforms(program, camera, lights);
                    let transformation = Mat4::from_translation(vec3(
                        self.center.x + self.offset.x,
                        0.0,
                        self.center.y + self.offset.y,
                    ));
                    program.use_uniform("modelMatrix", &transformation);
                    program.use_uniform("projectionMatrix", camera.projection());
                    program.use_uniform("viewMatrix", camera.view());
                    program.use_uniform("time", &(self.time as f32 * 0.001));
                    program.use_uniform("minWavelength", &self.parameters.min_wavelength);
                    program.use_uniform("maxWavelength", &self.parameters.max_wavelength);
                    let render_states = RenderStates {
                        blend: Blend::TRANSPARENCY,
                        ..Default::default()
                    };

                    program.use_vertex_attribute("position", &self.position_buffer);

                    program.draw_elements(render_states, camera.viewport(), &self.index_buffer);
                },
            )
            .unwrap();
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_with_positions(&[
            vec3(
                self.center.x + self.offset.x,
                0.0,
                self.center.y + self.offset.y,
            ),
            vec3(
                self.center.x + self.offset.x + self.size.x,
                0.0,
                self.center.y + self.offset.y + self.size.y,
            ),
        ])
    }
}
