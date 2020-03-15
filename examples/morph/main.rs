
use std::collections::HashMap;

/// Loads the mesh and scale/translate it.
fn on_startup(scene_center: &tri_mesh::prelude::Vec3, scene_radius: f64) -> tri_mesh::mesh::Mesh
{
    use tri_mesh::prelude::*;
    let mut mesh = MeshBuilder::new().with_3d(include_bytes!("../assets/models/suzanne.3d")).unwrap().build().unwrap();
    let (min, max) = mesh.extreme_coordinates();
    mesh.translate(-0.5 * (max + min)); // Translate such that the mesh center is in origo.
    let size = max - min;
    mesh.scale(0.5 * scene_radius / size.x.max(size.y).max(size.z)); // Scale the mesh such that the size of the biggest side of the bounding box is half a scene radius
    mesh.translate(*scene_center); // Translate the mesh to the scene center
    mesh
}

/// When the user clicks, we see if the model is hit. If it is, we compute the morph weights from the picking point.
fn on_click(mesh: &tri_mesh::mesh::Mesh, ray_start_point: &tri_mesh::prelude::Vec3, ray_direction: &tri_mesh::prelude::Vec3) -> Option<HashMap<tri_mesh::prelude::VertexID, tri_mesh::prelude::Vec3>>
{
    if let Some((vertex_id, point)) = pick(&mesh,&ray_start_point, &ray_direction) {
        Some(compute_weights(mesh, vertex_id, &point))
    }
    else {None}
}

/// Morphs the vertices based on the computed weights.
fn on_morph(mesh: &mut tri_mesh::mesh::Mesh, weights: &HashMap<tri_mesh::prelude::VertexID, tri_mesh::prelude::Vec3>, factor: f64)
{
    for (vertex_id, weight) in weights.iter() {
        mesh.move_vertex_by(*vertex_id,weight * factor);
    }
}

/// Picking used for determining whether a mouse click starts a morph operation. Returns a close vertex and the position of the click on the mesh surface.
fn pick(mesh: &tri_mesh::mesh::Mesh, ray_start_point: &tri_mesh::prelude::Vec3, ray_direction: &tri_mesh::prelude::Vec3) -> Option<(tri_mesh::prelude::VertexID, tri_mesh::prelude::Vec3)>
{
    use tri_mesh::prelude::*;
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
fn compute_weights(mesh: &tri_mesh::mesh::Mesh, start_vertex_id: tri_mesh::prelude::VertexID, start_point: &tri_mesh::prelude::Vec3) -> HashMap<tri_mesh::prelude::VertexID, tri_mesh::prelude::Vec3>
{
    use tri_mesh::prelude::*;
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
        let sqr_distance = start_point.distance2(mesh.vertex_position(vertex_id));
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

fn main()
{
    use dust::*;

    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let scene_radius = 10.0;
    let scene_center = vec3(0.0, 5.0, 0.0);
    let mut mesh = on_startup(&tri_mesh::prelude::vec3(scene_center.x as f64, scene_center.y as f64, scene_center.z as f64), scene_radius as f64);
    let positions: Vec<f32> = mesh.positions_buffer().iter().map(|v| *v as f32).collect();
    let normals: Vec<f32> = mesh.normals_buffer().iter().map(|v| *v as f32).collect();

    let mut window = Window::new_default("Morph tool").unwrap();
    let (width, height) = window.framebuffer_size();
    let window_size = window.size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, scene_center + scene_radius * vec3(1.0, 1.0, 1.0).normalize(), scene_center, vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    // Objects
    let mut wireframe_model = Edges::new(&gl, &mesh.indices_buffer(), &positions, 0.01);
    wireframe_model.diffuse_intensity = 0.8;
    wireframe_model.specular_intensity = 0.2;
    wireframe_model.specular_power = 5.0;
    wireframe_model.color = vec3(0.9, 0.2, 0.2);

    let mut model = Mesh::new(&gl, &mesh.indices_buffer(), &positions, &normals).unwrap();
    model.color = vec3(0.8, 0.8, 0.8);
    model.diffuse_intensity = 0.2;
    model.specular_intensity = 0.4;
    model.specular_power = 20.0;

    let mut plane_mesh = tri_mesh::MeshBuilder::new().plane().build().unwrap();
    plane_mesh.scale(100.0);
    let mut plane = Mesh::new(&gl, &plane_mesh.indices_buffer(), &plane_mesh.positions_buffer_f32(), &plane_mesh.normals_buffer_f32()).unwrap();
    plane.color = vec3(0.8, 0.8, 0.8);
    plane.diffuse_intensity = 0.2;
    plane.specular_intensity = 0.4;
    plane.specular_power = 20.0;

    let ambient_light = AmbientLight::new(&gl, 0.4, &vec3(1.0, 1.0, 1.0)).unwrap();

    let mut dir = vec3(-1.0, -1.0, -1.0).normalize();
    let mut spot_light0 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &(scene_center - 2.0f32 * scene_radius * dir),
                                   &dir, 25.0, 0.1, 0.001, 0.0001).unwrap();
    dir = vec3(1.0, -1.0, -1.0).normalize();
    let mut spot_light1 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &(scene_center - 2.0f32 * scene_radius * dir),
                                   &dir, 25.0, 0.1, 0.001, 0.0001).unwrap();
    dir = vec3(1.0, -1.0, 1.0).normalize();
    let mut spot_light2 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &(scene_center - 2.0f32 * scene_radius * dir),
                                   &dir, 25.0, 0.1, 0.001, 0.0001).unwrap();
    dir = vec3(-1.0, -1.0, 1.0).normalize();
    let mut spot_light3 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &(scene_center - 2.0f32 * scene_radius * dir),
                                   &dir, 25.0, 0.1, 0.001, 0.0001).unwrap();

    let mut weights: Option<HashMap<tri_mesh::prelude::VertexID, tri_mesh::prelude::Vec3>> = None;
    // main loop
    let mut rotating = false;
    window.render_loop(move |frame_input|
    {
        camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, position} => {
                    if *button == MouseButton::Left
                    {
                        if *state == State::Pressed
                        {
                            let (x, y) = (position.0 / window_size.0 as f64, position.1 / window_size.1 as f64);
                            let p = camera.position();
                            let dir = camera.view_direction_at((x, y));
                            weights = on_click(&mesh,&tri_mesh::prelude::vec3(p.x as f64, p.y as f64, p.z as f64), &tri_mesh::prelude::vec3(dir.x as f64, dir.y as f64, dir.z as f64));
                            if weights.is_none() {
                                rotating = true;
                            }
                        }
                        else {
                            weights = None;
                            rotating = false;
                        }
                    }
                },
                Event::MouseWheel {delta} => {
                    camera.zoom(*delta as f32);
                },
                Event::MouseMotion {delta} => {
                    if rotating {
                        camera.rotate(delta.0 as f32, delta.1 as f32);
                    }
                    if let Some(ref w) = weights
                    {
                        on_morph(&mut mesh, w, 0.001 * delta.1);
                        let positions: Vec<f32> = mesh.positions_buffer().iter().map(|v| *v as f32).collect();
                        let normals: Vec<f32> = mesh.normals_buffer().iter().map(|v| *v as f32).collect();
                        wireframe_model.update_positions(&positions);
                        model.update_positions(&positions).unwrap();
                        model.update_normals(&normals).unwrap();
                    }
                },
                _ => {}
            }
        }
        let render_scene = |camera: &Camera| {
            state::cull(&gl, state::CullType::Back);
            model.render(&Mat4::identity(), camera);
        };
        spot_light0.generate_shadow_map(50.0, 512, &render_scene);
        spot_light1.generate_shadow_map(50.0, 512, &render_scene);
        spot_light2.generate_shadow_map(50.0, 512, &render_scene);
        spot_light3.generate_shadow_map(50.0, 512, &render_scene);

        // Geometry pass
        renderer.geometry_pass(width, height, &|| {
            state::cull(&gl, state::CullType::Back);
            model.render(&Mat4::identity(), &camera);
            plane.render(&Mat4::identity(), &camera);
            wireframe_model.render(&Mat4::identity(), &camera);
        }).unwrap();

        // Light pass
        Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.5, 0.5, 0.5, 1.0)), None, &|| {
            renderer.light_pass(&camera, Some(&ambient_light), &[], &[&spot_light0, &spot_light1, &spot_light2, &spot_light3], &[]).unwrap();
        }).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            Screen::save_color(path, &gl, 0, 0, width, height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}