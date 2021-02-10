
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use std::cell::RefCell;
use std::rc::Rc;
use crate::frame_input::*;

#[derive(Debug)]
pub enum Error {
    WindowCreationError {message: String},
    ContextError {message: String},
    PerformanceError {message: String},
    EventListenerError {message: String}
}

pub struct Window
{
    gl: crate::Context,
    canvas: web_sys::HtmlCanvasElement,
    window: web_sys::Window,
    maximized: bool
}

impl Window
{
    pub fn new(_title: &str, size: Option<(u32, u32)>) -> Result<Window, Error>
    {
        let window = web_sys::window().ok_or(Error::WindowCreationError {message: "Unable to create web window".to_string()})?;
        let document = window.document().ok_or(Error::WindowCreationError {message: "Unable to get document".to_string()})?;
        let canvas = document.get_element_by_id("canvas").ok_or(Error::WindowCreationError {message: "Unable to get canvas, is the id different from 'canvas'?".to_string()})?;
        let mut canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|e| Error::WindowCreationError {message: format!("Unable to convert to HtmlCanvasElement. Error code: {:?}", e)})?;

        let context = canvas
            .get_context("webgl2").map_err(|e| Error::ContextError {message: format!("Unable to get webgl2 context for the given canvas. Maybe your browser doesn't support WebGL2? Error code: {:?}", e)})?
            .ok_or(Error::ContextError {message: "Unable to get webgl2 context for the given canvas. Maybe your browser doesn't support WebGL2?".to_string()})?
            .dyn_into::<WebGl2RenderingContext>().map_err(|e| Error::ContextError {message: format!("Unable to get webgl2 context for the given canvas. Maybe your browser doesn't support WebGL2? Error code: {:?}", e)})?;
        context.get_extension("EXT_color_buffer_float").map_err(|e| Error::ContextError {message: format!("Unable to get EXT_color_buffer_float extension for the given context. Maybe your browser doesn't support the get color_buffer_float extension? Error code: {:?}", e)})?;
        context.get_extension("OES_texture_float").map_err(|e| Error::ContextError {message: format!("Unable to get OES_texture_float extension for the given context. Maybe your browser doesn't support the get OES_texture_float extension? Error code: {:?}", e)})?;

        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
            event.stop_propagation();
        }) as Box<dyn FnMut(_)>);
        canvas.set_oncontextmenu(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        if let Some((width, height)) = size {
            canvas.set_width(width);
            canvas.set_height(height);
        } else {
            maximize(&window, &mut canvas);
        };

        Ok(Window { gl: crate::context::Glstruct::new(context), canvas, window, maximized: size.is_none() })
    }

    pub fn render_loop<F: 'static>(&mut self, mut callback: F) -> Result<(), Error>
        where F: FnMut(crate::FrameInput)
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let events = Rc::new(RefCell::new(Vec::new()));
        let performance = self.window.performance().ok_or(Error::PerformanceError {message: "Performance (for timing) is not found on the window.".to_string()})?;
        let mut last_time = performance.now();
        let last_position = Rc::new(RefCell::new(None));
        let last_zoom = Rc::new(RefCell::new(None));
        let maximized = self.maximized;

        self.add_mousedown_event_listener(events.clone())?;
        self.add_touchstart_event_listener(events.clone(), last_position.clone(), last_zoom.clone())?;
        self.add_mouseup_event_listener(events.clone())?;
        self.add_touchend_event_listener(events.clone(), last_position.clone(), last_zoom.clone())?;
        self.add_mousemove_event_listener(events.clone())?;
        self.add_touchmove_event_listener(events.clone(), last_position.clone(), last_zoom.clone())?;
        self.add_mousewheel_event_listener(events.clone())?;
        self.add_key_down_event_listener(events.clone())?;
        self.add_key_up_event_listener(events.clone())?;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let now = performance.now();
            let elapsed_time = now - last_time;
            last_time = now;
            let mut canvas = canvas();
            if maximized {
                maximize(&window(), &mut canvas);
            }
            let (width, height) = (canvas.width() as usize, canvas.height() as usize);
            let frame_input = crate::FrameInput {events: (*events).borrow().clone(), elapsed_time, viewport: crate::Viewport::new_at_origo(width, height),
                window_width: width, window_height: height
            };
            callback(frame_input);
            &(*events).borrow_mut().clear();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }

    fn add_mousedown_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if !event.default_prevented() {
                let button = match event.button() {
                    0 => Some(MouseButton::Left),
                    1 => Some(MouseButton::Middle),
                    2 => Some(MouseButton::Right),
                    _ => None
                };
                if let Some(b) = button {
                    (*events).borrow_mut().push(Event::MouseClick { state: State::Pressed, button: b, position: (event.offset_x() as f64, event.offset_y() as f64) });
                };
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add mouse down event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_mouseup_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if !event.default_prevented() {
                let button = match event.button() {
                    0 => Some(MouseButton::Left),
                    1 => Some(MouseButton::Middle),
                    2 => Some(MouseButton::Right),
                    _ => None
                };
                if let Some(b) = button {
                    (*events).borrow_mut().push(Event::MouseClick { state: State::Released, button: b, position: (event.offset_x() as f64, event.offset_y() as f64) });
                };
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add mouse up event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_mousemove_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if !event.default_prevented() {
                (*events).borrow_mut().push(Event::MouseMotion {
                    delta: (event.movement_x() as f64, event.movement_y() as f64),
                    position: (event.offset_x() as f64, event.offset_y() as f64)
                });
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add mouse move event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_mousewheel_event_listener(&self, events: Rc<RefCell<Vec<Event>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            if !event.default_prevented() {
                (*events).borrow_mut().push(Event::MouseWheel {
                    delta: 0.02499999912 * event.delta_y() as f64,
                    position: (event.offset_x() as f64, event.offset_y() as f64)
                });
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref()).map_err(|e| Error::EventListenerError {message: format!("Unable to add wheel event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_touchstart_event_listener(&self, events: Rc<RefCell<Vec<Event>>>, last_position: Rc<RefCell<Option<(i32, i32)>>>, last_zoom: Rc<RefCell<Option<f64>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            if !event.default_prevented() {
                if event.touches().length() == 1 {
                    let touch = event.touches().item(0).unwrap();
                    (*events).borrow_mut().push(Event::MouseClick { state: State::Pressed, button: MouseButton::Left, position: (touch.page_x() as f64, touch.page_y() as f64) });
                    *last_position.borrow_mut() = Some((touch.page_x(), touch.page_y()));
                    *last_zoom.borrow_mut() = None;
                } else if event.touches().length() == 2 {
                    let touch0 = event.touches().item(0).unwrap();
                    let touch1 = event.touches().item(1).unwrap();
                    let zoom = f64::sqrt(f64::powi((touch0.page_x() - touch1.page_x()) as f64, 2) + f64::powi((touch0.page_y() - touch1.page_y()) as f64, 2));
                    *last_zoom.borrow_mut() = Some(zoom);
                    *last_position.borrow_mut() = None;
                } else {
                    *last_zoom.borrow_mut() = None;
                    *last_position.borrow_mut() = None;
                }
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())
            .map_err(|e| Error::EventListenerError {message: format!("Unable to add touch start event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_touchend_event_listener(&self, events: Rc<RefCell<Vec<Event>>>, last_position: Rc<RefCell<Option<(i32, i32)>>>, last_zoom: Rc<RefCell<Option<f64>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            if !event.default_prevented() {
                let touch = event.touches().item(0).unwrap();
                *last_position.borrow_mut() = None;
                *last_zoom.borrow_mut() = None;
                (*events).borrow_mut().push(Event::MouseClick { state: State::Released, button: MouseButton::Left, position: (touch.page_x() as f64, touch.page_y() as f64) });
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())
            .map_err(|e| Error::EventListenerError {message: format!("Unable to add touch end event listener. Error code: {:?}", e)})?;
        closure.forget();
        Ok(())
    }

    fn add_touchmove_event_listener(&self, events: Rc<RefCell<Vec<Event>>>, last_position: Rc<RefCell<Option<(i32, i32)>>>, last_zoom: Rc<RefCell<Option<f64>>>) -> Result<(), Error>
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            if !event.default_prevented() {
                if event.touches().length() == 1 {
                    let touch = event.touches().item(0).unwrap();
                    if let Some((x,y)) = *last_position.borrow() {
                        (*events).borrow_mut().push(Event::MouseMotion {
                            delta: ((touch.page_x() - x) as f64, (touch.page_y() - y) as f64),
                            position: (touch.page_x() as f64, touch.page_y() as f64)
                        });
                    }
                    *last_position.borrow_mut() = Some((touch.page_x(), touch.page_y()));
                    *last_zoom.borrow_mut() = None;
                }
                else if event.touches().length() == 2 {
                    let touch0 = event.touches().item(0).unwrap();
                    let touch1 = event.touches().item(1).unwrap();
                    let zoom = f64::sqrt(f64::powi((touch0.page_x() - touch1.page_x()) as f64, 2) + f64::powi((touch0.page_y() - touch1.page_y()) as f64, 2));
                    if let Some(old_zoom) = *last_zoom.borrow() {
                        (*events).borrow_mut().push(Event::MouseWheel {
                            delta: old_zoom - zoom,
                            position: (0.5 * touch0.page_x() as f64 + 0.5 * touch1.page_x() as f64,
                                    0.5 * touch0.page_y() as f64 + 0.5 * touch1.page_y() as f64)
                        });
                    }
                    *last_zoom.borrow_mut() = Some(zoom);
                    *last_position.borrow_mut() = None;
                }
                else {
                    *last_zoom.borrow_mut() = None;
                    *last_position.borrow_mut() = None;
                }
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())
            .map_err(|e| Error::EventListenerError {message: format!("Unable to add touch move event listener. Error code: {:?}", e)})?;
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

    pub fn viewport(&self) -> crate::Viewport {
        let (w, h) = self.size();
        crate::Viewport::new_at_origo(w, h)
    }

    pub fn gl(&self) -> crate::Context
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

fn canvas() -> web_sys::HtmlCanvasElement {
    window().document().expect("no global `window` exists").get_element_by_id("canvas").expect("Canvas").dyn_into::<web_sys::HtmlCanvasElement>().expect("")
}

fn maximize(window: &web_sys::Window, canvas: &mut web_sys::HtmlCanvasElement) {
    let (w, h) = (window.inner_width().unwrap().as_f64().unwrap() as u32,
                    window.inner_height().unwrap().as_f64().unwrap() as u32);
    let (width, height) = (canvas.width() as u32, canvas.height() as u32);
    if w != width || h != height {
        canvas.set_width(w);
        canvas.set_height(h);
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}