
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use std::cell::RefCell;
use std::rc::Rc;
use crate::event::*;

#[derive(Debug)]
pub enum Error {
    WindowCreationError {message: String},
    ContextError {message: String},
    PerformanceError {message: String},
    EventListenerError {message: String}
}

pub struct Window
{
    gl: gl::Gl,
    canvas: web_sys::HtmlCanvasElement,
    window: web_sys::Window
}

impl Window
{
    pub fn new_default(title: &str) -> Result<Window, Error>
    {
        let window = web_sys::window().ok_or(Error::WindowCreationError {message: "Unable to create web window".to_string()})?;
        let document = window.document().ok_or(Error::WindowCreationError {message: "Unable to get document".to_string()})?;
        let canvas = document.get_element_by_id("canvas").ok_or(Error::WindowCreationError {message: "Unable to get canvas, is the id different from 'canvas'?".to_string()})?;
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|e| Error::WindowCreationError {message: format!("Unable to convert to HtmlCanvasElement. Error code: {:?}", e)})?;

        let context = canvas
            .get_context("webgl2").map_err(|e| Error::ContextError {message: format!("Unable to get webgl2 context for the given canvas. Maybe your browser doesn't support WebGL2? Error code: {:?}", e)})?
            .ok_or(Error::ContextError {message: "Unable to get webgl2 context for the given canvas. Maybe your browser doesn't support WebGL2?".to_string()})?
            .dyn_into::<WebGl2RenderingContext>().map_err(|e| Error::ContextError {message: format!("Unable to get webgl2 context for the given canvas. Maybe your browser doesn't support WebGL2? Error code: {:?}", e)})?;;
        context.get_extension("EXT_color_buffer_float").map_err(|e| Error::ContextError {message: format!("Unable to get EXT_color_buffer_float extension for the given context. Maybe your browser doesn't support the get color_buffer_float extension? Error code: {:?}", e)})?;
        context.get_extension("OES_texture_float").map_err(|e| Error::ContextError {message: format!("Unable to get OES_texture_float extension for the given context. Maybe your browser doesn't support the get OES_texture_float extension? Error code: {:?}", e)})?;

        let gl = gl::Gl::new(context);
        Ok(Window { gl, canvas, window })
    }

    pub fn render_loop<F: 'static>(&mut self, mut callback: F) -> Result<(), Error>
        where F: FnMut(&Vec<Event>, f64)
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let events = Rc::new(RefCell::new(Vec::new()));
        let performance = self.window.performance().ok_or(Error::PerformanceError {message: "Performance (for timing) is not found on the window.".to_string()})?;
        let mut last_time = performance.now();

        self.add_mousedown_event_listener(events.clone())?;
        self.add_mouseup_event_listener(events.clone())?;
        self.add_mousemove_event_listener(events.clone())?;
        self.add_mousewheel_event_listener(events.clone())?;
        self.add_key_down_event_listener(events.clone())?;
        self.add_key_up_event_listener(events.clone())?;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let now = performance.now();
            let elapsed_time = now - last_time;
            last_time = now;
            callback(&(*events).borrow(), elapsed_time);
            &(*events).borrow_mut().clear();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }

    fn add_mousedown_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let button = match event.button() {
                0 => Some(MouseButton::Left),
                1 => Some(MouseButton::Middle),
                2 => Some(MouseButton::Right),
                _ => None
            };
            return if let Some(b) = button {
                (*events).borrow_mut().push(Event::MouseClick {state: State::Pressed, button: b, position: (event.offset_x() as f64, event.offset_y() as f64)});
            };
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add mouse down event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_mouseup_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let button = match event.button() {
                0 => Some(MouseButton::Left),
                1 => Some(MouseButton::Middle),
                2 => Some(MouseButton::Right),
                _ => None
            };
            return if let Some(b) = button {
                (*events).borrow_mut().push(Event::MouseClick {state: State::Released, button: b, position: (event.offset_x() as f64, event.offset_y() as f64)});
            };
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add mouse up event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_mousemove_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            (*events).borrow_mut().push(Event::MouseMotion {delta: (event.movement_x() as f64, event.movement_y() as f64)});
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add mouse move event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_mousewheel_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            (*events).borrow_mut().push(Event::MouseWheel {delta: 0.02499999912 * event.delta_y() as f64});
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add wheel event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_key_down_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if !event.default_prevented() {
                (*events).borrow_mut().push(Event::Key {state: State::Pressed, kind: map_key_code(event.code())});
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add key down event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_key_up_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if !event.default_prevented() {
                (*events).borrow_mut().push(Event::Key {state: State::Released, kind: map_key_code(event.code())});
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        window().add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add key up event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    pub fn size(&self) -> (usize, usize)
    {
        (self.canvas.width() as usize, self.canvas.height() as usize)
    }

    pub fn framebuffer_size(&self) -> (usize, usize)
    {
        self.size()
    }

    pub fn gl(&self) -> gl::Gl
    {
        self.gl.clone()
    }
}

fn map_key_code(code: String) -> String
{
    code.trim_start_matches("Key").to_string()
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}