use std::rc::Rc;

use crate::option::{ViewConfig, ViewObject};
use crate::utils::{camera, constant, draw::Coordinate, vertex::Vertex};


#[derive(Debug)]
pub struct SemiCircle {
  pub center: (f32, f32),
  pub degree: (i32, i32),
  pub radius: f32,
  pub width: f32,
  pub color: [f32; 3],
  pub view_config: Rc<ViewConfig>, 
}

impl SemiCircle {
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
    // println!("total_triangle: {} <> radius : {} <> dev : {}", total_triangle, self.radius, deg);

    let start = self.degree.0 / (deg as i32);
    let end = self.degree.1 / (deg as i32);

    let [vertex1, vertex2] = self.generate_vertices(start as f32 * deg);

    let mut vertices = vec![
      vertex1,
      vertex2
    ];
    // log::info!("center: {:?}", vertices);
    let mut indices = vec![];

    let mut current_coordinate = (0, 1);

    // Calculate and store vertices
    for i in start..(end+1) {
      let [vertex1, vertex2] = self.generate_vertices(i as f32 * deg);
      vertices.push(vertex1);
      vertices.push(vertex2);


      indices.extend([current_coordinate.1, current_coordinate.0,   current_coordinate.1 + 1]);
      indices.extend([current_coordinate.1, current_coordinate.0+2, current_coordinate.1 + 2]);
      
      current_coordinate = (current_coordinate.0+2, current_coordinate.1+2);
    }

    // log::info!("{:?}", vertices);
    // log::info!("{:?}", indices);

    let indices_len = indices.len() as u32;
    Coordinate { vertices, indices, indices_len }
  }

  fn generate_vertices(&self, deg: f32) -> [Vertex; 2] {
    let angle_rad = deg.to_radians();
    let vertex1 = Vertex {
      position: [ 
        self.center.0 + (self.view_config.unit_x * (self.radius - self.width) * angle_rad.sin()), 
        self.center.1 + (self.view_config.unit_y * (self.radius - self.width) * angle_rad.cos()),	
        0.0
      ],
      color: self.color, tex_coords: [0.0, 0.0], mode: 0,
    };

    let vertex2 = Vertex {
      position: [ 
        self.center.0 + (self.view_config.unit_x * self.radius * angle_rad.sin()), 
        self.center.1 + (self.view_config.unit_y * self.radius * angle_rad.cos()),	
        0.0
      ],
      color: self.color, tex_coords: [0.0, 0.0], mode: 0,
    };

    [vertex1, vertex2]
  }
}