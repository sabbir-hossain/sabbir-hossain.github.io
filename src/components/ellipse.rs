use std::rc::Rc;

use crate::option::{ViewConfig, ViewObject};
use crate::utils::{camera, constant, draw::Coordinate, vertex::Vertex};


#[derive(Debug)]
pub struct Ellipse {
  pub center: (f32, f32),
  pub degree: (f32, f32),
  pub rotation: f32,
  pub major_axis: f32,
  pub minor_axis: f32,
  pub width: f32,
  pub color: [f32; 3],
  pub view_config: Rc<ViewConfig>, 
}

impl Ellipse {
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
    let total_triangle = 48;
    let deg = 360.0 / total_triangle as f32;
    // println!("total_triangle: {} <> radius : {} <> dev : {}", total_triangle, self.radius, deg);

    let start: i32 = (self.degree.0 / deg).round() as i32;
    let end: i32 = (self.degree.1 / deg).round() as i32;

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

      // [2, 1, 3]
      indices.extend([current_coordinate.1 + 1, current_coordinate.1, current_coordinate.1+2]);
      // [2, 0, 1],
      indices.extend([current_coordinate.1+1, current_coordinate.0,   current_coordinate.1]);

      current_coordinate = (current_coordinate.0+2, current_coordinate.1+2);
    }

    let indices_len = indices.len() as u32;
    Coordinate { vertices, indices, indices_len }
  }

  fn generate_vertices(&self, deg: f32) -> [Vertex; 2] {
    let theta = deg.to_radians();           // sweep angle
    let phi = self.rotation.to_radians();   // rotation of ellipse

    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    let cos_phi = phi.cos();
    let sin_phi = phi.sin();

    // Full outer ellipse point
    let x_outer = self.major_axis * cos_theta * cos_phi - self.minor_axis * sin_theta * sin_phi;
    let y_outer = self.major_axis * cos_theta * sin_phi + self.minor_axis * sin_theta * cos_phi;

    // Inner ellipse point (reduced radius)
    let a_inner = self.major_axis - self.width;
    let b_inner = self.minor_axis - self.width;
    let x_inner = a_inner * cos_theta * cos_phi - b_inner * sin_theta * sin_phi;
    let y_inner = a_inner * cos_theta * sin_phi + b_inner * sin_theta * cos_phi;

    let vertex1 = Vertex {
        position: [
            self.center.0 + self.view_config.unit_x * x_inner,
            self.center.1 + self.view_config.unit_y * y_inner,
            0.0
        ],
        color: self.color, tex_coords: [0.0, 0.0], mode: 0,
    };

    let vertex2 = Vertex {
        position: [
            self.center.0 + self.view_config.unit_x * x_outer,
            self.center.1 + self.view_config.unit_y * y_outer,
            0.0
        ],
        color: self.color, tex_coords: [0.0, 0.0], mode: 0,
    };

    [vertex1, vertex2]
  }
}