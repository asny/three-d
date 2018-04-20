use glm;

#[derive(Debug)]
pub enum Error {

}

pub struct Attribute {
    name: String,
    data: Vec<f32>,
    stride: usize
}


impl Attribute
{
    pub fn create_vec3_attribute(name: &str, data: Vec<glm::Vec3>) -> Result<Attribute, Error>
    {
        let mut d = Vec::new();
        for datum in data {
            d.push(datum.x);
            d.push(datum.y);
            d.push(datum.z);
        }
        Ok(Attribute{name: String::from(name), data: d, stride: 3})
    }

    pub fn data(&self) -> &Vec<f32>
    {
        &self.data
    }

    pub fn stride(&self) -> usize
    {
        self.stride
    }

    pub fn name(&self) -> &str
    {
        &self.name
    }
}
