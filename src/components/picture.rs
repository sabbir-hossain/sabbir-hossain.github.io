use std::{rc::Rc};

use crate::option::{ViewConfig, ViewObject};
use crate::utils::{camera, constant, draw::Coordinate, vertex::Vertex};
use crate::utils::texture;

#[derive(Debug)]
pub struct Picture {
  pub rotation: f32,
  pub top_left: (f32, f32),
  pub bottom_right: (f32, f32),
  pub bytes: Vec<u8>,
  pub view_config: Rc<ViewConfig>,
}

impl Picture {
  
  pub fn draw(&self) -> ViewObject {
    let image_data = texture::load_image(self.bytes.clone())
      .expect("Failed to load image");
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
      label: "Picture".to_string(),
      image_data: Some(image_data),
      tx_rotation: 0.0,
    }
  }

  
  pub fn get_coordinates(&self) -> Coordinate {
    
    let center_point = (
      (self.top_left.0 + self.bottom_right.0)/2.0,  
      (self.top_left.1 + self.bottom_right.1)/2.0
    );

    let rotation_rad = self.rotation.to_radians();
    // x′= xc​+(x−xc​)⋅cos(θ)−(y−yc​)⋅sin(θ)
    let top_left_x = center_point.0 + 
                          (self.top_left.0 - center_point.0) * rotation_rad.cos() - 
                          (self.top_left.1 - center_point.1) * rotation_rad.sin();
    // y′=yc​+(x−xc​)⋅sin(θ)+(y−yc​)⋅cos(θ)
    let top_left_y = center_point.1 + 
                          (self.top_left.0 - center_point.0) * rotation_rad.sin() + 
                          (self.top_left.1 - center_point.1) * rotation_rad.cos();

    let bottom_left_x = center_point.0 + 
                          (self.top_left.0 - center_point.0) * rotation_rad.cos() - 
                          (self.bottom_right.1 - center_point.1) * rotation_rad.sin();
    // y′=yc​+(x−xc​)⋅sin(θ)+(y−yc​)⋅cos(θ)
    let bottom_left_y = center_point.1 + 
                          (self.top_left.0 - center_point.0) * rotation_rad.sin() + 
                          (self.bottom_right.1 - center_point.1) * rotation_rad.cos();

    let top_right_x = center_point.0 + 
                          (self.bottom_right.0 - center_point.0) * rotation_rad.cos() - 
                          (self.top_left.1 - center_point.1) * rotation_rad.sin();

    let top_right_y = center_point.1 + 
                          (self.bottom_right.0 - center_point.0) * rotation_rad.sin() + 
                          (self.top_left.1 - center_point.1) * rotation_rad.cos();
  
    let bottom_right_x = center_point.0 + 
                          (self.bottom_right.0 - center_point.0) * rotation_rad.cos() - 
                          (self.bottom_right.1 - center_point.1) * rotation_rad.sin();
    // y′=yc​+(x−xc​)⋅sin(θ)+(y−yc​)⋅cos(θ)
    let bottom_right_y = center_point.1 + 
                          (self.bottom_right.0 - center_point.0) * rotation_rad.sin() + 
                          (self.bottom_right.1 - center_point.1) * rotation_rad.cos();

    let vertices = vec![
      Vertex {   // Bottom-left
        position: [bottom_left_x, bottom_left_y,    0.0], color: [0.0, 0.0, 0.0], 
        tex_coords: [0.0, 0.0], mode: 1 
      },
      Vertex {  // Top-left
        position: [top_left_x,    top_left_y, 0.0], color: [0.0, 0.0, 0.0], 
        tex_coords: [0.0, 1.0], mode: 1 
      },
      Vertex { // Bottom-right
        position: [bottom_right_x, bottom_right_y,   0.0], color: [0.0, 0.0, 0.0], 
        tex_coords: [1.0, 0.0], mode: 1 
      },
      Vertex {  // Top-right
        position: [top_right_x,   top_right_y,0.0], color: [0.0, 0.0, 0.0], 
        tex_coords: [1.0, 1.0], mode: 1 
      },
    ];
    // log::info!("vertices: {:?}", vertices);
    // log::info!("vertices: {:?}",  vertices.clone().into_iter()
    //   .map(|v| format!("({:.3}, {:.3})", v.tex_coords[0], v.tex_coords[1]))
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