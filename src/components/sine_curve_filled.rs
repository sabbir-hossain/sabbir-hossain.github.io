use std::rc::Rc;

use crate::components::box_area::BoxArea;
use crate::option::{ViewConfig, ViewObject};

#[derive(Debug, Clone)]
pub struct SineCurveFilled {
  pub start: (f32, f32),
  pub height: f32,
  pub is_up: bool,
  pub color: [f32; 3],
  pub view_config: Rc<ViewConfig>,
}

impl SineCurveFilled {

  pub fn draw(&self) -> Vec<ViewObject> {
    let mut result = vec![];
    let max_height = self.height * self.view_config.unit_y;
    let x_min = self.start.0;
    // trails.0 + (radius * 0.4 * view_config.unit_x)
    // let x_max = self.start.0 + self.height * 2.5 * self.view_config.unit_x;
    let x_max = self.start.0 + self.height * 3.0 * self.view_config.unit_x;
    let inc = 0.001;
    let mut x = x_min;
    while x < x_max {
      let rad = ((x - x_min) * 180.0 / (x_max - x_min)).to_radians();
      let y = max_height * rad.sin();

      let (bottom_left, top_right) = if self.is_up == true {
        ((x, self.start.1), (x+inc, self.start.1 + y))
      } else {
        ((x, self.start.1-y), (x+inc, self.start.1))
      };
      let line: BoxArea = BoxArea { 
        color: self.color, 
        rotation: 0.0, 
        bottom_left, 
        top_right,
        view_config: self.view_config.clone()
      };
      result.push(line.draw());

      x += inc;
    }


    result
  }

}
