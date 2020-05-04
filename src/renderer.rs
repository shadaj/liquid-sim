use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};
use js_sys::Float32Array;

use crate::world::World;

#[wasm_bindgen]
pub struct Renderer {
  program: WebGlProgram,
  context: WebGl2RenderingContext
}

#[wasm_bindgen]
impl Renderer {
  pub fn new(elem: web_sys::HtmlCanvasElement) -> Renderer {
    let context = elem
      .get_context("webgl2")
      .unwrap()
      .unwrap()
      .dyn_into::<WebGl2RenderingContext>().unwrap();
    
    let vert_shader = compile_shader(
      &context,
      WebGl2RenderingContext::VERTEX_SHADER,
      r#"
      attribute vec4 position;
      void main() {
        gl_Position = position;
      }
    "#,
    ).unwrap();

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"
        precision highp float;
        uniform vec2 screen_size;
        uniform float particle_count;
        uniform sampler2D particles_texture;

        void main() {
          float x = gl_FragCoord.x / screen_size.x;
          float y = gl_FragCoord.y / screen_size.y;
          vec2 my_pos = vec2(x, y);
          float r = 0.01; // hardcoded radius for each particle
          float threshhold = 1.0;

          gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
          int particle_count_int = int(particle_count);

          float v = 0.0;
          for (int i = 0; i < 100000; i++) { // 100000 must be higher than max number of particles
            if (i >= particle_count_int) {
              break;
            }

            float particle_x = texture2D(particles_texture, vec2(0.25, (float(i) + 0.5) / particle_count)).x;
            float particle_y = texture2D(particles_texture, vec2(0.75, (float(i) + 0.5) / particle_count)).x;

            vec2 particle_pos = vec2(particle_x, particle_y);
            float dist = length(particle_pos - my_pos);

            // if (dist < r) {
            //   gl_FragColor = vec4(0.0, 0.0, 1.0, 1.0);
            //   break;
            // }
            
            v += r*r / (dist * dist);

            if (v > threshhold) {
              gl_FragColor = vec4(0.0, 0.0, 1.0, 1.0);
              break;
            }
          }
        }
    "#,
    ).unwrap();

    let program = link_program(&context, &vert_shader, &frag_shader).unwrap();
    context.use_program(Some(&program));

    let buffer = context.create_buffer().ok_or("failed to create buffer").unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    let particles_texture = context.create_texture();
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, particles_texture.as_ref());

    context.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_MIN_FILTER,
      WebGl2RenderingContext::NEAREST as i32
    );

    context.tex_parameteri(
      WebGl2RenderingContext::TEXTURE_2D,
      WebGl2RenderingContext::TEXTURE_MAG_FILTER,
      WebGl2RenderingContext::NEAREST as i32
    );

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
      WebGl2RenderingContext::ARRAY_BUFFER,
      &data_array,
      WebGl2RenderingContext::STATIC_DRAW,
    );

    let position_handle = self.context.get_attrib_location(&self.program, "position") as u32;
    self.context.enable_vertex_attrib_array(position_handle);

    self.context.vertex_attrib_pointer_with_i32(
      0, 2, // 2 elements: x and y position
      WebGl2RenderingContext::FLOAT, false,
      0, 0
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

    let mut vert_array: Vec<f32> = vec![];
    for particle in &world.particles {
      vert_array.push(particle.pos.x / 100.0);
      vert_array.push(particle.pos.y / 100.0);
    }

    let particles_array = unsafe {
      Float32Array::view(&vert_array)
    };

    self.context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
      WebGl2RenderingContext::TEXTURE_2D,
      0,
      WebGl2RenderingContext::R32F as i32,
      2,
      world.particles.len() as i32,
      0,
      WebGl2RenderingContext::RED,
      WebGl2RenderingContext::FLOAT,
      &particles_array,
      0
    ).unwrap();

    self.context.uniform1i(
      self.context.get_uniform_location(&self.program, "particles_texture").as_ref(),
      0
    );

    self.context.clear_color(0.0, 0.0, 0.0, 1.0);
    self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    self.context.draw_arrays(
      WebGl2RenderingContext::TRIANGLE_STRIP,
      0,
      4,
    );
  }
}

pub fn compile_shader(
  context: &WebGl2RenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
      .create_shader(shader_type)
      .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);

  if context
      .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
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
  context: &WebGl2RenderingContext,
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
      .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
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
