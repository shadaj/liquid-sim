use wasm_bindgen::prelude::*;

use crate::particle::*;

#[wasm_bindgen]
pub struct World {
  pub(crate) particles: Vec<Particle>
}

const GRAVITY: f32 = 9.8;

#[wasm_bindgen]
impl World {
  pub fn new() -> World {
    World {
      particles: vec![]
    }
  }

  pub fn add_particle(&mut self, particle: Particle) {
    self.particles.push(particle)
  }

  fn apply_gravity(&mut self) {
    for particle in &mut self.particles {
      // we don't add, we just set to reset the accumulator
      particle.force_acc.y = -GRAVITY * particle.mass;
    }
  }

  fn update_velocities(&mut self, dt: f32) {
    for particle in &mut self.particles {
      particle.vel.x += (particle.force_acc.x / particle.mass) * dt;
      particle.vel.y += (particle.force_acc.y / particle.mass) * dt;
    }
  }

  fn update_positions(&mut self, dt: f32) {
    for particle in &mut self.particles {
      particle.pos.x += particle.vel.x * dt;
      particle.pos.y += particle.vel.y * dt;
    }
  }

  pub(crate) fn update(&mut self, dt: f32) {
    self.apply_gravity();
    self.update_velocities(dt);
    self.update_positions(dt);
  }
}
