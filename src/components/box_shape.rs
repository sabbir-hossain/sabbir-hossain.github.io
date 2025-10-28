use std::rc::Rc;

use crate::option::{ViewConfig, ViewObject};
use crate::utils::{camera, constant, draw::Coordinate, vertex::Vertex};
use crate::utils::color::BACKGROUND_COLOR;

/*
        1
     _________________
    |                 |
   3|                 |4
    |_________________|
            2
  only one part hide: 1/2/3/4, for no hide part: 0
  two part hide: 12/13/14/23/24/34
  three part hide: 123/234/134
*/

#[derive(Debug)]
pub struct BoxShape {
  pub width: f32,
  pub color: [f32; 3],

  pub hide_part: Option<String>,
  pub rotation: f32,

  pub top_left: (f32, f32),
  pub bottom_right: (f32, f32),
  pub view_config: Rc<ViewConfig>
}

impl BoxShape {
  pub fn draw(&self) -> ViewObject {
    let coordinate: Coordinate = self.get_coordinates();

    let model_matrix = camera::build_transformations(
      constant::TRANSLATION, 
      // [self.rotation.to_radians(), 0.0, 0.0],
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
      label: "BoxShape".to_string(),
      image_data: None,
      tx_rotation: 0.0,
    }
  }

  pub fn get_coordinates(&self) -> Coordinate {
    let hide_part = self.hide_part.clone().unwrap_or("0".to_string());
    let width_x = self.width * self.view_config.unit_x;
    let width_y = self.width * self.view_config.unit_y;
    let first_part_color = if hide_part.contains("1") {
      BACKGROUND_COLOR
    } else {
      self.color
    };

    let second_part_color = if hide_part.contains("2") {
      BACKGROUND_COLOR
    } else {
      self.color
    };

    let third_part_color = if hide_part.contains("3") {
      BACKGROUND_COLOR
    } else {
      self.color
    };

    let forth_part_color = if hide_part.contains("4") {
      BACKGROUND_COLOR
    } else {
      self.color
    };

    let center_point = (
      (self.bottom_right.0 + self.top_left.0)/2.0,  
      (self.bottom_right.1 + self.top_left.1)/2.0
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

    let top_right_x = center_point.0 + 
                          (self.bottom_right.0 - center_point.0) * rotation_rad.cos() - 
                          (self.top_left.1 - center_point.1) * rotation_rad.sin();

    let top_right_y = center_point.1 + 
                          (self.bottom_right.0 - center_point.0) * rotation_rad.sin() + 
                          (self.top_left.1 - center_point.1) * rotation_rad.cos();

    // x′= xc​+(x−xc​)⋅cos(θ)−(y−yc​)⋅sin(θ)
    let bottom_left_x = center_point.0 + 
                          (self.top_left.0 - center_point.0) * rotation_rad.cos() - 
                          (self.bottom_right.1 - center_point.1) * rotation_rad.sin();
    // y′=yc​+(x−xc​)⋅sin(θ)+(y−yc​)⋅cos(θ)
    let bottom_left_y = center_point.1 + 
                          (self.top_left.0 - center_point.0) * rotation_rad.sin() + 
                          (self.bottom_right.1 - center_point.1) * rotation_rad.cos();

    // x′= xc​+(x−xc​)⋅cos(θ)−(y−yc​)⋅sin(θ)
    let bottom_right_x = center_point.0 + 
                          (self.bottom_right.0 - center_point.0) * rotation_rad.cos() - 
                          (self.bottom_right.1 - center_point.1) * rotation_rad.sin();
    // y′=yc​+(x−xc​)⋅sin(θ)+(y−yc​)⋅cos(θ)
    let bottom_right_y = center_point.1 + 
                          (self.bottom_right.0 - center_point.0) * rotation_rad.sin() + 
                          (self.bottom_right.1 - center_point.1) * rotation_rad.cos();

                          
    let vertices = vec![
      //1
      Vertex { position: [top_left_x,  top_left_y,             0.0], color: first_part_color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [top_left_x,  top_left_y - width_y,   0.0], color: first_part_color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [top_right_x, top_right_y,            0.0], color: first_part_color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [top_right_x, top_right_y - width_y,  0.0], color: first_part_color, tex_coords: [0.0, 0.0], mode: 0 },

      //2
      Vertex { position: [bottom_left_x,  bottom_left_y,           0.0], color: second_part_color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [bottom_left_x,  bottom_left_y- width_y,  0.0], color: second_part_color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [bottom_right_x, bottom_right_y,          0.0], color: second_part_color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [bottom_right_x, bottom_right_y- width_y, 0.0], color: second_part_color, tex_coords: [0.0, 0.0], mode: 0 },

      //3
      Vertex { position: [top_left_x,         top_left_y-width_y, 0.0], color: third_part_color, tex_coords: [0.0, 0.0], mode: 0 },  
      Vertex { position: [top_left_x+width_x, top_left_y-width_y, 0.0], color: third_part_color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [bottom_left_x,         bottom_left_y-width_y, 0.0], color: third_part_color, tex_coords: [0.0, 0.0], mode: 0},  
      Vertex { position: [bottom_left_x+width_x, bottom_left_y-width_y, 0.0], color: third_part_color, tex_coords: [0.0, 0.0], mode: 0}, 

      //4
      Vertex { position: [top_right_x,         top_right_y-width_y,     0.0], color: forth_part_color, tex_coords: [0.0, 0.0], mode: 0 },  
      Vertex { position: [top_right_x-width_x, top_right_y-width_y, 0.0], color: forth_part_color, tex_coords: [0.0, 0.0], mode: 0 },
      Vertex { position: [bottom_right_x,         bottom_right_y-width_y, 0.0], color: forth_part_color, tex_coords: [0.0, 0.0], mode: 0},  
      Vertex { position: [bottom_right_x-width_x, bottom_right_y-width_y, 0.0], color: forth_part_color, tex_coords: [0.0, 0.0], mode: 0},
    ];
    // log::info!("vertices: {:?}", vertices);
    // log::info!("vertices: {:?}",  vertices.clone().into_iter()
    //   .map(|v| format!("({:.3}, {:.3})", v.position[0], v.position[1]))
    //   .collect::<Vec<_>>()
    //   .join(", "));
    // println!("vertices: {:?}", vertices);

    let indices = vec![
      // 1
      0, 1, 3,
      0, 3, 2,

      //2
      4, 5, 7,
      4, 7, 6,

      //3
      8, 10, 11,
      8, 11, 9,

      //4
      13, 15, 14,
      13, 14, 12
    ];
    let indices_len = indices.len() as u32;
    Coordinate { vertices, indices, indices_len }
  }
}
