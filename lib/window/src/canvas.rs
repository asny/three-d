
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

    pub fn render_loop<F>(&mut self, mut render: F)
        where F: FnMut(&mut Window)
    {
        render(self);
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
