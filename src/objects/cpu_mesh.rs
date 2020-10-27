
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

#[cfg(feature = "3d-io")]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CPUMesh {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>
}

#[cfg(feature = "3d-io")]
impl CPUMesh {
    pub fn empty() -> Self
    {
        Self::new(&[], &[], &[]).unwrap()
    }

    pub fn new(indices: &[u32], positions: &[f32], normals: &[f32]) -> Result<Self, objects::Error>
    {
        Ok(CPUMesh {magic_number: 61, version: 1, indices: indices.to_owned(), positions: positions.to_owned(), normals: normals.to_owned(), uvs: Vec::new()})
    }

    pub fn new_with_computed_normals(indices: &[u32], positions: &[f32]) -> Result<Self, objects::Error>
    {
        Self::new(indices, positions, &compute_normals(indices, positions))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<CPUMesh, bincode::Error>
    {
        let decoded: CPUMesh = bincode::deserialize(bytes)?;
        if decoded.magic_number != 61 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom("Corrupt file!".to_string())))?;
        }
        Ok(decoded)
    }

    pub fn from_file(path: &'static str) -> Rc<RefCell<CPUMesh>> {
        let mesh = Rc::new(RefCell::new(Self::empty()));
        Self::from_file_with_mapping(path, mesh.clone(), |mesh, output| {*output.borrow_mut() = mesh;});
        mesh
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_file_with_mapping<T: 'static, F: 'static>(path: &'static str, output: Rc<RefCell<T>>, mapping: F)
        where F: Fn(CPUMesh, Rc<RefCell<T>>)
    {
        let mut file = std::fs::File::open(path).unwrap();
        let mut bytes = Vec::new();
        use std::io::prelude::*;
        file.read_to_end(&mut bytes).unwrap();
        mapping(Self::from_bytes(&bytes).unwrap(), output);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_file_with_mapping<T: 'static, F: 'static>(path: &'static str, output: Rc<RefCell<T>>, mapping: F)
        where F: Fn(CPUMesh, Rc<RefCell<T>>)
    {
        wasm_bindgen_futures::spawn_local(Self::load(path, output, mapping));
    }

    #[cfg(target_arch = "wasm32")]
    async fn load<T: 'static, F: 'static>(url: &'static str, output: Rc<RefCell<T>>, mapping: F)
        where F: Fn(CPUMesh, Rc<RefCell<T>>)
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();
        request.headers().set("Accept", "application/octet-stream").unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let data: JsValue = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
        let bytes: Vec<u8> = js_sys::Uint8Array::new(&data).to_vec();
        mapping(CPUMesh::from_bytes(&bytes).unwrap(), output);
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, objects::Error>
    {
        Ok(bincode::serialize(self)?)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_file(&self, path: &str) -> Result<(), objects::Error>
    {
        let mut file = std::fs::File::create(path)?;
        use std::io::prelude::*;
        file.write_all(&self.to_bytes()?)?;
        Ok(())
    }

    pub fn to_mesh(&self, gl: &crate::Gl) -> Result<Mesh, objects::Error>
    {
        Ok(crate::Mesh::new( &gl, &self.indices, &self.positions, &self.normals)?)
    }
}

fn compute_normals(indices: &[u32], positions: &[f32]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len() * 3];
    for face in 0..indices.len()/3 {
        let index0 = indices[face*3] as usize;
        let p0 = vec3(positions[index0*3], positions[index0*3+1], positions[index0*3+2]);
        let index1 = indices[face*3 + 1] as usize;
        let p1 = vec3(positions[index1*3], positions[index1*3+1], positions[index1*3+2]);
        let index2 = indices[face*3 + 2] as usize;
        let p2 = vec3(positions[index2*3], positions[index2*3+1], positions[index2*3+2]);

        let normal = (p1 - p0).cross(p2 - p0);
        normals[index0*3] += normal.x;
        normals[index0*3+1] += normal.y;
        normals[index0*3+2] += normal.z;
        normals[index1*3] += normal.x;
        normals[index1*3+1] += normal.y;
        normals[index1*3+2] += normal.z;
        normals[index2*3] += normal.x;
        normals[index2*3+1] += normal.y;
        normals[index2*3+2] += normal.z;
    }

    for i in 0..normals.len()/3 {
        let normal = vec3(normals[3*i], normals[3*i+1], normals[3*i+2]).normalize();
        normals[3*i] = normal.x;
        normals[3*i+1] = normal.y;
        normals[3*i+2] = normal.z;
    }
    normals
}