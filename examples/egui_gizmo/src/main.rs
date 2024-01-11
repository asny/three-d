use egui::color_picker::Alpha;
use egui::menu;
use egui::{pos2, Align2, Color32, FontId, LayerId, Ui, Widget};
use egui_gizmo::{
    Gizmo, GizmoMode, GizmoOrientation, GizmoResult, GizmoVisuals, DEFAULT_SNAP_ANGLE,
    DEFAULT_SNAP_DISTANCE,
};
use three_d::*;

// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

struct GizmoOptions {
    gizmo_mode: GizmoMode,
    gizmo_orientation: GizmoOrientation,
    custom_highlight_color: bool,
    visuals: GizmoVisuals,
}

const SOURCE_URL: &str =
    "https://github.com/asny/three-d/blob/master/examples/egui_gizmo/src/main.rs";

pub fn convert_mat4_to_mint(mat: &Mat4) -> mint::ColumnMatrix4<f32> {
    let tab: [[f32; 4]; 4] = [
        [mat.x.x, mat.x.y, mat.x.z, mat.x.w],
        [mat.y.x, mat.y.y, mat.y.z, mat.y.w],
        [mat.z.x, mat.z.y, mat.z.z, mat.z.w],
        [mat.w.x, mat.w.y, mat.w.z, mat.w.w],
    ];
    mint::ColumnMatrix4::from(tab)
}

pub fn convert_mint_to_mat4(mat: &mint::ColumnMatrix4<f32>) -> Mat4 {
    Mat4::from_cols(
        vec4(mat.x.x, mat.x.y, mat.x.z, mat.x.w),
        vec4(mat.y.x, mat.y.y, mat.y.z, mat.y.w),
        vec4(mat.z.x, mat.z.y, mat.z.z, mat.z.w),
        vec4(mat.w.x, mat.w.y, mat.w.z, mat.w.w),
    )
}

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Egui Gizmo!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut gui = three_d::GUI::new(&context);

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 5.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );

    let mut gizmo_options = GizmoOptions {
        gizmo_mode: GizmoMode::Rotate,
        gizmo_orientation: GizmoOrientation::Global,
        custom_highlight_color: false,
        visuals: GizmoVisuals {
            x_color: Color32::from_rgb(255, 0, 148),
            y_color: Color32::from_rgb(148, 255, 0),
            z_color: Color32::from_rgb(0, 148, 255),
            s_color: Color32::WHITE,
            ..Default::default()
        },
    };

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    let mut frame_count = 0;

    let mut model_matrix = Mat4::identity();

    window.render_loop(move |mut frame_input| {
        frame_count += 1;
        let elapsed_time = frame_input.elapsed_time as f32 / 1000.0;
        let fps = (1.0 / elapsed_time).round() as i32;

        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ctx| {
                egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                    menu::bar(ui, |ui| {
                        menu::menu_button(ui, "File", |ui| {
                            if ui.button("Exit").clicked() {
                                std::process::exit(0);
                            }
                        });
                    });
                });

                egui::Window::new("Settings")
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(format!("Frames: {frame_count} | FPS: {fps}"));
                        egui::ComboBox::from_label("Mode")
                            .selected_text(format!("{:?}", gizmo_options.gizmo_mode))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut gizmo_options.gizmo_mode,
                                    GizmoMode::Rotate,
                                    "Rotate",
                                );
                                ui.selectable_value(
                                    &mut gizmo_options.gizmo_mode,
                                    GizmoMode::Translate,
                                    "Translate",
                                );
                                ui.selectable_value(
                                    &mut gizmo_options.gizmo_mode,
                                    GizmoMode::Scale,
                                    "Scale",
                                );
                            });
                        ui.end_row();

                        egui::ComboBox::from_label("Orientation")
                            .selected_text(format!("{:?}", gizmo_options.gizmo_orientation))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut gizmo_options.gizmo_orientation,
                                    GizmoOrientation::Global,
                                    "Global",
                                );
                                ui.selectable_value(
                                    &mut gizmo_options.gizmo_orientation,
                                    GizmoOrientation::Local,
                                    "Local",
                                );
                            });
                        ui.end_row();

                        ui.separator();

                        egui::Slider::new(&mut gizmo_options.visuals.gizmo_size, 10.0..=500.0)
                            .text("Gizmo size")
                            .ui(ui);
                        egui::Slider::new(&mut gizmo_options.visuals.stroke_width, 0.1..=10.0)
                            .text("Stroke width")
                            .ui(ui);
                        egui::Slider::new(&mut gizmo_options.visuals.inactive_alpha, 0.0..=1.0)
                            .text("Inactive alpha")
                            .ui(ui);
                        egui::Slider::new(&mut gizmo_options.visuals.highlight_alpha, 0.0..=1.0)
                            .text("Highlighted alpha")
                            .ui(ui);

                        ui.horizontal(|ui| {
                            egui::color_picker::color_edit_button_srgba(
                                ui,
                                gizmo_options
                                    .visuals
                                    .highlight_color
                                    .get_or_insert(Color32::GOLD),
                                Alpha::Opaque,
                            );
                            egui::Checkbox::new(
                                &mut gizmo_options.custom_highlight_color,
                                "Custom highlight color",
                            )
                            .ui(ui);
                        });

                        ui.horizontal(|ui| {
                            egui::color_picker::color_edit_button_srgba(
                                ui,
                                &mut gizmo_options.visuals.x_color,
                                Alpha::Opaque,
                            );
                            egui::Label::new("X axis color").wrap(false).ui(ui);
                        });

                        ui.horizontal(|ui| {
                            egui::color_picker::color_edit_button_srgba(
                                ui,
                                &mut gizmo_options.visuals.y_color,
                                Alpha::Opaque,
                            );
                            egui::Label::new("Y axis color").wrap(false).ui(ui);
                        });
                        ui.horizontal(|ui| {
                            egui::color_picker::color_edit_button_srgba(
                                ui,
                                &mut gizmo_options.visuals.z_color,
                                Alpha::Opaque,
                            );
                            egui::Label::new("Z axis color").wrap(false).ui(ui);
                        });
                        ui.horizontal(|ui| {
                            egui::color_picker::color_edit_button_srgba(
                                ui,
                                &mut gizmo_options.visuals.s_color,
                                Alpha::Opaque,
                            );
                            egui::Label::new("Screen axis color").wrap(false).ui(ui);
                        });
                        ui.end_row();

                        egui::Hyperlink::from_label_and_url("(source code)", SOURCE_URL).ui(ui);
                    });

                egui::Area::new("Viewport")
                    .fixed_pos((0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.with_layer_id(LayerId::background(), |ui| {
                            // Snapping is enabled with ctrl key.
                            let snapping = ui.input(|i| i.modifiers.ctrl);
                            let precise_snap = ui.input(|i| i.modifiers.shift);

                            // Snap angle to use for rotation when snapping is enabled.
                            // Smaller snap angle is used when shift key is pressed.
                            let snap_angle = if precise_snap {
                                DEFAULT_SNAP_ANGLE / 2.0
                            } else {
                                DEFAULT_SNAP_ANGLE
                            };

                            // Snap distance to use for translation when snapping is enabled.
                            // Smaller snap distance is used when shift key is pressed.
                            let snap_distance = if precise_snap {
                                DEFAULT_SNAP_DISTANCE / 2.0
                            } else {
                                DEFAULT_SNAP_DISTANCE
                            };

                            let visuals = GizmoVisuals {
                                highlight_color: if gizmo_options.custom_highlight_color {
                                    gizmo_options.visuals.highlight_color
                                } else {
                                    None
                                },
                                ..gizmo_options.visuals
                            };

                            let gizmo = Gizmo::new("My gizmo")
                                .view_matrix(convert_mat4_to_mint(camera.view()))
                                .projection_matrix(convert_mat4_to_mint(camera.projection()))
                                .model_matrix(convert_mat4_to_mint(&model_matrix))
                                .mode(gizmo_options.gizmo_mode)
                                .orientation(gizmo_options.gizmo_orientation)
                                .snapping(snapping)
                                .snap_angle(snap_angle)
                                .snap_distance(snap_distance)
                                .visuals(visuals);

                            let gizmo_response = gizmo.interact(ui);

                            if let Some(gizmo_response) = gizmo_response {
                                model_matrix = convert_mint_to_mat4(&gizmo_response.transform());
                                show_gizmo_status(ui, gizmo_response);
                            }

                            instructions_text(ui);
                        });
                    });
            },
        );

        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        cube.set_transformation(model_matrix);

        let screen = frame_input.screen();
        screen
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, cube.into_iter(), &[&light0, &light1]);

        screen.write(|| gui.render());

        FrameOutput::default()
    });
}

fn instructions_text(ui: &Ui) {
    let rect = ui.clip_rect();
    ui.painter().text(
        pos2(rect.left() + 10.0, rect.bottom() - 10.0),
        Align2::LEFT_BOTTOM,
        "Move and rotate the camera using the left mouse button\n\
         Toggle gizmo snapping with ctrl & shift",
        FontId::default(),
        Color32::GRAY,
    );
}

fn show_gizmo_status(ui: &Ui, response: GizmoResult) {
    let length = Vec3::from(response.value).distance(Vec3::zero());

    let text = match response.mode {
        GizmoMode::Rotate => format!("{:.1}°, {:.2} rad", length.to_degrees(), length),

        GizmoMode::Translate | GizmoMode::Scale => format!(
            "dX: {:.2}, dY: {:.2}, dZ: {:.2}",
            response.value[0], response.value[1], response.value[2]
        ),
    };

    let rect = ui.clip_rect();
    ui.painter().text(
        pos2(rect.right() - 10.0, rect.bottom() - 10.0),
        Align2::RIGHT_BOTTOM,
        text,
        FontId::default(),
        Color32::WHITE,
    );
}
