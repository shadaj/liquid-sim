import { Renderer, World, Particle, tick } from "../pkg/index.js";

const canvas_elem = document.getElementById("canvas");
const fps_elem = document.getElementById("fps");

const world = World.new();
const renderer = Renderer.new(canvas_elem);

var mouseX = 0;
var mouseY = 0;
var isMousedown = false;

for (let i = 0; i < 100; i++) {
  world.add_particle(Particle.new(Math.random() * 100, Math.random() * 100, 0.75 + Math.random() * 0.25));
}

const fixed_dt = 1 / 60;

let last_timestamp = null;
function on_frame(timestamp) {

  if (!last_timestamp) last_timestamp = timestamp;
  let dt = (timestamp - last_timestamp) / 1000;
  fps_elem.innerHTML = `FPS: ${(1 / dt).toFixed(2)}, ${(fixed_dt / dt).toFixed(2)}x Real Time`;
  tick(renderer, world, fixed_dt);


  //user interaction
  if(isMousedown) {
    world.apply_mouse_force(mouseX*100, mouseY*100, dt);
  }

  last_timestamp = timestamp;
  window.requestAnimationFrame(on_frame);
}

//////////////////////////////////////////////////////
// user interaction clicking/screen interaction ADDITIONS

function getCursorPosition(canvas, event) {
  const rect = canvas.getBoundingClientRect();
  const x = (event.clientX - rect.left) / rect.width;
  const y = (rect.height - (event.clientY - rect.top)) / rect.height;
  return {x, y};
}

canvas_elem.addEventListener('mousemove', function(e) {
  const pos = getCursorPosition(canvas, e);
  mouseX = pos.x;
  mouseY = pos.y;
})

document.addEventListener('mousedown', function(e) {
  isMousedown = true;
  const pos = getCursorPosition(canvas, e);
  mouseX = pos.x;
  mouseY = pos.y;
})

document.addEventListener('mouseup', function(e) {
  isMousedown = false;
})

canvas_elem.addEventListener('touchmove', function(e) {
  e.preventDefault();
  const pos = getCursorPosition(canvas_elem, e.changedTouches[0]);
  mouseX = pos.x;
  mouseY = pos.y;
})

canvas_elem.addEventListener('touchstart', function(e) {
  e.preventDefault();
  const pos = getCursorPosition(canvas_elem, e.changedTouches[0]);
  mouseX = pos.x;
  mouseY = pos.y;
  isMousedown = true;
})

canvas_elem.addEventListener('touchend', function(e) {
  e.preventDefault();
  isMousedown = false;
})

//End User interaction additions
//////////////////////////////////////////////////////

tick(renderer, world, 0);
setTimeout(() => {
  window.requestAnimationFrame(on_frame);
}, 1000);
