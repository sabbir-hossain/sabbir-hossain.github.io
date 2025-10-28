
use std::rc::Rc;

use crate::option::ViewConfig;
use crate::utils::vertex::Vertex;
use super::index::{Text, TextObject}; 

#[derive(Debug, Clone, Copy)]
pub struct SingleQuote {
  unit_x: f32,
  unit_y: f32,
  
  left_bottom_x: f32,
  left_bottom_y: f32,

  previous_length: u16,
}

impl SingleQuote {

  pub const RATIO: f32 = 0.2;

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
        position: [ 
          self.left_bottom_x, 
          self.left_bottom_y + max_y * 0.8, 
          0.0
        ], 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 1
        position: [ 
          self.left_bottom_x, 
          self.left_bottom_y + max_y * 1.15, 
          0.0
        ], 
        // position: self.generate_coordinates(0.0, max_y*text.font_wide_h,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 2
        position: [ 
          self.left_bottom_x + max_x*0.7, 
          self.left_bottom_y + max_y * 1.15,  
          0.0
        ],
        // position: self.generate_coordinates(max_x, max_y*text.font_wide_h*2.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 3
        position: [ 
          self.left_bottom_x + max_x*0.7, 
          self.left_bottom_y + max_y * 0.8, 
          0.0
        ],
        // position: self.generate_coordinates(max_x, 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      }
    ];

    let indices = vec![
      self.generate_indices(1, 0, 3), // 0
      self.generate_indices(1, 3, 2), // 1
    ].concat();
    
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
  
  fn generate_indices(self, a: u16, b: u16, c: u16) -> [u16; 3] {
    [
      self.previous_length + a,
      self.previous_length + b,
      self.previous_length + c
    ]
  }

}

