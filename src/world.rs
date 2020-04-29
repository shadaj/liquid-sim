use wasm_bindgen::prelude::*;

use crate::particle::*;
use std::panic;

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub struct World {
  pub(crate) particles: Vec<Particle>,
  
}

const GRAVITY: f32 = 0.5;
const BOUNDARY_COR: f32 = 0.5; // Coefficient of restitution
const BOUNDARY_MIN_DV: f32 = 0.005; // If particle is too slow, it is accelerated to atleast this much
// static particle_map: HashMap = HashMap::new();

#[wasm_bindgen]
impl World {
  pub fn new() -> World {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

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
      particle.force_acc.x = 0.0;
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

  fn wall_collisions(&mut self){
    for particle in &mut self.particles {
      if particle.pos.x < 0.0 {
        particle.vel.x = ( BOUNDARY_MIN_DV).max(-BOUNDARY_COR * particle.vel.x);
        particle.pos.x = -1.0 * particle.pos.x;
      } else if particle.pos.x > 1.0 {
        particle.vel.x = (-BOUNDARY_MIN_DV).min(-BOUNDARY_COR * particle.vel.x);
        particle.pos.x =  1.0 - (particle.pos.x - 1.0);
      } else if particle.pos.y < 0.0 {
        particle.vel.y = ( BOUNDARY_MIN_DV).max(-BOUNDARY_COR * particle.vel.y);
        particle.pos.y = -1.0 * particle.pos.y;
      } else if particle.pos.y > 1.0 {
        particle.vel.y = (-BOUNDARY_MIN_DV).min(-BOUNDARY_COR * particle.vel.y);
        particle.pos.y =  1.0 - (particle.pos.y - 1.0);
      }
      if particle.vel.x < 0.0001 && particle.vel.x > 0.0001{
        particle.vel.x = 0.0;
      }
      if particle.vel.y < 0.0001 && particle.vel.y > 0.0001{
        particle.vel.y = 0.0;
      }
    }
  }
  

  pub(crate) fn update(&mut self, dt: f32) {
    self.apply_gravity();
    self.update_velocities(dt);
    self.update_positions(dt);
    self.wall_collisions();
  }
}
