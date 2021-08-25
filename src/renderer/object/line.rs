use crate::renderer::*;

#[derive(Clone)]
pub struct Line {
    model: Model2D,
    context: Context,
}

impl Line {
    pub fn new(context: &Context) -> Result<Self> {
        let mut mesh = CPUMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5) * Mat4::from_translation(vec3(1.0, 0.0, 0.0))));
        Ok(Self {
            model: Model2D::new(context, &mesh)?,
            context: context.clone(),
        })
    }

    pub fn set_transformation(&mut self, p0: Vec2, p1: Vec2, width: f32) {
        self.model.set_transformation(
            Mat4::from_translation(vec3(p0.x, p0.y, 0.0))
                * rotation_matrix_from_dir_to_dir(
                    vec3(1.0, 0.0, 0.0),
                    vec3(p1.x - p0.x, p1.y - p0.y, 0.0).normalize(),
                )
                * Mat4::from_nonuniform_scale(p1.distance(p0), width, 1.0),
        );
    }

    pub fn render_with_color(&self, color: Color, viewport: Viewport) -> Result<()> {
        self.model.render_with_color(color, viewport)
    }
}
