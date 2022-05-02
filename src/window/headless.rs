use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::{ContextBuilder, ContextCurrentState, CreationError, NotCurrent};

use crate::Context;
use crate::ThreeDResult;

impl Context {
    ///
    /// Creates a new headless graphics context (a graphics context that is not associated with any window).
    ///
    ///
    pub fn new() -> ThreeDResult<Self> {
        let cb = ContextBuilder::new();
        let (headless_context, _el) = build_context(cb).unwrap();
        let headless_context = unsafe { headless_context.make_current().unwrap() };
        let mut c = Self::from_gl_context(std::rc::Rc::new(unsafe {
            crate::context::Context::from_loader_function(|s| {
                headless_context.get_proc_address(s) as *const _
            })
        }))?;
        c.glutin_context = Some(std::rc::Rc::new(headless_context));
        Ok(c)
    }
}

#[cfg(target_os = "linux")]
fn build_context_surfaceless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<glutin::Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    cb.build_surfaceless(&el)
}

fn build_context_headless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<glutin::Context<NotCurrent>, CreationError> {
    let size_one = PhysicalSize::new(1, 1);
    cb.build_headless(&el, size_one)
}

#[cfg(target_os = "linux")]
fn build_context_osmesa<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<glutin::Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    let size_one = PhysicalSize::new(1, 1);
    cb.build_osmesa(size_one)
}

#[cfg(target_os = "linux")]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<(glutin::Context<NotCurrent>, EventLoop<()>), [CreationError; 3]> {
    // On unix operating systems, you should always try for surfaceless first,
    // and if that does not work, headless (pbuffers), and if that too fails,
    // finally osmesa.
    //
    // If willing, you could attempt to use hidden windows instead of os mesa,
    // but note that you must handle events for the window that come on the
    // events loop.
    use glutin::platform::unix::EventLoopExtUnix;
    let el = EventLoopExtUnix::new_any_thread();

    println!("Trying surfaceless");
    let err1 = match build_context_surfaceless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    println!("Trying headless");
    let err2 = match build_context_headless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    println!("Trying osmesa");
    let err3 = match build_context_osmesa(cb) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    Err([err1, err2, err3])
}

#[cfg(not(target_os = "linux"))]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<(glutin::Context<NotCurrent>, EventLoop<()>), CreationError> {
    let el = EventLoop::new();
    build_context_headless(cb.clone(), &el).map(|ctx| (ctx, el))
}
