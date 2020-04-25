use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Vec2D {
  pub x: f32,
  pub y: f32
}

#[wasm_bindgen]
impl Vec2D {
  pub fn new(x: f32, y: f32) -> Vec2D {
    Vec2D {
      x: x, y: y
    }
  }
}

#[wasm_bindgen]
pub struct Particle {
  pub pos: Vec2D,
  pub vel: Vec2D,
  pub mass: f32, // TODO(shadaj): check units
  pub force_acc: Vec2D
}

#[wasm_bindgen]
impl Particle {
  pub fn new(x: f32, y: f32) -> Particle {
    Particle {
      pos: Vec2D::new(x, y),
      vel: Vec2D::new(0.0, 0.0),
      mass: 1.0,
      force_acc: Vec2D::new(0.0, 0.0)
    }
  }
}
