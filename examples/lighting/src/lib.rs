
include!("../example.rs");

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue>
{
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("Logging works!");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    run().await;
    Ok(())
}
