import { Renderer, World, Particle, tick } from "../pkg/index.js";

const renderer = Renderer.new(document.getElementById("canvas"))
const world = World.new();

world.add_particle(Particle.new(0.25, 0.25));

world.add_particle(Particle.new(0.5, 0.5));

world.add_particle(Particle.new(0.75, 0.75));

let last_timestamp = null;
function on_frame(timestamp) {
  if (!last_timestamp) last_timestamp = timestamp;
  let dt = (timestamp - last_timestamp) / 1000;
  tick(renderer, world, dt);

  last_timestamp = timestamp;
  window.requestAnimationFrame(on_frame);
}

tick(renderer, world, 0);
setTimeout(() => {
  window.requestAnimationFrame(on_frame);
}, 1000);
