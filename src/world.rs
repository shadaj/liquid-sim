use wasm_bindgen::prelude::*;

use crate::particle::*;
use std::panic;
use std::collections::HashMap;

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub struct World {
  pub(crate) particles: Vec<Particle>,
  neighbors: HashMap<(i32, i32), Vec<usize>>,
}

const PARTICLE_RADIUS: f32 = 1.0;

const GRAVITY: f32 = 40.0;
const BOUNDARY_COR: f32 = 0.01; // Coefficient of restitution
const BOUNDARY_MIN_DV: f32 = 0.005; // If particle is too slow, it is accelerated to atleast this much

const INTERACTION_RADIUS: f32 = 9.0;
const STIFFNESS: f32 = 35.0;
const REST_DENSITY: f32 = 5.0;
const STIFFNESS_NEAR: f32 = 100.0;

const VISCOSITY_LINEAR: f32 = 0.0;
const VISCOSITY_QUAD: f32 = -0.005;

const CELL_SIZE: f32 = INTERACTION_RADIUS * 4.0;
// static particle_map: HashMap = HashMap::new();

#[wasm_bindgen]
impl World {
  pub fn new() -> World {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    World {
      particles: vec![],
      neighbors: HashMap::new()
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

  fn apply_viscosity(&mut self, dt: f32) {
    //self.neighbors = self.build_spatial_map();
    let spatial_map = &self.neighbors;
    for idx in 0..self.particles.len() {
      let mut cur_particle = self.particles[idx];
      let key = World::hash_position(&cur_particle);

      let neighbors_default = vec![];
      let neighbors = spatial_map.get(&key).unwrap_or(&neighbors_default);

      for neighbor_idx in neighbors {
        if idx != *neighbor_idx  && idx < *neighbor_idx{
          let neighbor = &self.particles[*neighbor_idx];
          let mut vecCN = neighbor.pos - cur_particle.pos;
          let vel_vect = cur_particle.vel - neighbor.vel;
          //manually doing vec multiplication
          let mut vel_in  = vel_vect.x * vecCN.x +  vel_vect.y * vecCN.y;

          if vel_in > 0.0  && vecCN.length() < 0.1{
            let length = vecCN.length();
            vel_in = vel_in/length;
            vecCN.x = vecCN.x/length; vecCN.y = vecCN.y/length;
            let q = length/INTERACTION_RADIUS;
            let factor = 0.5 * dt * (1.0 - q) * (VISCOSITY_LINEAR * vel_in + VISCOSITY_QUAD * vel_in * vel_in);
            let I = Vec2D::new(factor * vecCN.x, factor * vecCN.y);
            cur_particle.vel = cur_particle.vel - I;
          }
        }
      }
      self.particles[idx] = cur_particle;
    }
//     for each Particle p
// 3: for each Particle n ∈ neighborsp
// 4: vp,n ← n.pos − p.pos
// 5: velinward ← (p.vel − n.vel) · vp,n
// 6: if velinward > 0
// 7: length ← | vp,n|
// 8: velinward ← velinward/length
// 9: vp,n ← vp,n/length
// 10: q ← length/radius
// 11: I ← 0.5 ∗ timeStep ∗ (1 − q) ∗ (σ ∗ velinward + β ∗ vel2
// inward) ∗ vp,n
// 12: p.vel ← p.vel − I
  
  }

  fn update_velocities(&mut self, dt: f32) {
    for particle in &mut self.particles {
      particle.vel.x += (particle.force_acc.x / particle.mass) * dt;
      particle.vel.y += (particle.force_acc.y / particle.mass) * dt;
    }
  }

  //worst case change this function to save prev position and at the end compute next velocity
  fn update_positions(&mut self, dt: f32) {
    for particle in &mut self.particles {
      particle.pos.x += particle.vel.x * dt;
      particle.pos.y += particle.vel.y * dt;
    }
  }

  fn wall_collisions(&mut self){
    for particle in &mut self.particles {
      if particle.pos.x < PARTICLE_RADIUS {
        particle.vel.x = ( BOUNDARY_MIN_DV).max(-BOUNDARY_COR * particle.vel.x/2.0);
        particle.pos.x = PARTICLE_RADIUS;
      } else if particle.pos.x > (100.0 - PARTICLE_RADIUS) {
        particle.vel.x = (-BOUNDARY_MIN_DV).min(-BOUNDARY_COR * particle.vel.x/2.0);
        particle.pos.x =  100.0 - PARTICLE_RADIUS;
      } else if particle.pos.y < PARTICLE_RADIUS {
        particle.vel.y = ( BOUNDARY_MIN_DV).max(-BOUNDARY_COR * particle.vel.y/2.0);
        particle.pos.y = PARTICLE_RADIUS;
      } else if particle.pos.y > (100.0 - PARTICLE_RADIUS) {
        particle.vel.y = (-BOUNDARY_MIN_DV).min(-BOUNDARY_COR * particle.vel.y/2.0);
        particle.pos.y =  100.0 - PARTICLE_RADIUS;
      }
      // if particle.vel.x < 0.0001 && particle.vel.x > 0.0001{
      //   particle.vel.x = 0.0;
      // }
      // if particle.vel.y < 0.0001 && particle.vel.y > 0.0001{
      //   particle.vel.y = 0.0;
      // }
    }
  }

  fn build_spatial_map(&self) -> HashMap<(i32, i32), Vec<usize>> {
    // build hash map with a list of particles with the same hash value 
    // (key = hash_position, value = list of particles)
    let mut particle_map = HashMap::<(i32, i32), Vec<usize>>::new();
    for (idx, particle) in self.particles.iter().enumerate() {
      let hash = World::hash_position(&particle);
      match particle_map.get_mut(&hash){
        Some(list) => list.push(idx),
        None => {
          particle_map.insert(hash, vec![idx]);
          ()
        }
      }
    }

    particle_map
  }

  fn double_density_relaxation(&mut self, dt: f32) {
    // calculate hash value and for all particles with the same hash value, 
    // check if the particles collide
    
    let spatial_map = &self.neighbors;
    
    // use indices to get around double-borrowing
    for idx in 0..self.particles.len() {
      let cur_particle = &self.particles[idx];
      let key = World::hash_position(&cur_particle);

      let mut density = 0.0;
      let mut near_density = 0.0;
      let mut gradient_cache = vec![];
      let neighbors_default = vec![];
      let neighbors = spatial_map.get(&key).unwrap_or(&neighbors_default);

      for neighbor_idx in neighbors {
        if idx != *neighbor_idx {
          let neighbor = &self.particles[*neighbor_idx];
          let distance = (cur_particle.pos - neighbor.pos).length();
          
          let gradient = f32::max(1.0 - distance / INTERACTION_RADIUS, 0.0);

          gradient_cache.push(gradient);
          
          density += gradient * gradient * cur_particle.mass;
          near_density += gradient * gradient * gradient * cur_particle.mass;
        } else {
          gradient_cache.push(0.0);
        }
      }

      let pressure = STIFFNESS * (density - REST_DENSITY) * cur_particle.mass;
      let pressure_near = STIFFNESS_NEAR * near_density * cur_particle.mass;

      let original_pos = cur_particle.pos;
      let cur_particle_mass = cur_particle.mass;

      for (grad_idx, neighbor_idx) in neighbors.iter().enumerate() {
        if idx != *neighbor_idx {
          let neighbor = &self.particles[*neighbor_idx];
          let neighbor_gradient = gradient_cache[grad_idx];
          let magnitude = pressure * neighbor_gradient + pressure_near * neighbor_gradient * neighbor_gradient;
          let direction = (neighbor.pos - original_pos) / (neighbor.pos - original_pos).length();
          
          let force = direction * magnitude;

          let neighbor_mass = neighbor.mass;
          let mass_total = cur_particle_mass + neighbor_mass;

          self.particles[idx].pos = self.particles[idx].pos - 0.5 * force * dt * dt * (neighbor_mass / mass_total);
          self.particles[idx].vel = self.particles[idx].vel - force * dt  * (neighbor_mass / mass_total);

          self.particles[*neighbor_idx].pos = self.particles[*neighbor_idx].pos + 0.5 * force * dt * dt  * (cur_particle_mass / mass_total);
          self.particles[*neighbor_idx].vel = self.particles[*neighbor_idx].vel + force * dt * (cur_particle_mass / mass_total);
        }
      }
    }
  }

  fn hash_position(p: &Particle) -> (i32, i32) {
    ((p.pos.x / CELL_SIZE).floor() as i32, (p.pos.y / CELL_SIZE).floor() as i32)
  }

  pub(crate) fn update(&mut self, dt: f32) {
    self.neighbors = self.build_spatial_map();
    self.apply_gravity();
    self.update_velocities(dt);
    self.apply_viscosity(dt);
    self.update_positions(dt);
    self.double_density_relaxation(dt);
    
    self.wall_collisions();
  }
}
