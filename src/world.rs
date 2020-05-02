use wasm_bindgen::prelude::*;

use crate::particle::*;
use std::panic;
use std::collections::HashMap;

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub struct World {
  pub(crate) particles: Vec<Particle>,
  pub(crate) cell_size: f32,
  pub(crate) particle_map: HashMap<f32, Vec<Particle>>
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
      particles: vec![],
      cell_size: 0.1,
      particle_map: HashMap::new()
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

  fn build_spatial_map(&mut self){
    // build hash map with a list of particles with the same hash value 
    // (key = hash_position, value = list of particles)
    self.particle_map.clear();
    for particle in &self.particles {
      let hash = self.hash_position(&particle);
      match self.particle_map.get_mut(hash){
        Some(&list) => list.push(particle),
        None => self.particle_map.insert_mut(&hash, vec![particle])
       }
    }
  }
  fn self_collide(self, p: &Particle){
    // calculate hash value and for all particles with the same hash value, 
    // check if the particles collide
    let key = self.hash_position(p.position);
    let check = self.particles.get(key);
    
    // sudo code that needs to become rust code
    // Vector3D correct = Vector3D();
    // let mut count = 0;
    // for(PointMass *c: check){
    //     //check that it's not the point mass itself
    //     if(c == &pm)
    //         continue;
    //     //calc for correction vector
    //     Vector3D diffVec = pm.position - (*c).position;
    //     double dist = diffVec.norm();
    //     if(dist > (2.0 * thickness))
    //         continue;
    //     count += 1;
    //     correct += (2 * thickness - dist) * diffVec.unit();
    // }
    // //average correction vector scaled down by simulation steps
    // if(count == 0)
    //     return;
    // correct /= count;
    // correct /= simulation_steps;
    // pm.position = pm.position + correct;
  }
  fn hash_position(self, p: &Particle) -> f32 {
    let width = 1.0/self.cell_size;
    let grid_cell = (p.pos.x/self.cell_size).floor() + (width * (p.pos.y/self.cell_size).floor());
    grid_cell
  }
  

  pub(crate) fn update(&mut self, dt: f32) {
    self.apply_gravity();
    self.update_velocities(dt);
    self.update_positions(dt);
    self.build_spatial_map();
    for &mut particle in &mut self.particles{
      self.self_collide(&particle)
    }
    self.wall_collisions();
  }
}
