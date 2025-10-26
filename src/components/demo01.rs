use std::rc::Rc;

use crate::option::{ViewConfig, ViewObject};
use crate::utils::{camera, constant, draw::Coordinate, vertex::Vertex};


#[derive(Debug, Clone)]
pub struct Demo01 {
  pub color: [f32; 3],
  pub bottom_left: (f32, f32),
  pub top_right: (f32, f32),
  pub rotation: f32,
  pub view_config: Rc<ViewConfig>,
}

impl Demo01 {

  pub fn draw(&self) -> ViewObject {
    let coordinate: Coordinate = self.get_coordinates();
    let pivot = [
        (coordinate.vertices[0].position[0] + coordinate.vertices[3].position[0]) / 2.0,
        (coordinate.vertices[0].position[1] + coordinate.vertices[3].position[1]) / 2.0,
        0.0
    ];

    let aspect_ratio = self.view_config.unit_y / self.view_config.unit_x;
  
    let model_matrix = camera::build_transformations_with_pivot(
        constant::TRANSLATION,
        [0.0, 0.0, self.rotation.to_radians()],
        constant::SCALING,
        pivot,
        aspect_ratio
    );

    let texture_offset = cgmath::Vector2::new(0.0, 0.0);

    ViewObject {
      texture_offset,
      model_matrix,
      vertices: Rc::new(coordinate.vertices), 
      indices: Rc::new(coordinate.indices),
      indices_len: coordinate.indices_len,
      label: "Box".to_string(),
      image_data: None,
      tx_rotation: 0.0,
    }
  }

  pub fn get_coordinates(&self) -> Coordinate {
    let vertices = vec![
      Vertex { position: [self.bottom_left.0, self.bottom_left.1, 0.0], color: self.color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [self.bottom_left.0,  self.top_right.1,    0.0], color: self.color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [self.top_right.0, self.bottom_left.1, 0.0], color: self.color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [self.top_right.0,   self.top_right.1,   0.0], color: self.color, tex_coords: [0.0, 0.0], mode: 0 }
    ];
    // log::info!("vertices: {:?}", vertices);
    // log::info!("vertices: {:?}",  vertices.clone().into_iter()
    //   .map(|v| format!("({:.3}, {:.3})", v.position[0], v.position[1]))
    //   .collect::<Vec<_>>()
    //   .join(", "));
    // println!("vertices: {:?}", vertices);

    let indices = vec![
      1, 0, 2,
      1, 2, 3,
    ];
    let indices_len = indices.len() as u32;
    Coordinate { vertices, indices, indices_len }
  }
}
