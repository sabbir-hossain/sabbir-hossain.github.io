use std::rc::Rc;

use crate::components::box_area::BoxArea;
use crate::components::circle::Circle;
use crate::components::semi_circle::SemiCircle;
use crate::option::{Point, ViewConfig, ViewObject};
use crate::utils::color::{get_random_color, Color};
use crate::utils::{camera, constant, draw::Coordinate, vertex::Vertex};

#[derive(Debug, Clone)]
pub struct Wheel {
  pub center: (f32, f32),
  pub radius: f32,
  pub view_config: Rc<ViewConfig>,

  // pub spokes_color_list: Vec<[f32; 3]>,
  pub spoke_obj_list: Vec<(BoxArea, ViewObject)>,
  pub barrel: Vec<ViewObject>, 
  pub center_cap: Vec<ViewObject>,
}

impl Wheel {
  pub fn new(
    view_config: Rc<ViewConfig>, 
    center: (f32, f32), 
    radius: f32, 
    num_of_spokes: u8, 
    _w_color: Option<Color>
  ) -> Self {
    // let color = Color::get(&w_color.unwrap_or(Color::ForestGreen));
    let color = get_random_color();

    let mut spoke_obj_list: Vec<(BoxArea, ViewObject)> = vec![];
    let mut barrel: Vec<ViewObject> = vec![];
    let mut center_cap: Vec<ViewObject> = vec![];

     barrel.push(
      SemiCircle {
        center: center.clone(),
        width: radius  * 0.17, 
        radius,
        degree:(0, 360),
        color,
        view_config: view_config.clone()
      }.draw()
    );

    let inc = 180 / num_of_spokes;

    for i in (0..=180).step_by(inc as usize) {
      let spoke_obj = BoxArea {
        // color: get_random_color(),
        color,
        top_right: (
          center.0+(radius*view_config.unit_x), 
          center.1+(radius*view_config.unit_y*0.05)
        ),
        bottom_left: (
          center.0-(radius*view_config.unit_x), 
          center.1-(radius*view_config.unit_y*0.05)
        ),
        rotation: i as f32,
        view_config: view_config.clone()
      };

      spoke_obj_list.push((spoke_obj.clone(), spoke_obj.draw()));
    }

    center_cap.push(
      Circle {
        center: center.clone(),
        radius: radius * 0.2,
        degree:(0, 360),
        color,
        view_config: view_config.clone()
      }.draw()
    );

    Self {
      center,
      radius,
      spoke_obj_list,
      barrel,
      center_cap,
      view_config: view_config.clone(),
    }
  }

  pub fn draw(&mut self, _center: (f32, f32)) -> Vec<ViewObject> {
    let mut result = self.barrel.clone();

    // for spk in self.spoke_obj_list.clone() {
    //   result.push(spk.1);
    // }

    // result.extend(self.spokes.clone());
    result.extend(self.center_cap.clone());

    result
  }

  pub fn animate(&mut self, center: (f32, f32), rotation: f32) -> Vec<ViewObject> {
    let mut updated_spokes: Vec<(BoxArea, ViewObject)> = vec![];
    
    // let mut counter = 0;
    for (obj, data) in self.spoke_obj_list.clone() {

      let pivot = [
        (data.vertices[0].position[0] + data.vertices[3].position[0]) / 2.0,
        (data.vertices[0].position[1] + data.vertices[3].position[1]) / 2.0,
        0.0
      ];

      // log::info!(" obj.rotation {} : {} + {} = {}", counter,  obj.rotation, rotation,  obj.rotation + rotation);
      // counter+=1;

      let mut obj_rotation = obj.rotation + rotation;
      obj_rotation = obj_rotation % 360.0;
      
      let aspect_ratio = self.view_config.unit_y / self.view_config.unit_x;
      // let model_matrix = camera::build_transformations_with_pivot(
          // [center.0, center.1, 0.0],
          // [0.0, 0.0, obj_rotation.to_radians()],
          // constant::SCALING,
      //     pivot,
      //     aspect_ratio
      // );
      let model_matrix = camera::build_transformations(
          [center.0, center.1, 0.0],
          constant::ROTATION,
          // [0.0, 0.0, obj_rotation.to_radians()],
          constant::SCALING,
      );

      updated_spokes.push((BoxArea {
        rotation: obj_rotation,
        ..obj
      }, ViewObject {
        model_matrix,
        ..data
      }));
    }

    self.spoke_obj_list = updated_spokes;

    // pub barrel: Vec<ViewObject>, 
    // pub center_cap: Vec<ViewObject>,
    let mut updated_barrel = vec![];
    for barrel in self.barrel.clone() {
        let model_matrix = camera::build_transformations(
          [center.0, center.1, 0.0],
          constant::ROTATION,
          // [0.0, 0.0, obj_rotation.to_radians()],
          constant::SCALING,
      );

      updated_barrel.push(ViewObject {
        model_matrix,
        ..barrel.clone()
      });
    }

    self.barrel = updated_barrel;

    let mut updated_center_cap = vec![];
    for center_cap in self.center_cap.clone() {
        let model_matrix = camera::build_transformations(
          [center.0, center.1, 0.0],
          // constant::ROTATION,
          [0.0, 0.0, rotation.to_radians()],
          constant::SCALING,
      );
      log::info!("center: {:?} matrix : {:?}", center, model_matrix);
      updated_center_cap.push(ViewObject {
        model_matrix,
        ..center_cap.clone()
      });
    }

    self.center_cap = updated_center_cap;

    self.draw(center)
  }
}
