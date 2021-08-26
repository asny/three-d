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

    ///
    /// Define the line by the two end points and the width of the line.
    /// Both the pixel coordinates and width must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    ///
    pub fn set_transformation(&mut self, pixel0: (f32, f32), pixel1: (f32, f32), width: f32) {
        let dx = pixel1.0 - pixel0.0;
        let dy = pixel1.1 - pixel0.1;
        let length = (dx * dx + dy * dy).sqrt();
        self.model.set_transformation(
            Mat4::from_translation(vec3(pixel0.0, pixel0.1, 0.0))
                * rotation_matrix_from_dir_to_dir(vec3(1.0, 0.0, 0.0), vec3(dx, dy, 0.0) / length)
                * Mat4::from_nonuniform_scale(length, width, 1.0),
        );
    }
}

impl std::ops::Deref for Line {
    type Target = Model2D;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}
