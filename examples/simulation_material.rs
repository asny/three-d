use dust::program;
use gl;
use dust::input;
use dust::material;
use dust::texture;
use gust::mesh;
use std::rc::Rc;
use dust::state;

pub struct SimulationMaterial {
    program: program::Program,
    texture_width: usize,
    index_to_position_texture: texture::Texture,
    faceid_to_indices_texture: texture::Texture,
    cellid_to_faceids_texture: texture::Texture,
    faceid_to_cellids_texture: texture::Texture
}

impl material::Material for SimulationMaterial
{
    fn apply(&self)
    {
        self.program.set_used();
    }

    fn setup_states(&self, gl: &gl::Gl) -> Result<(), material::Error> {
        state::cull_back_faces(gl, true);
        Ok(())
    }

    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), material::Error>
    {
        self.program.add_uniform_float("textureSpacing", &(1.0 / (self.texture_width as f32)))?;
        self.program.add_uniform_float("density", &3.0)?;

        self.index_to_position_texture.bind_at(0);
        self.program.add_uniform_int("indexToPosition", &0)?;
        self.faceid_to_indices_texture.bind_at(1);
        self.program.add_uniform_int("faceIdToIndices", &1)?;
        self.cellid_to_faceids_texture.bind_at(2);
        self.program.add_uniform_int("cellIdToFaceIds", &2)?;
        self.faceid_to_cellids_texture.bind_at(3);
        self.program.add_uniform_int("faceIdToCellIds", &3)?;

        self.program.add_uniform_vec3("cameraPosition", &input.camera_position)?;
        self.program.add_uniform_mat4("modelMatrix", &input.model)?;
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        Ok(())
    }

    fn setup_attributes(&self, mesh: &mesh::Mesh) -> Result<(), material::Error>
    {
        let mut list = Vec::new();
        list.push( mesh.positions());
        list.push(mesh.get("FaceId")?);
        self.program.add_attributes(&list)?;
        Ok(())
    }
}

impl SimulationMaterial
{
    pub fn create(gl: &gl::Gl, positions: &Vec<f32>, faces: &Vec<u32>, cell_to_faces: &Vec<u32>, face_to_cells: &Vec<u32>) -> Result<Rc<material::Material>, material::Error>
    {
        let shader_program = program::Program::from_resource(&gl, "examples/assets/shaders/simulation")?;
        let texture_width = 1024;

        let mut index_to_position_texture = texture::Texture::create(&gl).unwrap();
        index_to_position_texture.fill_with(positions, texture_width, texture_width, 3);

        let mut faceid_to_indices_texture = texture::Texture::create(&gl).unwrap();
        faceid_to_indices_texture.fill_with_int(faces, texture_width, texture_width);

        let mut cellid_to_faceids_texture = texture::Texture::create(&gl).unwrap();
        cellid_to_faceids_texture.fill_with_int(cell_to_faces, texture_width, texture_width);

        let mut faceid_to_cellids_texture = texture::Texture::create(&gl).unwrap();
        faceid_to_cellids_texture.fill_with_int(face_to_cells, texture_width, texture_width);

        Ok(Rc::new(SimulationMaterial {
            program: shader_program, texture_width, index_to_position_texture, faceid_to_indices_texture, cellid_to_faceids_texture, faceid_to_cellids_texture
        }))
    }
}
