use std::rc::Rc;
use std::vec;
use winit::keyboard::{KeyCode};

// use crate::components::{
//   box_area::BoxArea,
// };
use crate::option::{Scene, ViewConfig, ViewObject};
use crate::utils::color::Color;
use crate::text;
// use crate::utils::sound;
// use crate::{scene, text};
// use crate::utils::color::{get_random_color, Color};

#[derive(Debug, Clone)]
pub struct Scene0 {
	pub options: Vec<ViewObject>,
  pub view_config: Rc<ViewConfig>,

  scene_x: f32,
  scene_y: f32,

  unit_x: f32,
  unit_y: f32,
}
impl Scene0 {
  pub fn new(view_config: Rc<ViewConfig>) -> Self {
    Scene0 {
      options: vec![],
      view_config,

      scene_x: 0.0,
      scene_y: 0.0,

      unit_x: 0.0,
      unit_y: 0.0,
    }
  }

  pub fn draw(&mut self) -> Vec<ViewObject> {
    self.options = vec![];

    self.scene_y = 0.5;
    self.scene_x = -0.9;
    self.unit_x = 0.52 * self.view_config.unit_x;
    self.unit_y = 0.28 * self.view_config.unit_y;

    let text: &str = "
One day, Lord Krishna was returning to his kingdom. All the villagers were meticulously decorating their homes and roads, so that their God's Ratha Yatra (chariot procession) would look more resplendent.\n
But some villagers intentionally kept their homes and roads completely in darkness.\n
Naturally, Krishna asked them why they had chosen to do that.\n
They replied, \"Dear God, your Ratha Yatra is already so bright. We realized that if we remained in darkness, your glorious procession would shine much more brilliantly.\"\n
I wish I could be like those villagers.";


    let split_vec: Vec<&str> = text.split('\n').collect();

    let position_x = -0.9;
    let mut position_y = 0.75;
    let font_size = 18;
    let font_color = Color::get(&Color::Black);
    // log::info!("--- Using .lines() ---");
    for line in split_vec {
      // log::info!("[{}] --> len: {}", line, line.len()); // Brackets to show start/end of the line

      if line.trim().is_empty() {
        continue;
      }

      let mut text_obj = text::Text::new(
        line.to_string(), 
        self.view_config.clone(), 
        (position_x, position_y),
        Some(font_size),
        Some(font_color),
        false
      );  

      self.options.push(text_obj.draw()[0].clone());
      position_y = text_obj.line_position.1 - 0.2;
    }

    self.options.clone()
  }

  pub fn update(&mut self) -> Vec<ViewObject> {
    // log::info!("update : {} ", now_str());

    self.options.clone()
  }

  pub fn handle_keyboard_input(&mut self, physical_key: KeyCode) -> Option<Scene> {
  //  let scene_list = ['f', 'j', 'y', 'z'];
    match physical_key {
      KeyCode::KeyF => {
        // sound::select_scene();
        // let scene = scene::scene3::Scene3::new(self.view_config.clone());
        // Some(Scene::Scene3(scene))
        None
      }
      KeyCode::KeyJ => {
        // sound::select_scene();
        // let scene = scene::scene4::Scene4::new(self.view_config.clone());
        // Some(Scene::Scene4(scene))
        None
      }
      KeyCode::KeyY => {
        // sound::select_scene();
        // let scene = scene::scene1::Scene1::new(self.view_config.clone());
        // Some(Scene::Scene1(scene))
        None
      }
      KeyCode::KeyZ => {
        // sound::select_scene();
        // let scene = scene::scene2::Scene2::new(self.view_config.clone(), SelectedOption::Radius);
        // Some(Scene::Scene2(scene))
        None
      }
      KeyCode::KeyA => {
        None
      }
      _ => {
          log::info!("Invalid key pressed: {:?}", physical_key);
          None
      }
    }
  }

}
