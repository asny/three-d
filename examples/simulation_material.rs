use dust::program;
use gl;
use dust::input;
use dust::material;
use dust::texture;
use gust::mesh;
use std::rc::Rc;

pub struct SimulationMaterial {
    program: program::Program,
    texture_width: usize,
    index_to_position_texture: texture::Texture
}

impl material::Material for SimulationMaterial
{
    fn apply(&self)
    {
        self.program.set_used();
    }

    fn setup_states(&self, _gl: &gl::Gl) -> Result<(), material::Error> {
        Ok(())
    }

    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), material::Error>
    {
        self.program.add_uniform_float("textureSpacing", &(1.0 / (self.texture_width as f32)))?;
        self.index_to_position_texture.bind_at(0);
        self.program.add_uniform_int("indexToPosition", &0)?;
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        Ok(())
    }

    fn setup_attributes(&self, mesh: &mesh::Mesh) -> Result<(), material::Error>
    {
        self.program.add_attribute(&mesh.positions())?;
        Ok(())
    }
}

impl SimulationMaterial
{
    pub fn create(gl: &gl::Gl, positions: &Vec<f32>, faces: &Vec<u32>, owners: &Vec<u32>, neighbours: &Vec<u32>) -> Result<Rc<material::Material>, material::Error>
    {
        let shader_program = program::Program::from_resource(&gl, "examples/assets/shaders/simulation")?;
        let texture_width = 1024;

        let mut index_to_position_texture = texture::Texture::create(&gl).unwrap();
        index_to_position_texture.fill_with(positions, texture_width, texture_width, 3);

        let mut faceid_to_indices_texture = texture::Texture::create(&gl).unwrap();
        faceid_to_indices_texture.fill_with_int(faces, texture_width, texture_width);


        let no_cells = *owners.iter().max().unwrap() as usize + 1;
        println!("{}", no_cells);
        let mut cellid_to_faceids = Vec::new();
        use std::iter;
        cellid_to_faceids.extend(iter::repeat(0).take(4 * no_cells));

        let mut cell_count = Vec::new();
        cell_count.extend(iter::repeat(0).take(no_cells));

        for face_id in 0..owners.len() {
            let cell_id = owners[face_id] as usize;
            cellid_to_faceids[ cell_id * 4 + cell_count[cell_id] ] = face_id;
            cell_count[cell_id] = cell_count[cell_id] + 1;
        }

        for face_id in 0..neighbours.len() {
            let cell_id = neighbours[face_id] as usize;
            cellid_to_faceids[ cell_id * 4 + cell_count[cell_id] ] = face_id;
            cell_count[cell_id] = cell_count[cell_id] + 1;
        }
        println!("{:?}", cellid_to_faceids);

        let mut cellid_to_faceids_texture = texture::Texture::create(&gl).unwrap();
        cellid_to_faceids_texture.fill_with_int(faces, texture_width, texture_width);

        Ok(Rc::new(SimulationMaterial { program: shader_program, texture_width, index_to_position_texture }))
    }
}
