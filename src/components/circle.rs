use std::rc::Rc;

use crate::option::{ViewConfig, ViewObject};
use crate::utils::{camera, constant, draw::Coordinate, vertex::Vertex};


#[derive(Debug)]
pub struct Circle {
  pub center: (f32, f32),
  pub degree: (i32, i32),
  pub radius: f32,
  pub color: [f32; 3],
  pub view_config: Rc<ViewConfig>, 
}

impl Circle {
  pub fn draw(&self) -> ViewObject {
    let coordinate: Coordinate = self.get_coordinates();

    let model_matrix = camera::build_transformations(
      constant::TRANSLATION, 
      constant::ROTATION,
      constant::SCALING
    );

    let texture_offset = cgmath::Vector2::new(0.0, 0.0);

    ViewObject {
      texture_offset, 
      model_matrix,
      vertices: Rc::new(coordinate.vertices), 
      indices: Rc::new(coordinate.indices),
      indices_len: coordinate.indices_len,
      label: "Circle".to_string(),
      image_data: None,
      tx_rotation: 0.0,
    }
  }

  pub fn get_coordinates(&self) -> Coordinate {
    let total_triangle = 36;
    let deg = 360.0 / total_triangle as f32;
 
    let mut vertices = vec![Vertex { 
      position: [ self.center.0, self.center.1, 0.0], 
      color: self.color, tex_coords: [0.0, 0.0], mode: 0,
    }];

    let mut indices = vec![];
    let start = self.degree.0 / (deg as i32);
    let end = self.degree.1 / (deg as i32);
    let mut counter = 0;
    
    let total_triangle = (end - start).abs();
    // Calculate and store vertices
    for i in start..(end+1) {
      let angle_rad =  (i as f32 * deg).to_radians();
      let vertex = Vertex {
        position: [ 
          self.center.0 + (self.view_config.unit_x * self.radius * angle_rad.sin()), 
          self.center.1 + (self.view_config.unit_y * self.radius * angle_rad.cos()),	
          0.0
        ],
        color: self.color, tex_coords: [0.0, 0.0], mode: 0,
      };
      vertices.push(vertex);

      // Add indices for each triangle
      indices.push(0);
      indices.push(((counter + 1) % total_triangle) as u16 + 1);
      indices.push(counter as u16 + 1);
      counter += 1;
    }

    // log::info!("{:?}", vertices);
    // log::info!("{:?}", indices);

    let indices_len = indices.len() as u32;
    Coordinate { vertices, indices, indices_len }
  }
}