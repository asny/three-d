
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

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

    pub fn render_loop<F: 'static, G: 'static>(&mut self, mut render: F, mut handle_events: G)
        where F: FnMut(), G: FnMut(u32)
    {
        use std::cell::RefCell;
        use std::rc::Rc;

        let window = web_sys::window().expect("no global `window` exists");

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut i = 0;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            i += 1;
            if i > 300 {
                body().set_text_content(Some("All done!"));

                // Drop our handle to this closure so that it will get cleaned up once we return.
                let _ = f.borrow_mut().take();
                return;
            }
            render();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());

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

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}