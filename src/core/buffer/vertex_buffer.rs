use crate::context::consts;
use crate::core::*;

pub trait Attribute<T: BufferDataType>: std::fmt::Debug + Clone + attribute::Extension<T> {}

impl<T: BufferDataType> Attribute<T> for T {}
impl<T: BufferDataType> Attribute<T> for Vector2<T> {}
impl<T: BufferDataType> Attribute<T> for Vector3<T> {}
impl<T: BufferDataType> Attribute<T> for Vector4<T> {}
impl Attribute<u8> for Color {}

pub(crate) mod attribute {
    use crate::core::*;

    pub trait Extension<T: BufferDataType>: Clone {
        fn length() -> u32;
        fn flatten(data: &[Self]) -> Vec<T>;
    }

    impl<T: BufferDataType> Extension<T> for T {
        fn length() -> u32 {
            1
        }

        fn flatten(data: &[Self]) -> Vec<T> {
            data.to_vec()
        }
    }

    impl<T: BufferDataType> Extension<T> for Vector2<T> {
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

    impl<T: BufferDataType> Extension<T> for Vector3<T> {
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

    impl<T: BufferDataType> Extension<T> for Vector4<T> {
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

pub enum VertexBufferType {
    Static,
    Dynamic,
}

///
/// A buffer containing per vertex data, for example positions, normals, uv coordinates or colors.
/// Can send between 1 and 4 values of [InstanceBufferDataType] to a shader program for each vertex.
/// Bind this using the [Program::use_attribute], [Program::use_attribute_vec2], etc. functionality.
///
pub struct VertexBuffer<T: BufferDataType> {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    attribute_count: u32,
    attribute_size: u32,
    buffer_type: VertexBufferType,
    _dummy: T,
}

impl<T: BufferDataType> VertexBuffer<T> {
    ///
    /// Creates a new empty vertex buffer.
    ///
    pub fn new(context: &Context, buffer_type: VertexBufferType) -> ThreeDResult<Self> {
        Ok(VertexBuffer {
            context: context.clone(),
            id: context.create_buffer().unwrap(),
            count: 0,
            attribute_count: 0,
            attribute_size: 0,
            buffer_type,
            _dummy: T::default(),
        })
    }

    pub fn new_with_data<V: Attribute<T>>(
        context: &Context,
        buffer_type: VertexBufferType,
        data: &[V],
    ) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context, buffer_type)?;
        if data.len() > 0 {
            buffer.fill(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the vertex buffer with the given data.
    ///
    pub fn fill<V: Attribute<T>>(&mut self, data: &[V]) {
        self.bind();
        T::buffer_data(
            &self.context,
            consts::ARRAY_BUFFER,
            &V::flatten(data),
            match self.buffer_type {
                VertexBufferType::Static => consts::STATIC_DRAW,
                VertexBufferType::Dynamic => consts::DYNAMIC_DRAW,
            },
        );
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len() * V::length() as usize;
        self.attribute_count = data.len() as u32;
        self.attribute_size = V::length();
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_dynamic](VertexBuffer::new_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    #[deprecated = "use new() or new_with_data()"]
    pub fn new_with_static<V: Attribute<T>>(context: &Context, data: &[V]) -> ThreeDResult<Self> {
        Self::new_with_data(context, VertexBufferType::Static, data)
    }

    ///
    /// Fills the vertex buffer with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [fill_with_dynamic](VertexBuffer::fill_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    #[deprecated = "use fill()"]
    pub fn fill_with_static<V: Attribute<T>>(&mut self, data: &[V]) {
        self.fill(data)
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_static](VertexBuffer::new_with_static)
    /// when you expect the data to change often.
    ///
    #[deprecated = "use new() or new_with_data()"]
    pub fn new_with_dynamic<V: Attribute<T>>(context: &Context, data: &[V]) -> ThreeDResult<Self> {
        Self::new_with_data(context, VertexBufferType::Dynamic, data)
    }

    ///
    /// Fills the vertex buffer with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [fill_with_static](VertexBuffer::fill_with_static)
    /// when you expect the data to change often.
    ///
    #[deprecated = "use fill()"]
    pub fn fill_with_dynamic<V: Attribute<T>>(&mut self, data: &[V]) {
        self.fill(data)
    }

    ///
    /// The number of values in the buffer.
    ///
    pub fn count(&self) -> usize {
        self.count
    }

    ///
    /// The number of vertex attributes in the buffer.
    ///
    pub fn attribute_count(&self) -> u32 {
        self.attribute_count
    }

    ///
    /// The size of each vertex attributes, for example 3 if the vertex attribute is a [Vector3].
    ///
    pub fn attribute_size(&self) -> u32 {
        self.attribute_size
    }

    pub(crate) fn bind(&self) {
        self.context.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }
}

impl<T: BufferDataType> Drop for VertexBuffer<T> {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
