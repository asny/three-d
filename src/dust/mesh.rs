
#[derive(Debug)]
pub enum Error {
}

pub struct Mesh {
    positions: Vec<f32>
}


impl Mesh
{
    pub fn create(positions: Vec<f32>) -> Result<Mesh, Error>
    {
        Ok(Mesh { positions })
    }

    pub fn positions(&self) -> &Vec<f32>
    {
        &self.positions
    }
}
