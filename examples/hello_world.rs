use dust::*;
use window::event::*;

fn main() {

    let mut window = window::Window::new_default("Hello, world!");
    let (width, height) = window.size();

    let renderer = pipeline::ForwardPipeline::create(&window.gl(), width, height).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(0.0, 0.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 10.0);

    let model = crate::Triangle::create(&window.gl());

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    window.render_loop(move |events| {
        for event in events {
            handle_camera_events(&event, &mut camera_handler, &mut camera);
        }
        renderer.render_pass_begin();
        model.render(&camera);
    });
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

pub fn handle_camera_events(event: &Event, camera_handler: &mut dust::camerahandler::CameraHandler, camera: &mut Camera)
{
    match event {
        Event::Key {state, kind} => {
            if kind == "Tab" && *state == State::Pressed
            {
                camera_handler.next_state();
            }
        },
        Event::MouseClick {state, button} => {
            if *button == MouseButton::Left
            {
                if *state == State::Pressed { camera_handler.start_rotation(); }
                else { camera_handler.end_rotation() }
            }
        },
        Event::MouseMotion {delta} => {
            camera_handler.rotate(camera, delta.0 as f32, delta.1 as f32);
        },
        Event::MouseWheel {delta} => {
            camera_handler.zoom(camera, *delta as f32);
        }
    }
}