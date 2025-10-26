
use std::rc::Rc;

use crate::option::ViewConfig;
use crate::utils::vertex::Vertex;
use super::index::{Text, TextObject}; 

#[derive(Debug, Clone, Copy)]
pub struct Eight {
  unit_x: f32,
  unit_y: f32,
  
  left_bottom_x: f32,
  left_bottom_y: f32,

  previous_length: u16,
}

impl Eight {

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
        position: self.generate_coordinates(0.0, max_y*0.9,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 1
        position: self.generate_coordinates(0.0,  max_y * 0.6,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 2
        position: self.generate_coordinates(max_x*text.font_wide_h, max_y*0.65,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 3
        position: self.generate_coordinates(max_x*0.11, max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 4
        position: self.generate_coordinates(max_x*text.font_wide_h, max_y*0.85,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 5
        position: self.generate_coordinates(max_x*text.font_wide_h*1.3, max_y*(1.0-text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 6
        position: self.generate_coordinates(max_x*0.9, max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 7
        position: self.generate_coordinates(max_x, max_y*0.9,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 8
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h*1.3), max_y*(1.0-text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 9
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h), max_y*0.85,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 10
        position: self.generate_coordinates(max_x, max_y*(1.0-text.font_wide_v*2.0),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 11
        position: self.generate_coordinates(max_x * 0.82, max_y * 0.65,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 12
        position: self.generate_coordinates(max_x, max_y * 0.6,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 13
        position: self.generate_coordinates(max_x*0.74, max_y * 0.54,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 14
        position: self.generate_coordinates(max_x*0.9, max_y * 0.5,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 15
        position: self.generate_coordinates(max_x*0.9, max_y * 0.45,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 16
        position: self.generate_coordinates(max_x*0.74, max_y * (0.54-text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 17
        position: self.generate_coordinates(max_x, max_y * 0.35,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 18
        position: self.generate_coordinates(max_x*0.82, max_y * 0.3,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      }, 
      Vertex { // 19
        position: self.generate_coordinates(max_x*0.26, max_y * 0.54,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },  
      Vertex { // 20
        position: self.generate_coordinates(max_x*0.26, max_y * (0.54-text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 21
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h),  max_y*0.17,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 22
        position: self.generate_coordinates(max_x, max_y*0.08,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 23
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h*1.5), max_y*text.font_wide_v,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 24
        position: self.generate_coordinates(max_x*0.9, 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 25
        position: self.generate_coordinates(max_x*0.1, 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 26
        position: self.generate_coordinates(max_x*text.font_wide_h*1.2, max_y*text.font_wide_v,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 27
        position: self.generate_coordinates(0.0, max_y*0.05,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 28
        position: self.generate_coordinates(max_x*text.font_wide_h, max_y*text.font_wide_v*1.15,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 29
        position: self.generate_coordinates(max_x*text.font_wide_h, max_y * 0.30,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 30
        position: self.generate_coordinates(0.0,  max_y * 0.35,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      }, 
      Vertex { // 31
        position: self.generate_coordinates(max_x*0.1, max_y * 0.5,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 32
        position: self.generate_coordinates(max_x*0.1, max_y * 0.45,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
    ];

    let indices = vec![
      self.generate_indices(1, 2, 4),
      self.generate_indices(1, 4, 0),
      self.generate_indices(0, 4, 3),
      self.generate_indices(4, 5, 3),
      self.generate_indices(3, 5, 6),
      self.generate_indices(5, 8, 6),
      self.generate_indices(8, 7, 6),
      self.generate_indices(8, 9, 7),
      self.generate_indices(9, 10, 7),
      self.generate_indices(11, 12, 9),
      self.generate_indices(9, 12, 10),
      self.generate_indices(13, 14, 11),
      self.generate_indices(14, 12, 11),
      self.generate_indices(16, 14, 13),
      self.generate_indices(16, 15, 14),
      self.generate_indices(16, 18, 17),
      self.generate_indices(16, 17, 15),
      self.generate_indices(20, 16, 13),
      self.generate_indices(19, 20, 13),
      self.generate_indices(18, 21, 17),
      self.generate_indices(21, 22, 17),
      self.generate_indices(21, 22, 24),
      self.generate_indices(21, 24, 22),

      self.generate_indices(23, 24, 21),
      self.generate_indices(25, 24, 23),
      self.generate_indices(26, 25, 23),
      self.generate_indices(27, 25, 26),

      self.generate_indices(27, 26, 28),
      self.generate_indices(27, 28, 29),
      self.generate_indices(27, 29, 30),

      self.generate_indices(30, 29, 20),
      self.generate_indices(32, 30, 20),

      self.generate_indices(2, 1, 19),
      self.generate_indices(1, 31, 19),

      self.generate_indices(31, 32, 20),
      self.generate_indices(31, 20, 19),
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
  
  fn generate_coordinates(self, x: f32, y: f32, z: f32) -> [f32; 3] {
    [ 
      self.left_bottom_x + x, 
      self.left_bottom_y + y, 
      z
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

