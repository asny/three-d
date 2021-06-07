use crate::*;
#[doc(hidden)]
pub use egui;

///
/// Integration of [egui](https://crates.io/crates/egui), an immediate mode GUI.
///
pub struct GUI {
    context: Context,
    egui_context: egui::CtxRef,
    width: u32,
    height: u32,
    program: Program,
    texture_version: u64,
    texture: Option<Texture2D>,
}

impl GUI {
    ///
    /// Creates a new GUI.
    ///
    pub fn new(context: &Context) -> Result<Self, Error> {
        Ok(GUI {
            egui_context: egui::CtxRef::default(),
            context: context.clone(),
            width: 0,
            height: 0,
            texture_version: 0,
            texture: None,
            program: Program::from_source(
                context,
                &format!(
                    "{}{}",
                    include_str!("../core/shared.frag"),
                    include_str!("shaders/egui.vert")
                ),
                &format!(
                    "{}{}",
                    include_str!("../core/shared.frag"),
                    include_str!("shaders/egui.frag")
                ),
            )?,
        })
    }

    ///
    /// Initialises a new frame of the GUI and handles events.
    /// Construct the GUI (Add panels, widgets etc.) using the [egui::CtxRef](egui::CtxRef) in the callback function.
    /// This function returns whether or not the GUI has changed, ie. if it consumes any events, and therefore needs to be rendered again.
    ///
    pub fn update<F: FnOnce(&egui::CtxRef)>(
        &mut self,
        frame_input: &mut FrameInput,
        callback: F,
    ) -> Result<bool, Error> {
        self.width = frame_input.window_width;
        self.height = frame_input.window_height;
        let input_state = construct_input_state(frame_input);
        self.egui_context.begin_frame(input_state);
        callback(&self.egui_context);

        let mut change = false;
        for event in frame_input.events.iter_mut() {
            if self.egui_context.wants_pointer_input() {
                match event {
                    Event::MouseClick(MouseClickEvent {
                        ref mut handled, ..
                    }) => {
                        *handled = true;
                    }
                    Event::MouseWheel(MouseWheelEvent {
                        ref mut handled, ..
                    }) => {
                        *handled = true;
                    }
                    Event::MouseMotion(MouseMotionEvent {
                        ref mut handled, ..
                    }) => {
                        *handled = true;
                    }
                    _ => {}
                }
                change = true;
            }

            if self.egui_context.wants_keyboard_input() {
                match event {
                    Event::Key(KeyEvent {
                        ref mut handled, ..
                    }) => {
                        *handled = true;
                    }
                    _ => {}
                }
                change = true;
            }
        }
        Ok(change)
    }

    ///
    /// Render the GUI defined in the [update](Self::update) function. Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn render(&mut self) -> Result<(), Error> {
        let (_, shapes) = self.egui_context.end_frame();
        let clipped_meshes = self.egui_context.tessellate(shapes);

        let egui_texture = self.egui_context.texture();

        if self.texture.is_none() || self.texture_version != egui_texture.version {
            let mut pixels = Vec::new();
            for pixel in egui_texture.srgba_pixels() {
                pixels.push(pixel.r());
                pixels.push(pixel.g());
                pixels.push(pixel.b());
                pixels.push(pixel.a());
            }
            self.texture = Some(Texture2D::new(
                &self.context,
                &CPUTexture {
                    data: pixels,
                    format: Format::SRGBA,
                    width: egui_texture.width as u32,
                    height: egui_texture.height as u32,
                    mip_map_filter: None,
                    wrap_s: Wrapping::ClampToEdge,
                    wrap_t: Wrapping::ClampToEdge,
                    ..Default::default()
                },
            )?);
            self.texture_version = egui_texture.version;
        };

        for egui::ClippedMesh(_, mesh) in clipped_meshes {
            self.paint_mesh(
                self.width,
                self.height,
                self.egui_context.pixels_per_point(),
                &mesh,
                self.texture.as_ref().unwrap(),
            )?;
        }
        Ok(())
    }

    fn paint_mesh(
        &self,
        width: u32,
        height: u32,
        pixels_per_point: f32,
        mesh: &egui::paint::Mesh,
        texture: &Texture2D,
    ) -> Result<(), Error> {
        debug_assert!(mesh.is_valid());

        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut uvs = Vec::new();
        for v in mesh.vertices.iter() {
            positions.push(v.pos.x);
            positions.push(v.pos.y);
            uvs.push(v.uv.x);
            uvs.push(v.uv.y);
            colors.push(v.color[0] as f32);
            colors.push(v.color[1] as f32);
            colors.push(v.color[2] as f32);
            colors.push(v.color[3] as f32);
        }
        let indices: Vec<u32> = mesh.indices.iter().map(|idx| *idx as u32).collect();

        let position_buffer = VertexBuffer::new_with_static(&self.context, &positions)?;
        let uv_buffer = VertexBuffer::new_with_static(&self.context, &uvs)?;
        let color_buffer = VertexBuffer::new_with_static(&self.context, &colors)?;
        let index_buffer = ElementBuffer::new(&self.context, &indices)?;

        let viewport = Viewport::new_at_origo(
            (width as f32 * pixels_per_point).round() as u32,
            (height as f32 * pixels_per_point).round() as u32,
        );

        let render_states = RenderStates {
            blend: Some(BlendParameters {
                source_rgb_multiplier: BlendMultiplierType::One,
                destination_rgb_multiplier: BlendMultiplierType::OneMinusSrcAlpha,
                source_alpha_multiplier: BlendMultiplierType::OneMinusDstAlpha,
                destination_alpha_multiplier: BlendMultiplierType::One,
                ..Default::default()
            }),
            depth_test: DepthTestType::Always,
            ..Default::default()
        };

        self.program.use_texture(texture, "u_sampler")?;
        self.program
            .use_uniform_vec2("u_screen_size", &vec2(width as f32, height as f32))?;

        self.program.use_attribute_vec2(&position_buffer, "a_pos")?;
        self.program.use_attribute_vec4(&color_buffer, "a_srgba")?;
        self.program.use_attribute_vec2(&uv_buffer, "a_tc")?;

        self.program
            .draw_elements(render_states, CullType::None, viewport, &index_buffer);
        Ok(())
    }
}

fn construct_input_state(frame_input: &mut FrameInput) -> egui::RawInput {
    let mut scroll_delta = egui::Vec2::ZERO;
    let mut egui_modifiers = egui::Modifiers::default();
    let mut egui_events = Vec::new();
    for event in frame_input.events.iter() {
        match event {
            Event::Key(KeyEvent {
                kind,
                state,
                modifiers,
                handled,
            }) => {
                if !handled {
                    egui_events.push(egui::Event::Key {
                        key: translate_to_egui_key_code(kind),
                        pressed: *state == State::Pressed,
                        modifiers: map_modifiers(modifiers),
                    });
                }
            }
            Event::MouseClick(MouseClickEvent {
                state,
                button,
                position,
                modifiers,
                handled,
            }) => {
                if !handled {
                    egui_events.push(egui::Event::PointerButton {
                        pos: egui::Pos2 {
                            x: position.0 as f32,
                            y: position.1 as f32,
                        },
                        button: match button {
                            MouseButton::Left => egui::PointerButton::Primary,
                            MouseButton::Right => egui::PointerButton::Secondary,
                            MouseButton::Middle => egui::PointerButton::Middle,
                        },
                        pressed: *state == State::Pressed,
                        modifiers: map_modifiers(modifiers),
                    });
                }
            }
            Event::MouseMotion(MouseMotionEvent {
                position, handled, ..
            }) => {
                if !handled {
                    egui_events.push(egui::Event::PointerMoved(egui::Pos2 {
                        x: position.0 as f32,
                        y: position.1 as f32,
                    }));
                }
            }
            Event::Text(text) => {
                egui_events.push(egui::Event::Text(text.clone()));
            }
            Event::MouseLeave => {
                egui_events.push(egui::Event::PointerGone);
            }
            Event::MouseWheel(MouseWheelEvent { delta, handled, .. }) => {
                if !handled {
                    scroll_delta = egui::Vec2::new(delta.0 as f32, delta.1 as f32);
                }
            }
            Event::ModifiersChange(ModifiersChangeEvent { modifiers }) => {
                egui_modifiers = egui::Modifiers {
                    alt: modifiers.alt == State::Pressed,
                    ctrl: modifiers.ctrl == State::Pressed,
                    shift: modifiers.shift == State::Pressed,
                    mac_cmd: modifiers.command == State::Pressed,
                    command: modifiers.command == State::Pressed,
                }
            }
            _ => (),
        }
    }

    egui::RawInput {
        scroll_delta,
        screen_rect: Some(egui::Rect::from_min_size(
            Default::default(),
            egui::Vec2 {
                x: frame_input.window_width as f32,
                y: frame_input.window_height as f32,
            },
        )),
        pixels_per_point: Some(frame_input.device_pixel_ratio as f32),
        time: Some(frame_input.accumulated_time * 0.001),
        modifiers: egui_modifiers,
        events: egui_events,
        ..Default::default()
    }
}

fn translate_to_egui_key_code(key: &crate::Key) -> egui::Key {
    use crate::Key::*;
    use egui::Key;

    match key {
        ArrowDown => Key::ArrowDown,
        ArrowLeft => Key::ArrowLeft,
        ArrowRight => Key::ArrowRight,
        ArrowUp => Key::ArrowUp,

        Escape => Key::Escape,
        Tab => Key::Tab,
        Backspace => Key::Backspace,
        Enter => Key::Enter,
        Space => Key::Space,

        Insert => Key::Insert,
        Delete => Key::Delete,
        Home => Key::Home,
        End => Key::End,
        PageUp => Key::PageUp,
        PageDown => Key::PageDown,

        Num0 => Key::Num0,
        Num1 => Key::Num1,
        Num2 => Key::Num2,
        Num3 => Key::Num3,
        Num4 => Key::Num4,
        Num5 => Key::Num5,
        Num6 => Key::Num6,
        Num7 => Key::Num7,
        Num8 => Key::Num8,
        Num9 => Key::Num9,

        A => Key::A,
        B => Key::B,
        C => Key::C,
        D => Key::D,
        E => Key::E,
        F => Key::F,
        G => Key::G,
        H => Key::H,
        I => Key::I,
        J => Key::J,
        K => Key::K,
        L => Key::L,
        M => Key::M,
        N => Key::N,
        O => Key::O,
        P => Key::P,
        Q => Key::Q,
        R => Key::R,
        S => Key::S,
        T => Key::T,
        U => Key::U,
        V => Key::V,
        W => Key::W,
        X => Key::X,
        Y => Key::Y,
        Z => Key::Z,
    }
}

fn map_modifiers(modifiers: &Modifiers) -> egui::Modifiers {
    egui::Modifiers {
        alt: modifiers.alt == State::Pressed,
        ctrl: modifiers.ctrl == State::Pressed,
        shift: modifiers.shift == State::Pressed,
        command: modifiers.command == State::Pressed,
        mac_cmd: cfg!(target_os = "macos") && modifiers.command == State::Pressed,
    }
}
