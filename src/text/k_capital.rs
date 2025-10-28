
use std::rc::Rc;

use crate::option::ViewConfig;
use crate::utils::vertex::Vertex;
use super::index::{Text, TextObject}; 

#[derive(Debug, Clone, Copy)]
pub struct K {
  unit_x: f32,
  unit_y: f32,
  
  left_bottom_x: f32,
  left_bottom_y: f32,

  previous_length: u16,
}

impl K {

  pub const RATIO: f32 = 0.6;

  pub fn new(view_config: Rc<ViewConfig>) -> Self {
    Self {
      unit_x: view_config.unit_x,
      unit_y: view_config.unit_y,
      
      left_bottom_x: 0.0,
      left_bottom_y: 0.0,

      previous_length: 0,
    }
  }

  pub fn get_coordinates(mut self, text: &Text) -> TextObject {
    self.left_bottom_x = text.position.0;
    self.left_bottom_y = text.position.1;
    self.previous_length = text.previous_length;
  
    let max_y = text.font_y * self.unit_y;
    let max_x = text.font_y * Self::RATIO * self.unit_x;
    
    let vertices = vec![
      Vertex { // 0
        position: self.generate_coordinates(0.0, 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 1
        position: self.generate_coordinates(0.0, max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 2
        position: self.generate_coordinates(max_x * text.font_wide_h, max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 3
        position: self.generate_coordinates(max_x * text.font_wide_h, max_y * 0.55,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 4
        position: self.generate_coordinates(max_x * text.font_wide_h, max_y * 0.4,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 5
        position: self.generate_coordinates(max_x * text.font_wide_h, 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 6
        position: self.generate_coordinates(max_x * 0.8, max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 7
        position: self.generate_coordinates(max_x, max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 8
        position: self.generate_coordinates(max_x * text.font_wide_h * 1.5, max_y * 0.5,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 9
        position: self.generate_coordinates(max_x * (1.0 - text.font_wide_h), 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 10
        position: self.generate_coordinates(max_x, 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
    ];

    // log::info!("vertices: {:?}",  vertices.clone().into_iter()
    //   .map(|v| format!("({:.3}, {:.3})", v.position[0] - self.left_bottom_x, v.position[1] - self.left_bottom_y))
    //   .collect::<Vec<_>>()
    //   .join(", "));
    // println!("vertices: {:?}", vertices);
  
    let indices = vec![
      self.generate_indices(0, 2, 1), // 0
      self.generate_indices(0, 5, 2), // 1
      self.generate_indices(2, 3, 1), // 2
      self.generate_indices(3, 7, 6), // 3
      self.generate_indices(3, 4, 7), // 4
      self.generate_indices(4, 9, 8), // 5
      self.generate_indices(8, 9, 10), // 6
    ].concat();
    
    // println!("indices: {:?}", indices);
    let indices_len = indices.len() as u32;
    let vertex_len = vertices.len() as u32;
  
    TextObject {
      vertices: Rc::new(vertices), 
      indices: Rc::new(indices),
      indices_len,
      max_x: max_x + max_x * text.font_space,
      max_y,
      vertex_len
    }
  }
  
  fn generate_coordinates(self, x: f32, y: f32, z: f32) -> [f32; 3] {
    [ 
      // self.left_bottom_x + (self.unit_x * x), 
      // self.left_bottom_y + (self.unit_y * y), 
      // z

      self.left_bottom_x + x, 
      self.left_bottom_y + y, 
      z

      // (self.left_bottom_x + x) * self.unit_x, 
      // (self.left_bottom_y + y) * self.unit_y, 
      // z
    ]
  }

  fn generate_indices(self, a: u16, b: u16, c: u16) -> [u16; 3] {
    [
      self.previous_length + a,
      self.previous_length + b,
      self.previous_length + c
    ]
  }

}

