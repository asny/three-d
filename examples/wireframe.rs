extern crate sdl2;
extern crate dust;

mod scene_objects;

use std::process;

use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use dust::*;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    #[cfg(target_os = "macos")] // Use OpenGL 4.1 since that is the newest version supported on macOS
    {
        let gl_attr = video_ctx.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);
    }

    let width: usize = 900;
    let height: usize = 700;
    let window = video_ctx
        .window("Dust", width as u32, height as u32)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_ctx.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Screen
    let screen = screen::Screen {width, height};

    // Renderer
    let renderer = pipeline::DeferredPipeline::create(&gl, &screen, false).unwrap();

    // Camera
    let mut camera = camera::PerspectiveCamera::new(vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0),
                                                    vec3(0.0, 1.0, 0.0),screen.aspect(), 0.1, 1000.0);

    // Objects
    let mut mesh = gust::loader::load_obj_as_dynamic_mesh("../Dust/examples/assets/models/box.obj").unwrap();
    mesh.update_vertex_normals();
    let model = ::objects::ShadedMesh::create(&gl, &mesh);

    let mut wireframe = ::objects::Wireframe::create(&gl, &mesh, 0.015);
    wireframe.set_parameters(0.8, 0.2, 5.0);

    let mut plane = Plane::create(&gl);

    let mut ambient_light = ::light::AmbientLight::new();
    ambient_light.base.intensity = 0.2;

    let mut light1 = dust::light::DirectionalLight::new(vec3(-1.0, -1.0, -1.0));
    light1.enable_shadows(&gl, 4.0, 10.0).unwrap();
    light1.base.intensity = 0.5;

    let mut light2 = dust::light::DirectionalLight::new(vec3(1.0, -1.0, 1.0));
    light2.enable_shadows(&gl, 4.0, 10.0).unwrap();
    light2.base.intensity = 0.5;

    // set up event handling
    let mut events = ctx.event_pump().unwrap();

    // main loop
    let main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::MouseMotion {xrel, yrel, mousestate, .. } => {
                    if mousestate.left()
                    {
                        eventhandler::rotate(&mut camera, xrel, yrel);
                    }
                },
                Event::MouseWheel {y, .. } => {
                    eventhandler::zoom(&mut camera, y);
                },
                _ => {}
            }
        }

        // Draw
        let render_scene = |camera: &Camera| {
            //model.render(&Mat4::identity(), camera);
            wireframe.render(camera);
        };

        // Mirror pass
        plane.mirror_pass_begin(&camera.position(), &vec3(0.0, 0.0, 0.0), &camera.up());
        render_scene(plane.mirror_camera());

        // Shadow pass
        light1.shadow_cast_begin().unwrap();
        render_scene(light1.shadow_camera().unwrap());
        plane.render(&camera);

        light2.shadow_cast_begin().unwrap();
        render_scene(light2.shadow_camera().unwrap());
        plane.render(&camera);

        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        render_scene(&camera);
        plane.render(&camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_directional_light(&light1).unwrap();
        renderer.shine_directional_light(&light2).unwrap();

        window.gl_swap_window();
    };

    renderer::set_main_loop(main_loop);
}

pub struct Plane {
    program: program::Program,
    model: surface::TriangleSurface,
    mirror_rendertarget: ::core::rendertarget::ColorRendertarget,
    mirror_camera: camera::OrthographicCamera,
    pub height: f32,
    pub color: Vec3,
    pub texture: Option<texture::Texture2D>,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl Plane
{
    pub fn create(gl: &gl::Gl) -> Plane
    {
        let mut mesh = mesh_generator::create_plane().unwrap().to_dynamic();
        mesh.scale(10.0);

        let program = program::Program::from_resource(&gl, "../Dust/src/objects/shaders/mesh_shaded",
                                                      "../Dust/examples/assets/shaders/mirror_plane").unwrap();
        let mut model = surface::TriangleSurface::create(gl, &mesh).unwrap();
        model.add_attributes(&mesh, &program, &vec!["position", "normal"]).unwrap();

        // Mirror
        let mirror_rendertarget = ::core::rendertarget::ColorRendertarget::create(&gl, 1024, 1024, 1).unwrap();
        let mut mirror_camera = camera::OrthographicCamera::new(vec3(5.0, 0.0, 5.0), vec3(0.0, 5.0, 0.0),
                                                               vec3(0.0, 1.0, 0.0),10.0, 10.0, 100.0);

        Plane { program, model, mirror_rendertarget, mirror_camera, height: -1.0, color: vec3(1.0, 1.0, 1.0), texture: None,
            diffuse_intensity: 0.1, specular_intensity: 0.3, specular_power: 40.0 }
    }

    pub fn mirror_pass_begin(&mut self, eye: &Vec3, target: &Vec3, up: &Vec3)
    {
        let factor = 0.5 * (eye.y - self.height).abs() / (target.y - self.height).abs();
        let mirror_pos = target * factor + eye * (1.0 - factor);
        self.mirror_camera.set_view(mirror_pos, *target, *up);
        use ::rendertarget::Rendertarget;
        self.mirror_rendertarget.bind();
        self.mirror_rendertarget.clear();
    }

    pub fn mirror_camera(&self) -> &camera::Camera
    {
        &self.mirror_camera
    }

    pub fn render(&self, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);
        self.program.polygon_mode(state::PolygonType::Fill);

        self.mirror_rendertarget.targets[0].bind(0);
        self.program.add_uniform_int("tex", &0).unwrap();
        self.program.add_uniform_vec3("color", &self.color).unwrap();

        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        let transformation = Mat4::new_translation(&vec3(0.0, self.height, 0.0));
        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();
        self.program.add_uniform_mat4("normalMatrix", &transformation.try_inverse().unwrap().transpose()).unwrap();
        self.model.render().unwrap();
    }
}
