use wasm_bindgen::prelude::*;

mod particle;
mod world;
mod renderer;

use crate::world::*;
use crate::renderer::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const MAX_DT: f32 = 0.01;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn tick(renderer: &Renderer, world: &mut World, dt: f32) -> Result<(), JsValue> {
    world.update(dt);

    renderer.render(world);

    Ok(())
}
