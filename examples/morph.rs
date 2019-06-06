
use tri_mesh::prelude::*;
use tri_mesh::prelude::Vec3 as Vec3;
use tri_mesh::prelude::vec3 as vec3;
use tri_mesh::prelude::vec4 as vec4;
use std::collections::HashMap;

/// Loads the mesh and scale/translate it.
fn on_startup(scene_center: &Vec3, scene_radius: f64) -> Mesh
{
    let mut mesh = MeshBuilder::new().with_obj(include_str!("assets/models/suzanne.obj").to_string()).build().unwrap();
    let (min, max) = mesh.extreme_coordinates();
    mesh.translate(-0.5 * (max + min)); // Translate such that the mesh center is in origo.
    let size = max - min;
    mesh.scale(0.5 * scene_radius / size.x.max(size.y).max(size.z)); // Scale the mesh such that the size of the biggest side of the bounding box is half a scene radius
    mesh.translate(*scene_center); // Translate the mesh to the scene center
    mesh
}

/// When the user clicks, we see if the model is hit. If it is, we compute the morph weights from the picking point.
fn on_click(mesh: &Mesh, ray_start_point: &Vec3, ray_direction: &Vec3) -> Option<HashMap<VertexID, Vec3>>
{
    if let Some((vertex_id, point)) = pick(&mesh,&ray_start_point, &ray_direction) {
        Some(compute_weights(mesh, vertex_id, &point))
    }
    else {None}
}

/// Morphs the vertices based on the computed weights.
fn on_morph(mesh: &mut Mesh, weights: &HashMap<VertexID, Vec3>, factor: f64)
{
    for (vertex_id, weight) in weights.iter() {
        mesh.move_vertex_by(*vertex_id,weight * factor);
    }
}

/// Picking used for determining whether a mouse click starts a morph operation. Returns a close vertex and the position of the click on the mesh surface.
fn pick(mesh: &Mesh, ray_start_point: &Vec3, ray_direction: &Vec3) -> Option<(VertexID, Vec3)>
{
    if let Some(Intersection::Point {primitive, point}) = mesh.ray_intersection(ray_start_point, ray_direction) {
        let start_vertex_id = match primitive {
            Primitive::Face(face_id) => {
                mesh.walker_from_face(face_id).vertex_id().unwrap()
            },
            Primitive::Edge(halfedge_id) => {
                let (vertex_id, ..) = mesh.edge_vertices(halfedge_id);
                vertex_id
            },
            Primitive::Vertex(vertex_id) => {
                vertex_id
            }
        };
        Some((start_vertex_id, point))
    }
    else {None}
}

/// Compute a directional weight for each vertex to be used for the morph operation.
fn compute_weights(mesh: &Mesh, start_vertex_id: VertexID, start_point: &Vec3) -> HashMap<VertexID, Vec3>
{
    static SQR_MAX_DISTANCE: f64 = 1.0;

    // Use the smoothstep function to get a smooth morphing
    let smoothstep_function = |sqr_distance| {
        let x = sqr_distance / SQR_MAX_DISTANCE;
        1.0 - x*x*(3.0 - 2.0 * x)
    };

    // Visit all the vertices close to the start vertex.
    let mut weights = HashMap::new();
    let mut to_be_tested = vec![start_vertex_id];
    while let Some(vertex_id) = to_be_tested.pop()
    {
        let sqr_distance = start_point.distance2(*mesh.vertex_position(vertex_id));
        if sqr_distance < SQR_MAX_DISTANCE
        {
            // The weight is computed as the smoothstep function to the square euclidean distance
            // to the start point on the surface multiplied by the vertex normal.
            weights.insert(vertex_id, smoothstep_function(sqr_distance) * mesh.vertex_normal(vertex_id));

            // Add neighbouring vertices to be tested if they have not been visited yet
            for halfedge_id in mesh.vertex_halfedge_iter(vertex_id)
            {
                let neighbour_vertex_id = mesh.walker_from_halfedge(halfedge_id).vertex_id().unwrap();
                if !weights.contains_key(&neighbour_vertex_id) {
                    to_be_tested.push(neighbour_vertex_id);
                }
            }
        }
    }
    weights
}

///
/// Above: Everything related to tri-mesh
/// Below: Visualisation of the mesh, event handling and so on
///
use dust::*;
use dust::objects::*;
use dust::window::{event::*, Window};

fn main()
{
    let scene_radius = 10.0;
    let scene_center = dust::vec3(0.0, 5.0, 0.0);
    let mut mesh = on_startup(&vec3(scene_center.x as f64, scene_center.y as f64, scene_center.z as f64), scene_radius as f64);
    let positions: Vec<f32> = mesh.positions_buffer().iter().map(|v| *v as f32).collect();
    let normals: Vec<f32> = mesh.normals_buffer().iter().map(|v| *v as f32).collect();

    let mut window = Window::new_default("Morph tool").unwrap();
    let (width, height) = window.framebuffer_size();
    let window_size = window.size();
    let gl = window.gl();

    // Renderer
    let renderer = DeferredPipeline::new(&gl, width, height, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();

    // Camera
    let mut camera = Camera::new_perspective(&gl, scene_center + scene_radius * vec3(1.0, 1.0, 1.0).normalize(), scene_center,
                                                    vec3(0.0, 1.0, 0.0),degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    // Objects
    let mut wireframe_model = Wireframe::new(&gl, &mesh.indices_buffer(), &positions, 0.02);
    wireframe_model.set_parameters(0.8, 0.2, 5.0);
    wireframe_model.set_color(&vec3(0.9, 0.2, 0.2));

    let mut model = ShadedMesh::new(&gl, &mesh.indices_buffer(), &positions, &normals).unwrap();
    model.color = vec3(0.8, 0.8, 0.8);

    let plane_positions: Vec<f32> = vec![
        -1.0, 0.0, -1.0,
        1.0, 0.0, -1.0,
        1.0, 0.0, 1.0,
        -1.0, 0.0, 1.0
    ];
    let plane_normals: Vec<f32> = vec![
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0
    ];
    let plane_indices: Vec<u32> = vec![
        0, 2, 1,
        0, 3, 2,
    ];
    let mut plane = ShadedMesh::new(&gl, &plane_indices, &plane_positions, &plane_normals).unwrap();
    plane.diffuse_intensity = 0.2;
    plane.specular_intensity = 0.4;
    plane.specular_power = 20.0;

    let mut ambient_light = light::AmbientLight::new();
    ambient_light.base.intensity = 0.4;

    let mut dir = vec3(-1.0, -1.0, -1.0).normalize();
    let mut light1 = light::SpotLight::new(scene_center - 2.0 * scene_radius * dir, dir);
    light1.enable_shadows(&gl, scene_radius * 4.0).unwrap();
    light1.base.intensity = 0.75;

    dir = vec3(-1.0, -1.0, 1.0).normalize();
    let mut light2 = light::SpotLight::new(scene_center - 2.0 * scene_radius * dir, dir);
    light2.enable_shadows(&gl, scene_radius * 4.0).unwrap();
    light2.base.intensity = 0.75;

    dir = vec3(1.0, -1.0, 1.0).normalize();
    let mut light3 = light::SpotLight::new(scene_center - 2.0 * scene_radius * dir, dir);
    light3.enable_shadows(&gl, scene_radius * 4.0).unwrap();
    light3.base.intensity = 0.75;

    dir = vec3(1.0, -1.0, -1.0).normalize();
    let mut light4 = light::SpotLight::new(scene_center - 2.0 * scene_radius * dir, dir);
    light4.enable_shadows(&gl, scene_radius * 4.0).unwrap();
    light4.base.intensity = 0.75;

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    let mut weights: Option<HashMap<VertexID, Vec3>> = None;
    // main loop
    window.render_loop(move |events, _elapsed_time|
    {
        for event in events {
            match event {
                Event::Key {state, kind} => {
                    if kind == "Tab" && *state == State::Pressed
                    {
                        camera_handler.next_state();
                    }
                },
                Event::MouseClick {state, button, position} => {
                    if *button == MouseButton::Left
                    {
                        if *state == State::Pressed
                        {
                            let (x, y) = (position.0 / window_size.0 as f64, position.1 / window_size.1 as f64);
                            let p = camera.position();
                            let dir = camera.view_direction_at((x, y));
                            weights = on_click(&mesh,&vec3(p.x as f64, p.y as f64, p.z as f64), &vec3(dir.x as f64, dir.y as f64, dir.z as f64));
                            if weights.is_none() {
                                camera_handler.start_rotation();
                            }
                        }
                        else {
                            weights = None;
                            camera_handler.end_rotation()
                        }
                    }
                },
                Event::MouseWheel {delta} => {
                    camera_handler.zoom(&mut camera, *delta as f32);
                },
                Event::MouseMotion {delta} => {
                    camera_handler.rotate(&mut camera, delta.0 as f32, delta.1 as f32);
                    if let Some(ref w) = weights
                    {
                        on_morph(&mut mesh, w, 0.001 * delta.1);
                        let positions: Vec<f32> = mesh.positions_buffer().iter().map(|v| *v as f32).collect();
                        let normals: Vec<f32> = mesh.normals_buffer().iter().map(|v| *v as f32).collect();
                        wireframe_model.update_positions(&positions);
                        model.update_attributes(&positions, &normals).unwrap();
                    }
                }
            }
        }

        // Draw
        let render_scene = |camera: &Camera| {
            let model_matrix = dust::Mat4::identity();
            model.render(&model_matrix, camera);
            wireframe_model.render(camera);
        };

        // Shadow pass
        light1.shadow_cast_begin().unwrap();
        render_scene(light1.shadow_camera().unwrap());

        light2.shadow_cast_begin().unwrap();
        render_scene(light2.shadow_camera().unwrap());

        light3.shadow_cast_begin().unwrap();
        render_scene(light3.shadow_camera().unwrap());

        light4.shadow_cast_begin().unwrap();
        render_scene(light4.shadow_camera().unwrap());

        // Geometry pass
        renderer.geometry_pass_begin().unwrap();
        render_scene(&camera);
        plane.render(&dust::Mat4::from_scale(100.0), &camera);

        // Light pass
        renderer.light_pass_begin(&camera).unwrap();
        renderer.shine_ambient_light(&ambient_light).unwrap();
        renderer.shine_spot_light(&light1).unwrap();
        renderer.shine_spot_light(&light2).unwrap();
        renderer.shine_spot_light(&light3).unwrap();
        renderer.shine_spot_light(&light4).unwrap();
    }).unwrap();
}