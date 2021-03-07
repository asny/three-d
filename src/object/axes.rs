use crate::math::*;
use crate::core::*;
use crate::camera::*;
use crate::object::*;

pub struct Axes {
    x: Mesh,
    y: Mesh,
    z: Mesh
}

impl Axes {
    pub fn new(context: &Context) -> Result<Self, Error> {
        Ok(Self {
            x: Mesh::new(context, &CPUMesh::arrow(1.0, 2.0, 8, 8))?,
            y: Mesh::new(context, &CPUMesh::arrow(1.0, 2.0, 8, 8))?,
            z: Mesh::new(context, &CPUMesh::arrow(1.0, 2.0, 8, 8))?,
        })
    }

    pub fn render(&self, viewport: Viewport, camera: &camera::Camera) -> Result<(), Error> {
        self.x.render_with_color(&vec4(1.0, 0.0, 0.0, 1.0), RenderStates::default(), viewport, &Mat4::identity(), camera)?;

        Ok(())
    }
}
