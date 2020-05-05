use std::ops::Add;
use std::ops::Mul;
use wasm_bindgen::prelude::*;
use std::ops::{Sub, Div};

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

  pub fn length(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }
}

impl Add for Vec2D {
  type Output = Vec2D;

  fn add(self, other: Vec2D) -> Vec2D {
    Vec2D {
      x: self.x + other.x,
      y: self.y + other.y
    }
  }
}

impl Sub for Vec2D {
  type Output = Vec2D;

  fn sub(self, other: Vec2D) -> Vec2D {
    Vec2D {
      x: self.x - other.x,
      y: self.y - other.y
    }
  }
}

impl Mul<Vec2D> for f32 {
  type Output = Vec2D;

  fn mul(self, other: Vec2D) -> Vec2D {
    Vec2D {
      x: self * other.x,
      y: self * other.y
    }
  }
}

impl Mul<f32> for Vec2D {
  type Output = Vec2D;

  fn mul(self, other: f32) -> Vec2D {
    Vec2D {
      x: self.x * other,
      y: self.y * other
    }
  }
}

impl Div<f32> for Vec2D {
  type Output = Vec2D;

  fn div(self, other: f32) -> Vec2D {
    Vec2D {
      x: self.x / other,
      y: self.y / other
    }
  }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Particle {
  pub pos: Vec2D,
  pub vel: Vec2D,
  pub mass: f32, // TODO(shadaj): check units
  pub force_acc: Vec2D
}

#[wasm_bindgen]
impl Particle {
  pub fn new(x: f32, y: f32, mass: f32) -> Particle {
    Particle {
      pos: Vec2D::new(x, y),
      vel: Vec2D::new(0.0, 0.0),
      mass: mass,
      force_acc: Vec2D::new(0.0, 0.0)
    }
  }
}
