use crate::context::{consts, Context};
use crate::core::{Error, VertexBufferDataType};
use crate::{vec3, Vec3};

/// A buffer containing vertex data.
#[derive(Default, Debug)]
pub struct CPUVertexBuffer {
    data: Vec<f32>,
}

impl CPUVertexBuffer {
    pub fn from_xyz(xyz: Vec<f32>) -> CPUVertexBuffer {
        Self::try_from_xyz(xyz).unwrap()
    }

    pub fn try_from_xyz(xyz: Vec<f32>) -> Result<CPUVertexBuffer, Error> {
        if xyz.len() % 3 != 0 {
            return Err(Error::MeshError {
                message: format!("positions len must be divisible by 3: {}", xyz.len()),
            });
        }
        Ok(CPUVertexBuffer { data: xyz })
    }

    pub fn xyz_at(&self, index: usize) -> Vec3 {
        vec3(
            self.data[index * 3],
            self.data[index * 3 + 1],
            self.data[index * 3 + 2],
        )
    }

    pub fn set_xyz_at(&mut self, index: usize, vertex: Vec3) {
        self.data[index * 3] = vertex.x;
        self.data[index * 3 + 1] = vertex.y;
        self.data[index * 3 + 2] = vertex.z;
    }

    pub fn position_count(&self) -> usize {
        self.data.len() / 3
    }

    pub fn extend(&mut self, other: &CPUVertexBuffer) {
        self.data.extend(&other.data);
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn into_data(self) -> Vec<f32> {
        self.data
    }
}

///
/// A buffer containing per vertex data, for example positions, normals, uv coordinates or colors
/// (see also [use_attribute](crate::Program::use_attribute), [use_attribute_vec2](crate::Program::use_attribute_vec2), etc.).
///
pub struct VertexBuffer {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    data_type: u32,
}

impl VertexBuffer {
    ///
    /// Creates a new empty vertex buffer.
    ///
    pub fn new(context: &Context) -> Result<VertexBuffer, Error> {
        Ok(VertexBuffer {
            context: context.clone(),
            id: context.create_buffer().unwrap(),
            count: 0,
            data_type: consts::FLOAT,
        })
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data.
    /// Use this method instead of [new_with_dynamic](crate::VertexBuffer::new_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    pub fn new_with_static<T: VertexBufferDataType>(
        context: &Context,
        data: &[T],
    ) -> Result<VertexBuffer, Error> {
        let mut buffer = Self::new(context)?;
        if data.len() > 0 {
            buffer.fill_with_static(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the vertex buffer with the given data.
    /// Use this method instead of [fill_with_dynamic](crate::VertexBuffer::fill_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    pub fn fill_with_static<T: VertexBufferDataType>(&mut self, data: &[T]) {
        self.bind();
        T::buffer_data(
            &self.context,
            consts::ARRAY_BUFFER,
            data,
            consts::STATIC_DRAW,
        );
        self.data_type = T::data_type();
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data.
    /// Use this method instead of [new_with_static](crate::VertexBuffer::new_with_static)
    /// when you expect the data to change often.
    ///
    pub fn new_with_dynamic<T: VertexBufferDataType>(
        context: &Context,
        data: &[T],
    ) -> Result<VertexBuffer, Error> {
        let mut buffer = Self::new(context).unwrap();
        if data.len() > 0 {
            buffer.fill_with_dynamic(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the vertex buffer with the given data.
    /// Use this method instead of [fill_with_static](crate::VertexBuffer::fill_with_static)
    /// when you expect the data to change often.
    ///
    pub fn fill_with_dynamic<T: VertexBufferDataType>(&mut self, data: &[T]) {
        self.bind();
        T::buffer_data(
            &self.context,
            consts::ARRAY_BUFFER,
            data,
            consts::DYNAMIC_DRAW,
        );
        self.data_type = T::data_type();
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    ///
    /// The number of elements in the buffer.
    ///
    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn data_type(&self) -> u32 {
        self.data_type
    }

    pub(crate) fn bind(&self) {
        self.context.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
