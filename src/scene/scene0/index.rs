use std::collections::HashMap;
use std::rc::Rc;
use std::vec;
use winit::keyboard::{KeyCode};

use crate::option::{ViewConfig, ViewObject};
use crate::utils::color::Color;
use crate::text;

use super::constant::TEXT_DATA;


#[derive(Debug, Clone)]
pub struct Scene0 {
	pub options: Vec<ViewObject>,
  pub view_config: Rc<ViewConfig>,

  scene_x: f32,
  scene_y: f32,

  unit_x: f32,
  unit_y: f32,

  text: String,
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

      text: String::new(),
    }
  }

  pub fn draw(&mut self, data: Option<&HashMap<String, String>>) -> Vec<ViewObject> {
    self.options = vec![];

    self.scene_y = 0.5;
    self.scene_x = -0.9;
    self.unit_x = 0.52 * self.view_config.unit_x;
    self.unit_y = 0.28 * self.view_config.unit_y;

    // log::info!(" data --> {:?} ", data);

    if let Some(map) = data {
      // This block runs ONLY if a HashMap was provided.
      // log::info!("✅ Data was provided. Processing {} key-value pairs...", map.len());
      
      if let Some(text_position) = map.get("text_position") {
        self.text = format!("{}", TEXT_DATA[text_position.parse::<usize>().unwrap_or(0)]);
      }
    } else {
      self.text = format!("{}", TEXT_DATA[0]);

    }

    let split_vec: Vec<&str> = self.text.split('\n').collect();

    let position_x = -0.9;
    let mut position_y = 0.85;
    let font_size = 16;
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
      position_y = text_obj.line_position.1 - 0.15;
    }

    self.options.clone()
  }

  pub fn update(&mut self, _data: Option<&HashMap<String, String>>) -> Vec<ViewObject> {
    // log::info!("update : {} ", now_str());

    self.options.clone()
  }

  // pub fn handle_keyboard_input(&mut self, physical_key: KeyCode) -> Option<Scene> {
  pub fn handle_keyboard_input(
    &mut self, 
    physical_key: KeyCode, 
    data: Option<&HashMap<String, String>>
  ) -> char {
  //  let scene_list = ['f', 'j', 'y', 'z'];

    let last_option = if let Some(map) = data {
      if let Some(text_position) = map.get("text_position") {
        text_position.parse::<usize>().unwrap_or(0)
      } else {
        0
      }
    } else {
      0
    };
    

    match physical_key {
      KeyCode::ArrowRight => {
        let next_position: u8 = if last_option + 1 < TEXT_DATA.len() {
          last_option as u8 + 1
        } else {
          0
        };
        // format!('{}', next_position)
        (next_position + b'0') as char
      }
      KeyCode::ArrowLeft => {
        let previous_position: u8 = if last_option >= 1 {
          last_option as u8 - 1
        } else {
          TEXT_DATA.len() as u8 - 1
        };
        // format!("{}", previous_position)
        (previous_position + b'0') as char
      }
      KeyCode::KeyA => {
        // "A".to_string()
        'A'
      }
      // KeyCode::KeyY => {
      //   // sound::select_scene();
      //   // let scene = scene::scene1::Scene1::new(self.view_config.clone());
      //   // Some(Scene::Scene1(scene))
      //   None
      // }
      // KeyCode::KeyZ => {
      //   // sound::select_scene();
      //   // let scene = scene::scene2::Scene2::new(self.view_config.clone(), SelectedOption::Radius);
      //   // Some(Scene::Scene2(scene))
      //   None
      // }
      // KeyCode::KeyA => {
      //   None
      // }
      _ => {
          log::info!("Invalid key pressed: {:?}", physical_key);
          // "X".to_string()
          'X'
      }
    }
  }

}
