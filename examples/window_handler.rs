
use glutin::*;

pub struct WindowHandler
{
    gl_window: GlWindow,
    events_loop: EventsLoop
}

impl WindowHandler
{
    pub fn new_default(title: &str) -> WindowHandler
    {
        let width: usize = 900;
        let height: usize = 700;
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dpi::LogicalSize::new(width as f64, height as f64));

        WindowHandler::new(window)
    }

    pub fn new(window: WindowBuilder) -> WindowHandler
    {
        let events_loop = EventsLoop::new();

        let context = ContextBuilder::new().with_vsync(true);

        let gl_window = GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }
        WindowHandler {gl_window, events_loop}
    }

    pub fn size(&self) -> (usize, usize)
    {
        let size: (u32, u32) = self.gl_window.get_inner_size().unwrap().to_physical(self.gl_window.get_hidpi_factor()).into();
        (size.0 as usize, size.1 as usize)
    }

    pub fn get_proc_address(&self, address: &str) -> *const std::os::raw::c_void
    {
        self.gl_window.get_proc_address(address) as *const std::os::raw::c_void
    }

    pub fn handle_events(&mut self)
    {
        self.events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent{ event, .. } => match event {
                    WindowEvent::CloseRequested => std::process::exit(1),
                    WindowEvent::Resized(logical_size) => {
                        //let dpi_factor = self.gl_window.get_hidpi_factor();
                        //self.gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => ()
                },
                _ => ()
            }
        });
    }

    pub fn swap_buffers(&self)
    {
        self.gl_window.swap_buffers().unwrap();
    }
}
