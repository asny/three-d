
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use dust::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let gl = gl::Gl::new(context);

    let renderer = pipeline::ForwardPipeline::create(&gl, canvas.width() as usize, canvas.height() as usize).unwrap();

    // Camera
    let camera = camera::PerspectiveCamera::new(vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), canvas.width() as f32 / canvas.height() as f32, 0.1, 10.0);

    let model = crate::Triangle::create(&gl);

    // main loop
    //loop {
        // draw
        renderer.render_pass_begin();

        model.render(&camera);

        //window_handler.swap_buffers();
    //};

    Ok(())
}

pub struct Triangle {
    program: program::Program,
    model: surface::TriangleSurface
}

impl Triangle
{
    pub fn create(gl: &gl::Gl) -> Triangle
    {
        let indices: Vec<u32> = (0..3).collect();
        let positions: Vec<f32> = vec![
            0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0,// bottom left
            0.0,  0.5, 0.0 // top
        ];
        let colors: Vec<f32> = vec![
            1.0, 0.0, 0.0,   // bottom right
            0.0, 1.0, 0.0,   // bottom left
            0.0, 0.0, 1.0    // top
        ];
        let program = Triangle::create_program(gl);
        //let program = program::Program::from_resource(&gl, "examples/assets/shaders/color", "examples/assets/shaders/color").unwrap();
        let mut model = surface::TriangleSurface::create(gl, &indices).unwrap();
        model.add_attributes(&program, &att!["position" => (positions, 3), "color" => (colors, 3)]).unwrap();

        Triangle { program, model }
    }

    fn create_program(gl: &gl::Gl) -> program::Program
    {
        program::Program::from_source(gl, r#"
        uniform mat4 viewMatrix;
        uniform mat4 projectionMatrix;

        in vec3 position;
        in vec3 color;

        out vec3 col;

        void main()
        {
          col = color;
          gl_Position = projectionMatrix * viewMatrix * vec4(position, 1.0);
        }
        "#,
        r#"
            in vec3 col;

            out vec4 fragmentColor;

            void main()
            {
                fragmentColor = vec4(col, 1.0f);
            }

        "#).unwrap()
    }

    pub fn render(&self, camera: &camera::Camera)
    {
        self.program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();
        self.model.render().unwrap();
    }
}