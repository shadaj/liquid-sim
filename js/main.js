import { Renderer, World, Particle, tick } from "../pkg/index.js";

const canvas_elem = document.getElementById("canvas");
const fps_elem = document.getElementById("fps");

const world = World.new();
const renderer = Renderer.new(canvas_elem);

for (let i = 0; i < 250; i++) {
  world.add_particle(Particle.new(Math.random() * 100, Math.random() * 100));
}

let last_timestamp = null;
function on_frame(timestamp) {
  if (!last_timestamp) last_timestamp = timestamp;
  let dt = (timestamp - last_timestamp) / 1000;
  fps_elem.innerHTML = `FPS: ${(1 / dt).toFixed(2)}`;
  tick(renderer, world, dt);

  last_timestamp = timestamp;
  window.requestAnimationFrame(on_frame);
}

tick(renderer, world, 0);
setTimeout(() => {
  window.requestAnimationFrame(on_frame);
}, 1000);
