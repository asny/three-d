
use wasm_bindgen::prelude::*;

include!("../../hello_world.rs");

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue>
{
    main();
    Ok(())
}
