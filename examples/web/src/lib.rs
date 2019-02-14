
use wasm_bindgen::prelude::*;

include!("../../hello_world.rs");

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue>
{
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main();
    Ok(())
}
