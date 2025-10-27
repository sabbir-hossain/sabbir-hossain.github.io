use std::rc::Rc;
use std::vec;
use winit::keyboard::{KeyCode};

use crate::components::{
  box_area::BoxArea,
};
use crate::option::{Scene, ViewConfig, ViewObject};
// use crate::utils::sound;
use crate::{scene, text};
use crate::utils::color::{get_random_color, Color};

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

    let text: &str = "One day, Krishna was returning to his kingdom. All the villagers were meticulously decorating their homes and roads so that their God's Ratha Yatra (chariot procession) would look more resplendent. ";

    //  But some villagers intentionally kept their homes and roads completely in darkness. Krishna asked them why they had chosen to do that. They replied [Dear God your Ratha Yatra is already so bright. We realized that if we remained in darkness, your glorious procession would shine much more brilliantly]. I wish I could be like those villagers

    self.options.push(
      text::Text::new(
        text.to_string(), 
        self.view_config.clone(), 
        (-0.9, 0.85),
        Some(18),
        Some(Color::get(&Color::Black)),
        false
      ).draw()[0].clone()
    );


    // self.scene_y = 0.0;
    // self.scene_x = -0.9;
    // self.options.push(
    //   text::Text::new(
    //     "Gravitational force".to_string(), 
    //     self.view_config.clone(), 
    //     (-0.9, 0.35),
    //     Some(18),
    //     Some(Color::get(&Color::Black)),
    //     false
    //   ).draw()[0].clone()
    // );


    // self.scene_y = -0.5;
    // self.scene_x = -0.9;
    // self.options.push(
    //   text::Text::new(
    //     "Electricity".to_string(), 
    //     self.view_config.clone(), 
    //     (-0.9, -0.15),
    //     Some(18),
    //     Some(Color::get(&Color::Black)),
    //     false
    //   ).draw()[0].clone()
    // );

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

  fn generate_input(&mut self, text: String) -> (ViewObject, ViewObject) {
    let color = get_random_color();
    // log::info!("color: {:?} <> txt : {}", color, text);
    let box1: BoxArea = BoxArea { 
      color, 
      rotation: 0.0, 
      bottom_left: (self.scene_x, self.scene_y), 
      top_right: (self.scene_x + self.unit_x, self.scene_y + self.unit_y),
      view_config: self.view_config.clone()
    };
    let box_data = box1.draw();

    let mut text = text::Text::new(
        text, 
        self.view_config.clone(), 
        (box1.bottom_left.0 + self.unit_x * 0.1, box1.bottom_left.1 + self.unit_y * 0.35),
        Some(18),
        Some(Color::get(&Color::Black)),
        false
    );
    let objects_data = text.draw();

    self.scene_x = self.scene_x + (self.unit_x * 1.25);

    (box_data, objects_data[0].clone())
  }

}
