#[deprecated = "renamed to CpuElementBuffer"]
pub type Indices = CpuElementBuffer;

///
/// An array of indices. Supports different data types.
///
#[derive(Clone)]
pub enum CpuElementBuffer {
    /// Uses unsigned 8 bit integer for each index.
    U8(Vec<u8>),
    /// Uses unsigned 16 bit integer for each index.
    U16(Vec<u16>),
    /// Uses unsigned 32 bit integer for each index.
    U32(Vec<u32>),
}

impl CpuElementBuffer {
    ///
    /// Converts all the indices as `u32` data type.
    ///
    pub fn into_u32(self) -> Vec<u32> {
        match self {
            Self::U8(mut values) => values.drain(..).map(|i| i as u32).collect::<Vec<u32>>(),
            Self::U16(mut values) => values.drain(..).map(|i| i as u32).collect::<Vec<u32>>(),
            Self::U32(values) => values,
        }
    }

    ///
    /// Clones and converts all the indices as `u32` data type.
    ///
    pub fn to_u32(&self) -> Vec<u32> {
        match self {
            Self::U8(values) => values.iter().map(|i| *i as u32).collect::<Vec<u32>>(),
            Self::U16(values) => values.iter().map(|i| *i as u32).collect::<Vec<u32>>(),
            Self::U32(values) => values.clone(),
        }
    }

    ///
    /// Returns the number of indices.
    ///
    pub fn len(&self) -> usize {
        match self {
            Self::U8(values) => values.len(),
            Self::U16(values) => values.len(),
            Self::U32(values) => values.len(),
        }
    }
}

impl std::default::Default for CpuElementBuffer {
    fn default() -> Self {
        Self::U32(Vec::new())
    }
}

impl std::fmt::Debug for CpuElementBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("CpuElementBuffer");
        match self {
            Self::U8(ind) => d.field("u8", &ind.len()),
            Self::U16(ind) => d.field("u16", &ind.len()),
            Self::U32(ind) => d.field("u32", &ind.len()),
        };
        d.finish()
    }
}
