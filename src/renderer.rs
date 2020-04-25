use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use js_sys::Float32Array;

use crate::world::World;

#[wasm_bindgen]
pub struct Renderer {
  program: WebGlProgram,
  context: WebGlRenderingContext
}

#[wasm_bindgen]
impl Renderer {
  pub fn new(elem: web_sys::HtmlCanvasElement) -> Renderer {
    let context = elem
      .get_context("webgl")
      .unwrap()
      .unwrap()
      .dyn_into::<WebGlRenderingContext>().unwrap();
    
    let vert_shader = compile_shader(
      &context,
      WebGlRenderingContext::VERTEX_SHADER,
      r#"
      attribute vec4 position;
      void main() {
        gl_Position = position;
      }
    "#,
    ).unwrap();

    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision highp float;
        uniform vec2 particles[2 * 256];
        uniform vec2 screen_size;
        uniform float particle_count;
        void main() {
          float x = gl_FragCoord.x / screen_size.x;
          float y = gl_FragCoord.y / screen_size.y;
          vec2 my_pos = vec2(x, y);

          gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
          int particle_count_int = int(particle_count);

          for (int i = 0; i < 256; i++) {
            if (i >= particle_count_int) {
              break;
            }

            vec2 particle_pos = particles[i];
            float r = length(particle_pos - my_pos);
            if (r <= 0.05) {
              gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);
            } 
          }
        }
    "#,
    ).unwrap();

    let program = link_program(&context, &vert_shader, &frag_shader).unwrap();
    context.use_program(Some(&program));

    let buffer = context.create_buffer().ok_or("failed to create buffer").unwrap();
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    Renderer {
      program: program,
      context: context
    }
  }

  pub(crate) fn render(&self, world: &World) {
    let data = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];
    let data_array = unsafe {
      Float32Array::view(&data)
    };

    self.context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &data_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    let position_handle = self.context.get_attrib_location(&self.program, "position") as u32;
    self.context.enable_vertex_attrib_array(position_handle);

    self.context.vertex_attrib_pointer_with_i32(
      0, 2, // 2 elements: x and y position
      WebGlRenderingContext::FLOAT, false,
      0, 0
    );

    let particles_handle = self.context.get_uniform_location(&self.program, "particles");
    let mut vert_array: Vec<f32> = vec![];
    for particle in &world.particles {
      vert_array.push(particle.pos.x);
      vert_array.push(particle.pos.y);
    }

    self.context.uniform2fv_with_f32_array(
      particles_handle.as_ref(),
      &vert_array
    );

    let screen_size_handle = self.context.get_uniform_location(&self.program, "screen_size");
    self.context.uniform2f(
      screen_size_handle.as_ref(),
      300.0, 300.0
    );

    let particle_count_handle = self.context.get_uniform_location(&self.program, "particle_count");
    self.context.uniform1f(
      particle_count_handle.as_ref(),
      world.particles.len() as f32
    );

    self.context.clear_color(0.0, 0.0, 0.0, 1.0);
    self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    self.context.draw_arrays(
      WebGlRenderingContext::TRIANGLE_STRIP,
      0,
      4,
    );
  }
}

pub fn compile_shader(
  context: &WebGlRenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
      .create_shader(shader_type)
      .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);

  if context
      .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
      .as_bool()
      .unwrap_or(false)
  {
      Ok(shader)
  } else {
      Err(context
          .get_shader_info_log(&shader)
          .unwrap_or_else(|| String::from("Unknown error creating shader")))
  }
}

pub fn link_program(
  context: &WebGlRenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = context
      .create_program()
      .ok_or_else(|| String::from("Unable to create shader object"))?;

  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);

  if context
      .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
      .as_bool()
      .unwrap_or(false)
  {
      Ok(program)
  } else {
      Err(context
          .get_program_info_log(&program)
          .unwrap_or_else(|| String::from("Unknown error creating program object")))
  }
}
