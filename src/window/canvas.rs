use crate::core::{Context, Viewport};
use crate::window::*;
use crate::Result;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use thiserror::Error;

///
/// Error related to the canvas.
///
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum CanvasError {
    #[error("failed creating a new window")]
    WindowCreation,
    #[error("unable to get document from canvas")]
    DocumentMissing,
    #[error("unable to get canvas")]
    CanvasMissing,
    #[error("unable to convert canvas to html canvas: {0}")]
    CanvasConvertFailed(String),
    #[error("unable to get webgl2 context for the given canvas, maybe the browser doesn't support WebGL2{0}")]
    WebGL2NotSupported(String),
    #[error("unable to get EXT_color_buffer_float extension for the given canvas, maybe the browser doesn't support EXT_color_buffer_float: {0}")]
    ColorBufferFloatNotSupported(String),
    #[error("unable to get OES_texture_float extension for the given canvas, maybe the browser doesn't support OES_texture_float: {0}")]
    OESTextureFloatNotSupported(String),
    #[error("performance (for timing) is not found on the window")]
    PerformanceMissing,
    #[error("unable to add {0} event listener: {1}")]
    EventListenerFail(String, String),
}

pub struct Window {
    canvas: Option<web_sys::HtmlCanvasElement>,
    window: Rc<web_sys::Window>,
    settings: WindowSettings,
    closures: Vec<Closure<dyn FnMut()>>,
    closures_with_event: Vec<Closure<dyn FnMut(web_sys::Event)>>,
    closures_with_mouseevent: Vec<Closure<dyn FnMut(web_sys::MouseEvent)>>,
    closures_with_wheelevent: Vec<Closure<dyn FnMut(web_sys::WheelEvent)>>,
    closures_with_touchevent: Vec<Closure<dyn FnMut(web_sys::TouchEvent)>>,
    closures_with_keyboardevent: Vec<Closure<dyn FnMut(web_sys::KeyboardEvent)>>,
}

impl Window {
    pub fn new(settings: WindowSettings) -> Result<Window> {
        let websys_window = web_sys::window().ok_or(CanvasError::WindowCreation)?;
        let document = websys_window
            .document()
            .ok_or(CanvasError::DocumentMissing)?;

        let mut window = Window {
            canvas: None,
            window: Rc::new(websys_window),
            settings,
            closures: Vec::new(),
            closures_with_event: Vec::new(),
            closures_with_mouseevent: Vec::new(),
            closures_with_wheelevent: Vec::new(),
            closures_with_touchevent: Vec::new(),
            closures_with_keyboardevent: Vec::new(),
        };
        if let Some(canvas) = document.get_elements_by_tag_name("canvas").item(0) {
            window.set_canvas(
                canvas
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .map_err(|e| CanvasError::CanvasConvertFailed(format!("{:?}", e)))?,
            )?;
        };
        Ok(window)
    }

    ///
    /// Get the canvas which is rendered to when using [Screen](crate::Screen).
    /// If there is no canvas specified using the set_canvas function and no default canvas is found, this will return an error.
    ///
    pub fn canvas(&self) -> Result<&web_sys::HtmlCanvasElement> {
        self.canvas
            .as_ref()
            .ok_or(Box::new(CanvasError::CanvasMissing))
    }

    ///
    /// Specifies the canvas to write to when using [Screen](crate::Screen). Will overwrite the default canvas if any has been found.
    ///
    pub fn set_canvas(&mut self, canvas: web_sys::HtmlCanvasElement) -> Result<()> {
        self.canvas = Some(canvas);
        self.set_canvas_size()?;
        Ok(())
    }

    pub fn size(&self) -> Result<(u32, u32)> {
        let canvas = self.canvas.as_ref().ok_or(CanvasError::CanvasMissing)?;
        Ok((canvas.width(), canvas.height()))
    }

    pub fn viewport(&self) -> Result<Viewport> {
        let (w, h) = self.size()?;
        Ok(Viewport::new_at_origo(w, h))
    }

    pub fn gl(&self) -> Result<Context> {
        let context_options = ContextOptions {
            antialias: self.settings.multisamples > 0,
        };
        let context = self
            .canvas
            .as_ref()
            .ok_or(CanvasError::CanvasMissing)?
            .get_context_with_context_options(
                "webgl2",
                &JsValue::from_serde(&context_options).unwrap(),
            )
            .map_err(|e| CanvasError::WebGL2NotSupported(format!(": {:?}", e)))?
            .ok_or(CanvasError::WebGL2NotSupported("".to_string()))?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|e| CanvasError::WebGL2NotSupported(format!(": {:?}", e)))?;
        context
            .get_extension("EXT_color_buffer_float")
            .map_err(|e| CanvasError::ColorBufferFloatNotSupported(format!("{:?}", e)))?;
        context
            .get_extension("OES_texture_float")
            .map_err(|e| CanvasError::OESTextureFloatNotSupported(format!(": {:?}", e)))?;
        Ok(crate::core::Context::new(crate::context::Context::new(
            context,
        )))
    }

    pub fn render_loop<F: 'static + FnMut(FrameInput) -> FrameOutput>(
        mut self,
        mut callback: F,
    ) -> Result<()> {
        let performance = self
            .window
            .performance()
            .ok_or(CanvasError::PerformanceMissing)?;
        let mut last_time = performance.now();
        let mut accumulated_time = 0.0;
        let mut first_frame = true;

        let input = Input::new(self.window.clone());
        self.add_context_menu_event_listener()?;
        self.add_resize_event_listener(input.clone())?;
        self.add_mouseenter_event_listener(input.clone())?;
        self.add_mouseleave_event_listener(input.clone())?;
        self.add_mousedown_event_listener(input.clone())?;
        self.add_mouseup_event_listener(input.clone())?;
        self.add_mousemove_event_listener(input.clone())?;
        self.add_mousewheel_event_listener(input.clone())?;
        self.add_touchstart_event_listener(input.clone())?;
        self.add_touchend_event_listener(input.clone())?;
        self.add_touchmove_event_listener(input.clone())?;
        self.add_key_down_event_listener(input.clone())?;
        self.add_key_up_event_listener(input.clone())?;

        let input_clone = input.clone();
        input.borrow_mut().render_loop_closure = Some(Closure::wrap(Box::new(move || {
            let events = input_clone.borrow_mut().start_frame();
            let now = performance.now();
            let elapsed_time = now - last_time;
            last_time = now;
            accumulated_time += elapsed_time;
            self.set_canvas_size().unwrap();
            let device_pixel_ratio = self.pixels_per_point();
            let canvas = self.canvas.as_ref().unwrap();
            let (width, height) = (
                (canvas.width() as f64 / device_pixel_ratio) as u32,
                (canvas.height() as f64 / device_pixel_ratio) as u32,
            );
            let frame_input = FrameInput {
                events,
                elapsed_time,
                accumulated_time,
                viewport: Viewport::new_at_origo(
                    (device_pixel_ratio * width as f64) as u32,
                    (device_pixel_ratio * height as f64) as u32,
                ),
                window_width: width,
                window_height: height,
                device_pixel_ratio,
                first_frame: first_frame,
            };
            first_frame = false;
            let frame_output = callback(frame_input);

            if !frame_output.wait_next_event {
                input_clone.borrow_mut().request_animation_frame();
            }
        })
            as Box<dyn FnMut()>));
        input.borrow_mut().request_animation_frame();
        Ok(())
    }

    fn pixels_per_point(&self) -> f64 {
        let pixels_per_point = self.window.device_pixel_ratio() as f64;
        if pixels_per_point > 0.0 && pixels_per_point.is_finite() {
            pixels_per_point
        } else {
            1.0
        }
    }

    fn set_canvas_size(&self) -> Result<()> {
        let canvas = self.canvas.as_ref().ok_or(CanvasError::CanvasMissing)?;
        let (window_width, window_height) = (
            self.window.inner_width().unwrap().as_f64().unwrap() as u32,
            self.window.inner_height().unwrap().as_f64().unwrap() as u32,
        );
        let (mut width, mut height) = if let Some((w, h)) = self.settings.max_size {
            (u32::min(w, window_width), u32::min(h, window_height))
        } else {
            (window_width, window_height)
        };
        width = u32::max(width, self.settings.min_size.0);
        height = u32::max(height, self.settings.min_size.1);
        let mut style = canvas.style().css_text();
        let w = format!("width:{}px;", width);
        let h = format!("height:{}px;", height);
        if let Some(start) = style.find("width") {
            let mut end = start;
            for char in style[start..].chars() {
                end += 1;
                if char == ';' {
                    break;
                }
            }
            style.replace_range(start..end, &w);
        } else {
            style.push_str(&w);
        }
        if let Some(start) = style.find("height") {
            let mut end = start;
            for char in style[start..].chars() {
                end += 1;
                if char == ';' {
                    break;
                }
            }
            style.replace_range(start..end, &h);
        } else {
            style.push_str(&h);
        }

        canvas.style().set_css_text(&style);
        let device_pixel_ratio = self.pixels_per_point();
        canvas.set_width((device_pixel_ratio * width as f64) as u32);
        canvas.set_height((device_pixel_ratio * height as f64) as u32);
        Ok(())
    }

    fn add_context_menu_event_listener(&mut self) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
            event.stop_propagation();
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .set_oncontextmenu(Some(closure.as_ref().unchecked_ref()));
        self.closures_with_event.push(closure);
        Ok(())
    }

    fn add_resize_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move || {
            input.borrow_mut().request_animation_frame();
        }) as Box<dyn FnMut()>);
        self.canvas()?
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("resize".to_string(), format!("{:?}", e))
            })?;
        self.closures.push(closure);
        Ok(())
    }

    fn add_mouseleave_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                input.mouse_pressed = None;
                input.events.push(Event::MouseLeave);
                event.stop_propagation();
                event.prevent_default();

                input.request_animation_frame();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("mouseleave".to_string(), format!("{:?}", e))
            })?;
        self.closures_with_mouseevent.push(closure);
        Ok(())
    }

    fn add_mouseenter_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                input.events.push(Event::MouseEnter);
                event.stop_propagation();
                event.prevent_default();

                input.request_animation_frame();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .add_event_listener_with_callback("mouseenter", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("mouseenter".to_string(), format!("{:?}", e))
            })?;
        self.closures_with_mouseevent.push(closure);
        Ok(())
    }

    fn add_mousedown_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                let button = match event.button() {
                    0 => Some(MouseButton::Left),
                    1 => Some(MouseButton::Middle),
                    2 => Some(MouseButton::Right),
                    _ => None,
                };
                if let Some(button) = button {
                    let modifiers = input.modifiers;
                    input.mouse_pressed = Some(button);
                    input.events.push(Event::MousePress {
                        button,
                        position: (event.offset_x() as f64, event.offset_y() as f64),
                        modifiers,
                        handled: false,
                    });
                };
                event.stop_propagation();
                event.prevent_default();

                input.request_animation_frame();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("mousedown".to_string(), format!("{:?}", e))
            })?;
        self.closures_with_mouseevent.push(closure);
        Ok(())
    }

    fn add_mouseup_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                let button = match event.button() {
                    0 => Some(MouseButton::Left),
                    1 => Some(MouseButton::Middle),
                    2 => Some(MouseButton::Right),
                    _ => None,
                };
                if let Some(button) = button {
                    let modifiers = input.modifiers;
                    input.mouse_pressed = None;
                    input.events.push(Event::MouseRelease {
                        button,
                        position: (event.offset_x() as f64, event.offset_y() as f64),
                        modifiers,
                        handled: false,
                    });
                };
                event.stop_propagation();
                event.prevent_default();

                input.request_animation_frame();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("mouseup".to_string(), format!("{:?}", e))
            })?;
        self.closures_with_mouseevent.push(closure);
        Ok(())
    }

    fn add_mousemove_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                let delta = if let Some((x, y)) = input.last_position {
                    ((event.offset_x() - x) as f64, (event.offset_y() - y) as f64)
                } else {
                    (0.0, 0.0)
                };
                let modifiers = input.modifiers;
                let button = input.mouse_pressed;
                input.events.push(Event::MouseMotion {
                    button,
                    delta,
                    position: (event.offset_x() as f64, event.offset_y() as f64),
                    modifiers,
                    handled: false,
                });
                input.last_position = Some((event.offset_x(), event.offset_y()));
                event.stop_propagation();
                event.prevent_default();

                input.request_animation_frame();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("mousemove".to_string(), format!("{:?}", e))
            })?;
        self.closures_with_mouseevent.push(closure);
        Ok(())
    }

    fn add_mousewheel_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                let modifiers = input.modifiers;
                input.events.push(Event::MouseWheel {
                    delta: (event.delta_x() as f64, -event.delta_y() as f64),
                    position: (event.offset_x() as f64, event.offset_y() as f64),
                    modifiers,
                    handled: false,
                });
                event.stop_propagation();
                event.prevent_default();
                input.request_animation_frame();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
            .map_err(|e| CanvasError::EventListenerFail("wheel".to_string(), format!("{:?}", e)))?;
        self.closures_with_wheelevent.push(closure);
        Ok(())
    }

    fn add_touchstart_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                if event.touches().length() == 1 {
                    let touch = event.touches().item(0).unwrap();
                    let modifiers = input.modifiers;
                    input.mouse_pressed = Some(MouseButton::Left);
                    input.events.push(Event::MousePress {
                        button: MouseButton::Left,
                        position: (touch.page_x() as f64, touch.page_y() as f64),
                        modifiers,
                        handled: false,
                    });
                    input.last_position = Some((touch.page_x(), touch.page_y()));
                    input.last_zoom = None;
                } else if event.touches().length() == 2 {
                    let touch0 = event.touches().item(0).unwrap();
                    let touch1 = event.touches().item(1).unwrap();
                    let zoom = f64::sqrt(
                        f64::powi((touch0.page_x() - touch1.page_x()) as f64, 2)
                            + f64::powi((touch0.page_y() - touch1.page_y()) as f64, 2),
                    );
                    input.last_zoom = Some(zoom);
                    input.last_position = None;
                } else {
                    input.last_zoom = None;
                    input.last_position = None;
                }
                event.stop_propagation();
                event.prevent_default();

                input.request_animation_frame();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("touchstart".to_string(), format!("{:?}", e))
            })?;
        self.closures_with_touchevent.push(closure);
        Ok(())
    }

    fn add_touchend_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                if let Some((x, y)) = input.last_position {
                    let modifiers = input.modifiers;
                    input.mouse_pressed = None;
                    input.events.push(Event::MouseRelease {
                        button: MouseButton::Left,
                        position: (x as f64, y as f64),
                        modifiers,
                        handled: false,
                    });
                    input.last_position = None;
                }
                input.last_zoom = None;
                event.stop_propagation();
                event.prevent_default();

                input.request_animation_frame();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("touchend".to_string(), format!("{:?}", e))
            })?;
        self.closures_with_touchevent.push(closure);
        Ok(())
    }

    fn add_touchmove_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                if event.touches().length() == 1 {
                    let touch = event.touches().item(0).unwrap();
                    if let Some((x, y)) = input.last_position {
                        let modifiers = input.modifiers;
                        let button = input.mouse_pressed;
                        input.events.push(Event::MouseMotion {
                            button,
                            delta: ((touch.page_x() - x) as f64, (touch.page_y() - y) as f64),
                            position: (touch.page_x() as f64, touch.page_y() as f64),
                            modifiers,
                            handled: false,
                        });
                    }
                    input.last_position = Some((touch.page_x(), touch.page_y()));
                    input.last_zoom = None;
                } else if event.touches().length() == 2 {
                    let touch0 = event.touches().item(0).unwrap();
                    let touch1 = event.touches().item(1).unwrap();
                    let zoom = f64::sqrt(
                        f64::powi((touch0.page_x() - touch1.page_x()) as f64, 2)
                            + f64::powi((touch0.page_y() - touch1.page_y()) as f64, 2),
                    );
                    if let Some(old_zoom) = input.last_zoom {
                        let modifiers = input.modifiers;
                        input.events.push(Event::MouseWheel {
                            delta: (0.0, zoom - old_zoom),
                            position: (
                                0.5 * touch0.page_x() as f64 + 0.5 * touch1.page_x() as f64,
                                0.5 * touch0.page_y() as f64 + 0.5 * touch1.page_y() as f64,
                            ),
                            modifiers,
                            handled: false,
                        });
                    }
                    input.last_zoom = Some(zoom);
                    input.last_position = None;
                } else {
                    input.last_zoom = None;
                    input.last_position = None;
                }
                event.stop_propagation();
                event.prevent_default();

                input.request_animation_frame();
            }
        }) as Box<dyn FnMut(_)>);
        self.canvas()?
            .add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("touchmove".to_string(), format!("{:?}", e))
            })?;
        self.closures_with_touchevent.push(closure);
        Ok(())
    }

    fn add_key_down_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                if update_modifiers(&mut input.modifiers, &event) {
                    let modifiers = input.modifiers;
                    input.events.push(Event::ModifiersChange { modifiers });
                    event.stop_propagation();
                    event.prevent_default();
                }
                let key = event.key();
                let modifiers = input.modifiers;
                if let Some(kind) = translate_key(&key) {
                    input.events.push(Event::KeyPress {
                        kind,
                        modifiers,
                        handled: false,
                    });
                    event.stop_propagation();
                    event.prevent_default();
                }
                if !modifiers.ctrl && !modifiers.command && !should_ignore_key(&key) {
                    input.events.push(Event::Text(key));
                    event.stop_propagation();
                    event.prevent_default();

                    input.request_animation_frame();
                }
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .map_err(|e| {
                CanvasError::EventListenerFail("keydown".to_string(), format!("{:?}", e))
            })?;
        self.closures_with_keyboardevent.push(closure);
        Ok(())
    }

    fn add_key_up_event_listener(&mut self, input: Rc<RefCell<Input>>) -> Result<()> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if !event.default_prevented() {
                let mut input = input.borrow_mut();
                if update_modifiers(&mut input.modifiers, &event) {
                    let modifiers = input.modifiers;
                    input.events.push(Event::ModifiersChange { modifiers });
                    event.stop_propagation();
                    event.prevent_default();
                }
                if let Some(kind) = translate_key(&event.key()) {
                    let modifiers = input.modifiers;
                    input.events.push(Event::KeyRelease {
                        kind,
                        modifiers,
                        handled: false,
                    });
                    event.stop_propagation();
                    event.prevent_default();

                    input.request_animation_frame();
                }
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
            .map_err(|e| CanvasError::EventListenerFail("keyup".to_string(), format!("{:?}", e)))?;
        self.closures_with_keyboardevent.push(closure);
        Ok(())
    }
}

#[derive(Serialize)]
struct ContextOptions {
    antialias: bool,
}

struct Input {
    window: Rc<web_sys::Window>,
    render_loop_closure: Option<Closure<dyn FnMut()>>,
    render_requested: bool,
    events: Vec<Event>,
    modifiers: Modifiers,
    last_position: Option<(i32, i32)>,
    last_zoom: Option<f64>,
    mouse_pressed: Option<MouseButton>,
}

impl Input {
    pub fn new(window: Rc<web_sys::Window>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            window,
            render_loop_closure: None,
            render_requested: false,
            events: Vec::new(),
            modifiers: Modifiers::default(),
            last_position: None,
            last_zoom: None,
            mouse_pressed: None,
        }))
    }

    pub fn start_frame(&mut self) -> Vec<Event> {
        let events = self.events.clone();
        self.events.clear();
        self.render_requested = false;
        events
    }

    pub fn request_animation_frame(&mut self) {
        if !self.render_requested {
            self.render_requested = true;
            self.window
                .request_animation_frame(
                    self.render_loop_closure
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                )
                .expect("Unable to request a new frame.");
        }
    }
}

fn update_modifiers(modifiers: &mut Modifiers, event: &web_sys::KeyboardEvent) -> bool {
    let old = modifiers.clone();
    *modifiers = Modifiers {
        alt: event.alt_key(),
        ctrl: event.ctrl_key(),
        shift: event.shift_key(),
        command: event.ctrl_key() || event.meta_key(),
    };
    old.alt != modifiers.alt
        || old.ctrl != modifiers.ctrl
        || old.shift != modifiers.shift
        || old.command != modifiers.command
}

fn translate_key(key: &str) -> Option<Key> {
    use Key::*;
    Some(match key {
        "ArrowDown" => ArrowDown,
        "ArrowLeft" => ArrowLeft,
        "ArrowRight" => ArrowRight,
        "ArrowUp" => ArrowUp,

        "Esc" | "Escape" => Escape,
        "Tab" => Tab,
        "Backspace" => Backspace,
        "Enter" => Enter,
        "Space" => Space,

        "Help" | "Insert" => Insert,
        "Delete" => Delete,
        "Home" => Home,
        "End" => End,
        "PageUp" => PageUp,
        "PageDown" => PageDown,

        "0" => Num0,
        "1" => Num1,
        "2" => Num2,
        "3" => Num3,
        "4" => Num4,
        "5" => Num5,
        "6" => Num6,
        "7" => Num7,
        "8" => Num8,
        "9" => Num9,

        "a" | "A" => A,
        "b" | "B" => B,
        "c" | "C" => C,
        "d" | "D" => D,
        "e" | "E" => E,
        "f" | "F" => F,
        "g" | "G" => G,
        "h" | "H" => H,
        "i" | "I" => I,
        "j" | "J" => J,
        "k" | "K" => K,
        "l" | "L" => L,
        "m" | "M" => M,
        "n" | "N" => N,
        "o" | "O" => O,
        "p" | "P" => P,
        "q" | "Q" => Q,
        "r" | "R" => R,
        "s" | "S" => S,
        "t" | "T" => T,
        "u" | "U" => U,
        "v" | "V" => V,
        "w" | "W" => W,
        "x" | "X" => X,
        "y" | "Y" => Y,
        "z" | "Z" => Z,

        _ => return None,
    })
}

fn should_ignore_key(key: &str) -> bool {
    let is_function_key = key.starts_with('F') && key.len() > 1;
    is_function_key
        || matches!(
            key,
            "Alt"
                | "ArrowDown"
                | "ArrowLeft"
                | "ArrowRight"
                | "ArrowUp"
                | "Backspace"
                | "CapsLock"
                | "ContextMenu"
                | "Control"
                | "Delete"
                | "End"
                | "Enter"
                | "Esc"
                | "Escape"
                | "Help"
                | "Home"
                | "Insert"
                | "Meta"
                | "NumLock"
                | "PageDown"
                | "PageUp"
                | "Pause"
                | "ScrollLock"
                | "Shift"
                | "Tab"
        )
}
