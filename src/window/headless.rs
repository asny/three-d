use crate::{Context, CoreError};
use glutin_029::{
    dpi::PhysicalSize, event_loop::EventLoop, ContextBuilder, ContextCurrentState, CreationError,
    NotCurrent, PossiblyCurrent,
};
use std::rc::Rc;
use thiserror::Error;

///
/// Error associated with a headless context.
///
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum HeadlessError {
    #[error("glutin error")]
    GlutinCreationError(#[from] glutin_029::CreationError),
    #[error("glutin error")]
    GlutinContextError(#[from] glutin_029::ContextError),
    #[error("error in three-d")]
    ThreeDError(#[from] CoreError),
}

///
/// A headless graphics context, ie. a graphics context that is not associated with any window.
/// For a graphics context associated with a window, see [WindowedContext](crate::WindowedContext).
/// Can only be created on native, not on web.
///
#[derive(Clone)]
pub struct HeadlessContext {
    context: Context,
    _glutin_context: Rc<glutin_029::Context<PossiblyCurrent>>,
}

impl HeadlessContext {
    ///
    /// Creates a new headless graphics context.
    ///
    #[allow(unsafe_code)]
    pub fn new() -> Result<Self, HeadlessError> {
        let cb = ContextBuilder::new();
        let (glutin_context, _el) = build_context(cb)?;
        let glutin_context = unsafe { glutin_context.make_current().map_err(|(_, e)| e)? };
        let context = Context::from_gl_context(std::sync::Arc::new(unsafe {
            crate::context::Context::from_loader_function(|s| {
                glutin_context.get_proc_address(s) as *const _
            })
        }))?;
        Ok(Self {
            context,
            _glutin_context: Rc::new(glutin_context),
        })
    }
}

impl std::ops::Deref for HeadlessContext {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

/*#[cfg(target_os = "linux")]
fn build_context_surfaceless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<glutin_029::Context<NotCurrent>, CreationError> {
    use glutin_029::platform::unix::HeadlessContextExt;
    cb.build_surfaceless(&el)
}*/

fn build_context_headless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<glutin_029::Context<NotCurrent>, CreationError> {
    let size_one = PhysicalSize::new(1, 1);
    cb.build_headless(&el, size_one)
}

#[cfg(target_os = "linux")]
fn build_context_osmesa<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<glutin_029::Context<NotCurrent>, CreationError> {
    use glutin_029::platform::unix::HeadlessContextExt;
    let size_one = PhysicalSize::new(1, 1);
    cb.build_osmesa(size_one)
}

#[cfg(target_os = "linux")]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<(glutin_029::Context<NotCurrent>, EventLoop<()>), CreationError> {
    // On unix operating systems, you should always try for surfaceless first,
    // and if that does not work, headless (pbuffers), and if that too fails,
    // finally osmesa.
    //
    // If willing, you could attempt to use hidden windows instead of os mesa,
    // but note that you must handle events for the window that come on the
    // events loop.
    let el = EventLoop::new();

    /*
    let err1 = match build_context_surfaceless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };*/

    let err2 = match build_context_headless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    let err3 = match build_context_osmesa(cb) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    Err(err2)
}

#[cfg(not(target_os = "linux"))]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<(glutin_029::Context<NotCurrent>, EventLoop<()>), CreationError> {
    let el = EventLoop::new();
    build_context_headless(cb.clone(), &el).map(|ctx| (ctx, el))
}
