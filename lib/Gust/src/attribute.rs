use glm;

#[derive(Debug)]
pub enum Error {

}

pub struct Attribute {
    name: String,
    data: Vec<f32>,
    no_components: usize
}


impl Attribute
{
    pub fn create_int_attribute(name: &str, data: &Vec<u32>) -> Result<Attribute, Error>
    {
        let d = data.iter().map(|i| *i as f32).collect();
        Ok(Attribute{name: String::from(name), data: d, no_components: 1})
    }

    pub fn create_vec3_attribute(name: &str, data: Vec<glm::Vec3>) -> Result<Attribute, Error>
    {
        let mut d = Vec::new();
        for datum in data {
            d.push(datum.x);
            d.push(datum.y);
            d.push(datum.z);
        }
        Ok(Attribute{name: String::from(name), data: d, no_components: 3})
    }

    pub fn data(&self) -> &Vec<f32>
    {
        &self.data
    }

    pub fn no_components(&self) -> usize
    {
        self.no_components
    }

    pub fn name(&self) -> &str
    {
        &self.name
    }
}
