
use crate::core::*;

pub struct DirectionalLight {
    context: Context,
    light_buffer: UniformBuffer,
    shadow_texture: Texture2D,
    shadow_camera: Option<Camera>
}

impl DirectionalLight {

    pub fn new(context: &Context, intensity: f32, color: &Vec3, direction: &Vec3) -> Result<DirectionalLight, Error>
    {
        let mut light = DirectionalLight {
            context: context.clone(),
            light_buffer: UniformBuffer::new(context, &[3u32, 1, 3, 1, 16])?,
            shadow_texture: Texture2D::new(context, 1, 1, Interpolation::Nearest, Interpolation::Nearest, None,Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::Depth32F)?,
            shadow_camera: None};

        light.set_intensity(intensity);
        light.set_color(color);
        light.set_direction(direction);
        Ok(light)
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.light_buffer.update(0, &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32)
    {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn set_direction(&mut self, direction: &Vec3)
    {
        self.light_buffer.update(2, &direction.normalize().to_slice()).unwrap();
    }

    pub fn direction(&self) -> Vec3 {
        let d = self.light_buffer.get(2).unwrap();
        vec3(d[0], d[1], d[2])
    }

    pub fn clear_shadow_map(&mut self)
    {
        self.shadow_camera = None;
        self.shadow_texture = Texture2D::new(&self.context, 1, 1, Interpolation::Nearest, Interpolation::Nearest, None,Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::Depth32F).unwrap();
        self.light_buffer.update(3, &[0.0]).unwrap();
    }

    pub fn generate_shadow_map<F: FnOnce(Viewport, &Camera) -> Result<(), Error>>(&mut self, target: &Vec3,
                                  frustrum_width: f32, frustrum_height: f32, frustrum_depth: f32,
                                  texture_width: usize, texture_height: usize, render_scene: F) -> Result<(), Error>
    {
        let direction = self.direction();
        let up = compute_up_direction(direction);

        self.shadow_camera = Some(Camera::new_orthographic(&self.context, target - direction.normalize()*0.5*frustrum_depth, *target, up,
                                                           frustrum_width, frustrum_height, frustrum_depth));
        self.light_buffer.update(4, &shadow_matrix(self.shadow_camera.as_ref().unwrap()).to_slice())?;

        self.shadow_texture = Texture2D::new(&self.context, texture_width, texture_height,
                                                        Interpolation::Nearest, Interpolation::Nearest, None, // Linear filtering is not working on web
                                                        Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::Depth32F).unwrap();
        RenderTarget::new_depth(&self.context,&self.shadow_texture)?
            .write_depth(Some(1.0),
            || {
                render_scene(Viewport::new_at_origo(texture_width, texture_height), self.shadow_camera.as_ref().unwrap())?;
                Ok(())
            })?;
        self.light_buffer.update(3, &[1.0])?;
        Ok(())
    }

    pub fn shadow_map(&self) -> &Texture2D
    {
        &self.shadow_texture
    }

    pub fn buffer(&self) -> &UniformBuffer
    {
        &self.light_buffer
    }
}

fn shadow_matrix(camera: &Camera) -> Mat4
{
    let bias_matrix = crate::Mat4::new(
                         0.5, 0.0, 0.0, 0.0,
                         0.0, 0.5, 0.0, 0.0,
                         0.0, 0.0, 0.5, 0.0,
                         0.5, 0.5, 0.5, 1.0);
    bias_matrix * camera.get_projection() * camera.get_view()
}

fn compute_up_direction(direction: Vec3) -> Vec3
{
    if vec3(1.0, 0.0, 0.0).dot(direction).abs() > 0.9
    {
        (vec3(0.0, 1.0, 0.0).cross(direction)).normalize()
    }
    else {
        (vec3(1.0, 0.0, 0.0).cross(direction)).normalize()
    }
}