use crate::context::consts;
use crate::core::*;

/// The basic data type used for each element in a [VertexBuffer].
pub trait VertexBufferDataType:
    Default + std::fmt::Debug + Clone + Copy + internal::BufferDataTypeExtension
{
}
impl VertexBufferDataType for u8 {}
impl VertexBufferDataType for u16 {}
impl VertexBufferDataType for f16 {}
impl VertexBufferDataType for f32 {}

pub trait VertexAttribute<T: VertexBufferDataType>:
    std::fmt::Debug + Clone + vertex_attribute::Extension<T>
{
}

impl<T: VertexBufferDataType> VertexAttribute<T> for T {}
impl<T: VertexBufferDataType> VertexAttribute<T> for Vector2<T> {}
impl<T: VertexBufferDataType> VertexAttribute<T> for Vector3<T> {}
impl<T: VertexBufferDataType> VertexAttribute<T> for Vector4<T> {}
impl VertexAttribute<u8> for Color {}

pub(crate) mod vertex_attribute {
    use crate::core::*;

    pub trait Extension<T: VertexBufferDataType>: Clone {
        fn length() -> u32;
        fn flatten(data: &[Self]) -> Vec<T>;
    }

    impl<T: VertexBufferDataType> Extension<T> for T {
        fn length() -> u32 {
            1
        }

        fn flatten(data: &[Self]) -> Vec<T> {
            data.to_vec()
        }
    }

    impl<T: VertexBufferDataType> Extension<T> for Vector2<T> {
        fn length() -> u32 {
            2
        }

        fn flatten(data: &[Self]) -> Vec<T> {
            let mut res = Vec::with_capacity(data.len() * Self::length() as usize);
            for d in data {
                res.push(d.x);
                res.push(d.y);
            }
            res
        }
    }

    impl<T: VertexBufferDataType> Extension<T> for Vector3<T> {
        fn length() -> u32 {
            3
        }

        fn flatten(data: &[Self]) -> Vec<T> {
            let mut res = Vec::with_capacity(data.len() * Self::length() as usize);
            for d in data {
                res.push(d.x);
                res.push(d.y);
                res.push(d.z);
            }
            res
        }
    }

    impl<T: VertexBufferDataType> Extension<T> for Vector4<T> {
        fn length() -> u32 {
            4
        }

        fn flatten(data: &[Self]) -> Vec<T> {
            let mut res = Vec::with_capacity(data.len() * Self::length() as usize);
            for d in data {
                res.push(d.x);
                res.push(d.y);
                res.push(d.z);
                res.push(d.w);
            }
            res
        }
    }

    impl Extension<u8> for Color {
        fn length() -> u32 {
            4
        }

        fn flatten(data: &[Self]) -> Vec<u8> {
            let mut res = Vec::with_capacity(data.len() * Self::length() as usize);
            for d in data {
                res.push(d.r);
                res.push(d.g);
                res.push(d.b);
                res.push(d.a);
            }
            res
        }
    }
}

///
/// A buffer containing per vertex data, for example positions, normals, uv coordinates or colors.
/// Can send between 1 and 4 values of [InstanceBufferDataType] to a shader program for each vertex.
/// Bind this using the [Program::use_attribute], [Program::use_attribute_vec2], etc. functionality.
///
pub struct VertexBuffer<T: VertexBufferDataType> {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    element_size: u32,
    _dummy: T,
}

impl<T: VertexBufferDataType> VertexBuffer<T> {
    ///
    /// Creates a new empty vertex buffer.
    ///
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        Ok(VertexBuffer {
            context: context.clone(),
            id: context.create_buffer().unwrap(),
            count: 0,
            element_size: 0,
            _dummy: T::default(),
        })
    }

    pub fn new_with<V: VertexAttribute<T>>(context: &Context, data: &[V]) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context)?;
        if data.len() > 0 {
            buffer.fill_with_static(&V::flatten(data));
        }
        buffer.element_size = V::length();
        Ok(buffer)
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_dynamic](VertexBuffer::new_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    pub fn new_with_static(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context)?;
        if data.len() > 0 {
            buffer.fill_with_static(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the vertex buffer with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [fill_with_dynamic](VertexBuffer::fill_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    pub fn fill_with_static(&mut self, data: &[T]) {
        self.bind();
        T::buffer_data(
            &self.context,
            consts::ARRAY_BUFFER,
            data,
            consts::STATIC_DRAW,
        );
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_static](VertexBuffer::new_with_static)
    /// when you expect the data to change often.
    ///
    pub fn new_with_dynamic(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context).unwrap();
        if data.len() > 0 {
            buffer.fill_with_dynamic(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the vertex buffer with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [fill_with_static](VertexBuffer::fill_with_static)
    /// when you expect the data to change often.
    ///
    pub fn fill_with_dynamic(&mut self, data: &[T]) {
        self.bind();
        T::buffer_data(
            &self.context,
            consts::ARRAY_BUFFER,
            data,
            consts::DYNAMIC_DRAW,
        );
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    ///
    /// The number of elements in the buffer.
    ///
    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn element_size(&self) -> u32 {
        self.element_size
    }

    pub(crate) fn bind(&self) {
        self.context.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }
}

impl<T: VertexBufferDataType> Drop for VertexBuffer<T> {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
