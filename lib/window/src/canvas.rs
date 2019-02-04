
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use std::cell::RefCell;
use std::rc::Rc;
use crate::event::*;

pub struct Window
{
    gl: gl::Gl,
    canvas: web_sys::HtmlCanvasElement
}

impl Window
{
    pub fn new_default(title: &str) -> Window
    {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("webgl2").unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>().unwrap();

        let gl = gl::Gl::new(context);
        Window { gl, canvas }
    }

    pub fn render_loop<F: 'static>(&mut self, mut callback: F)
        where F: FnMut(&Vec<Event>)
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let events = Rc::new(RefCell::new(Vec::new()));

        self.add_mousedown_event_listener(events.clone());
        self.add_mouseup_event_listener(events.clone());
        self.add_mousemove_event_listener(events.clone());

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            callback(&(*events).borrow());
            &(*events).borrow_mut().clear();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    fn add_mousedown_event_listener(&self, events: Rc<RefCell<Vec<Event>>>)
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let button = match event.button() {
                0 => Some(MouseButton::Left),
                1 => Some(MouseButton::Middle),
                2 => Some(MouseButton::Right),
                _ => None
            };
            return if let Some(b) = button {
                (*events).borrow_mut().push(Event::MouseClick {state: State::Pressed, button: b});
            };
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    fn add_mouseup_event_listener(&self, events: Rc<RefCell<Vec<Event>>>)
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let button = match event.button() {
                0 => Some(MouseButton::Left),
                1 => Some(MouseButton::Middle),
                2 => Some(MouseButton::Right),
                _ => None
            };
            return if let Some(b) = button {
                (*events).borrow_mut().push(Event::MouseClick {state: State::Released, button: b});
            };
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    fn add_mousemove_event_listener(&self, events: Rc<RefCell<Vec<Event>>>)
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            (*events).borrow_mut().push(Event::MouseMotion {delta: (event.offset_x() as f64, event.offset_y() as f64)});
        }) as Box<dyn FnMut(_)>);
        self.canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    pub fn size(&self) -> (usize, usize)
    {
        (self.canvas.width() as usize, self.canvas.height() as usize)
    }

    pub fn gl(&self) -> gl::Gl
    {
        self.gl.clone()
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}