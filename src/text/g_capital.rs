
use std::rc::Rc;

use crate::option::ViewConfig;
use crate::utils::vertex::Vertex;
use super::index::{Text, TextObject}; 

#[derive(Debug, Clone, Copy)]
pub struct G {
  unit_x: f32,
  unit_y: f32,
  
  left_bottom_x: f32,
  left_bottom_y: f32,

  previous_length: u16,
}

impl G {

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
        position: self.generate_coordinates(0.0, max_y * text.font_wide_v,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 1
        position: self.generate_coordinates(0.0, max_y * (1.0-text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 2
        position: self.generate_coordinates(max_x * text.font_wide_h, max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 3
        position: self.generate_coordinates(max_x * text.font_wide_h, max_y * 0.85,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 4
        position: self.generate_coordinates(max_x * 0.23, max_y * (1.0 - text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 5
        position: self.generate_coordinates(max_x * text.font_wide_h, 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 6
        position: self.generate_coordinates(max_x * text.font_wide_h, max_y * 0.15,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 7
        position: self.generate_coordinates(max_x * 0.23, max_y * text.font_wide_v,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 8
        position: self.generate_coordinates(max_x * (1.0-text.font_wide_v), max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 9
        position: self.generate_coordinates(max_x, max_y * (1.0 - text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 10
        position: self.generate_coordinates(max_x * 0.78, max_y * (1.0 - text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 11
        position: self.generate_coordinates(max_x * 0.82, max_y * 0.80,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 12
        position: self.generate_coordinates(max_x * 0.82, max_y * 0.70,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 13
        position: self.generate_coordinates(max_x, max_y * 0.70,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 14
        position: self.generate_coordinates(max_x * 0.82, 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 15
        position: self.generate_coordinates(max_x, max_y * text.font_wide_v,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 16
        position: self.generate_coordinates(max_x * 0.78, max_y * text.font_wide_v,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 17
        position: self.generate_coordinates(max_x * 0.82, max_y * 0.20,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 18
        position: self.generate_coordinates(max_x * 0.82, max_y * 0.40,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 19
        position: self.generate_coordinates(max_x, max_y * 0.40,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 20
        position: self.generate_coordinates(max_x*0.6, max_y * 0.40,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 21
        position: self.generate_coordinates(max_x*0.6, max_y * 0.30,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 22
        position: self.generate_coordinates(max_x*0.82, max_y * 0.30,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
    ];
    // log::info!("vertices: {:?}", vertices);
    // println!("vertices: {:?}", vertices);
  
    let indices = vec![
      self.generate_indices(0, 6, 3),
      self.generate_indices(1, 0, 3),
      self.generate_indices(1, 3, 2),
      self.generate_indices(3, 4, 2),
      self.generate_indices(2, 4, 8),
      self.generate_indices(4, 10, 8),
      self.generate_indices(10, 9, 8),
      self.generate_indices(10, 11, 9),
      self.generate_indices(11, 12, 9),
      self.generate_indices(12, 13, 9),
      self.generate_indices(0, 5, 7),
      self.generate_indices(0, 7, 6),
      self.generate_indices(7, 5, 16),
      self.generate_indices(5, 14, 16),
      self.generate_indices(16, 14, 15),
      self.generate_indices(16, 15, 17),
      self.generate_indices(17, 15, 19),
      self.generate_indices(18, 17, 19),
      self.generate_indices(20, 21, 22),
      self.generate_indices(20, 22, 18),
    ].concat();
    
    // log::info!("vertices: {:?}",  vertices.clone().into_iter()
    //   .map(|v| format!("({:.2}, {:.2})", v.position[0], v.position[1]))
    //   .collect::<Vec<_>>()
    //   .join(", "));
    
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

