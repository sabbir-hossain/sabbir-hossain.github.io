
use std::rc::Rc;

use crate::option::ViewConfig;
use crate::utils::vertex::Vertex;
use super::index::{Text, TextObject}; 

#[derive(Debug, Clone, Copy)]
pub struct S {
  unit_x: f32,
  unit_y: f32,
  
  left_bottom_x: f32,
  left_bottom_y: f32,

  previous_length: u16,
}

impl S {

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
        position: self.generate_coordinates(0.0, max_y * text.font_wide_v*0.8,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 1
        position: self.generate_coordinates(max_x * text.font_wide_h*0.8, 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 2
        position: self.generate_coordinates(max_x * text.font_wide_h*1.2, max_y * text.font_wide_v,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 3
        position: self.generate_coordinates(max_x * (1.0-text.font_wide_h), 0.0,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 4
        position: self.generate_coordinates(max_x * (1.0-text.font_wide_h*1.2), max_y * text.font_wide_v,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 5
        position: self.generate_coordinates(max_x, max_y * text.font_wide_v,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 6
        position: self.generate_coordinates(max_x * text.font_wide_h, max_y * text.font_wide_v*1.2,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 7
        position: self.generate_coordinates(max_x * (1.0-text.font_wide_h), max_y * text.font_wide_v*1.2,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 8
        position: self.generate_coordinates(0.0, max_y * text.font_wide_v*2.2,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 9
        position: self.generate_coordinates(max_x*text.font_wide_h, max_y * text.font_wide_v*2.2,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 10
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h), max_y * 0.4,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 11
        position: self.generate_coordinates(max_x, max_y * 0.47,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 12
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h*1.2), max_y * 0.43,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 13
        position: self.generate_coordinates(max_x * text.font_wide_h*0.8, max_y * 0.43,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 14
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h), max_y * 0.54,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 15
        position: self.generate_coordinates(max_x * text.font_wide_h*1.2, max_y * 0.54,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 16
        position: self.generate_coordinates(max_x * text.font_wide_h, max_y * 0.57,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 17
        position: self.generate_coordinates(0.0, max_y * 0.5,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 18
        position: self.generate_coordinates(max_x*text.font_wide_h, max_y * (1.0-text.font_wide_v*1.2),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 19
        position: self.generate_coordinates(0.0, max_y * (1.0-text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 20
        position: self.generate_coordinates(max_x*text.font_wide_h*0.8, max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { //21
        position: self.generate_coordinates(max_x*text.font_wide_h*1.2, max_y * (1.0-text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { //22
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h*1.2), max_y * (1.0-text.font_wide_v),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 23
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_v), max_y,  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 24
        position: self.generate_coordinates(max_x, max_y * (1.0-text.font_wide_v*0.8),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 25
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h), max_y * (1.0-text.font_wide_v*1.2),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 26
        position: self.generate_coordinates(max_x*(1.0-text.font_wide_h), max_y * (1.0-text.font_wide_v*2.2),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 27
        position: self.generate_coordinates(max_x, max_y * (1.0-text.font_wide_v*2.2),  0.0), 
        color: text.color, tex_coords: [0.0, 0.0], mode: 0,
      },
    ];
    // log::info!("vertices: {:?}", vertices);
    // println!("vertices: {:?}", vertices);
  
    let indices = vec![
      self.generate_indices(0, 1, 2), // 0
      self.generate_indices(0, 2, 6), // 1
      self.generate_indices(0, 6, 8), // 2
      self.generate_indices(8, 6, 9), // 3
      self.generate_indices(1, 3, 4), // 4
      self.generate_indices(1, 4, 2), // 5
      self.generate_indices(4, 3, 7), // 6
      self.generate_indices(7, 3, 5), // 7
      self.generate_indices(7, 5, 10), // 8
      self.generate_indices(10, 5, 11), // 9
      self.generate_indices(12, 10, 11), // 10
      self.generate_indices(14, 12, 11), // 11
      self.generate_indices(13, 12, 14), // 12
      self.generate_indices(15, 13, 14), // 13
      self.generate_indices(16, 13, 15), // 14
      self.generate_indices(17, 13, 16), // 15
      self.generate_indices(17, 16, 18), // 16
      self.generate_indices(19, 17, 18), // 17
      self.generate_indices(19, 18, 21), // 18
      self.generate_indices(19, 21, 20), // 19
      self.generate_indices(20, 21, 22), // 20
      self.generate_indices(20, 22, 23), // 21
      self.generate_indices(22, 24, 23), // 22
      self.generate_indices(22, 25, 24), // 23
      self.generate_indices(25, 26, 24), // 24
      self.generate_indices(26, 27, 24), // 25
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

