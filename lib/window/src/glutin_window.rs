
use glutin::*;
use crate::event;

#[derive(Debug)]
pub enum Error {
    WindowCreationError(glutin::CreationError),
    ContextError(glutin::ContextError)
}

impl From<glutin::CreationError> for Error {
    fn from(other: glutin::CreationError) -> Self {
        Error::WindowCreationError(other)
    }
}

impl From<glutin::ContextError> for Error {
    fn from(other: glutin::ContextError) -> Self {
        Error::ContextError(other)
    }
}

pub struct Window
{
    gl_window: GlWindow,
    events_loop: EventsLoop,
    gl: gl::Gl
}

impl Window
{
    pub fn new_default(title: &str) -> Result<Window, Error>
    {
        let width: usize = 1280;
        let height: usize = 720;
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dpi::LogicalSize::new(width as f64, height as f64));

        Window::new(window)
    }

    pub fn new(window: WindowBuilder) -> Result<Window, Error>
    {
        let events_loop = EventsLoop::new();

        let context = ContextBuilder::new().with_vsync(true);

        let gl_window = GlWindow::new(window, context, &events_loop)?;

        unsafe {
            gl_window.make_current()?;
        }
        let gl = gl::Gl::load_with(|s| gl_window.get_proc_address(s) as *const std::os::raw::c_void);
        Ok(Window {gl_window, events_loop, gl})
    }

    pub fn render_loop<F: 'static>(&mut self, mut callback: F) -> Result<(), Error>
        where F: FnMut(&Vec<event::Event>, f64)
    {
        let mut events = Vec::new();
        let mut last_time = std::time::Instant::now();
        let mut error = Ok(());
        while error.is_ok() {
            self.events_loop.poll_events(|event| {
                Self::handle_window_close_events(&event);
                if let Some(e) = Self::map_event(&event)
                {
                    events.push(e);
                }
            });

            let now = std::time::Instant::now();
            let duration = now.duration_since(last_time);
            last_time = now;
            let elapsed_time = duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 * 1e-6;

            callback(&events, elapsed_time);
            events.clear();
            error = self.gl_window.swap_buffers();
        }
        error?;
        Ok(())
    }

    pub fn framebuffer_size(&self) -> (usize, usize)
    {
        let t: (u32, u32) = self.gl_window.get_inner_size().unwrap().to_physical(self.gl_window.get_hidpi_factor()).into();
        (t.0 as usize, t.1 as usize)
    }

    pub fn size(&self) -> (usize, usize)
    {
        let t: (u32, u32) = self.gl_window.get_inner_size().unwrap().into();
        (t.0 as usize, t.1 as usize)
    }

    pub fn gl(&self) -> gl::Gl
    {
        self.gl.clone()
    }

    pub fn map_event(event: &Event) -> Option<event::Event>
    {
        static mut CURSOR_POS: Option<(f64, f64)> = None;
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::KeyboardInput {input, ..} => {
                    if let Some(keycode) = input.virtual_keycode {
                        let state = if input.state == ElementState::Pressed {event::State::Pressed} else {event::State::Released};
                        return Some(event::Event::Key {state, kind: format!("{:?}", keycode)});
                    }
                },
                WindowEvent::MouseWheel {delta, ..} => {
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) => {
                            return Some(event::Event::MouseWheel { delta: *y as f64 });
                        },
                        MouseScrollDelta::PixelDelta(logical_position) => {
                            return Some(event::Event::MouseWheel { delta: logical_position.y });
                        }
                    }
                },
                WindowEvent::MouseInput {state, button, ..} => {
                    if let Some(position) = unsafe {CURSOR_POS}
                    {
                        let state = if *state == ElementState::Pressed {event::State::Pressed} else {event::State::Released};
                        let button = match button {
                            MouseButton::Left => Some(event::MouseButton::Left),
                            MouseButton::Middle => Some(event::MouseButton::Middle),
                            MouseButton::Right => Some(event::MouseButton::Right),
                            _ => None
                        };
                        return button.map(|b| event::Event::MouseClick { state, button: b, position });
                    }
                },
                WindowEvent::CursorMoved {position, ..} => {
                    unsafe {
                        CURSOR_POS = Some((position.x, position.y));
                    }
                },
                _ => ()
            },
            Event::DeviceEvent{ event, .. } => match event {
                DeviceEvent::MouseMotion {delta} => {
                    return Some(event::Event::MouseMotion {delta: *delta});
                },
                _ => {}
            }
            _ => ()
        }
        None
    }

    fn handle_window_close_events(event: &Event)
    {
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::CloseRequested => std::process::exit(1),
                WindowEvent::KeyboardInput {input, ..} => {
                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == VirtualKeyCode::Escape
                        {
                            std::process::exit(1);
                        }
                    }
                },
                _ => ()
            },
            _ => ()
        }
    }
}
