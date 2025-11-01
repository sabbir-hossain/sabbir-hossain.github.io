use std::collections::HashMap;
use std::{rc::Rc};
use winit::keyboard::KeyCode;

use crate::utils::texture::ImageData;
use crate::utils::vertex::Vertex;
use crate::scene::scene0::Scene0;


#[derive(Debug)]
pub struct Point {
	pub x: f32,
	pub y: f32
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct ViewObject {
	pub model_matrix: cgmath::Matrix4<f32>,
	pub texture_offset: cgmath::Vector2<f32>,
	pub tx_rotation: f32,
	pub vertices: Rc<Vec<Vertex>>, 
	pub indices: Rc<Vec<u16>>,
  pub indices_len: u32,
	pub label: String,
	pub image_data: Option<ImageData>,
}

#[derive(Debug)]
pub struct ViewConfig {
	pub unit_x: f32,
	pub unit_y: f32,
	pub total_width: u32,
	pub total_height: u32,
}

impl ViewConfig {
	pub fn new(width: u32, height: u32) -> Self {
		if width < height {
			let unit_y =  width as f32 / height as f32;
			let unit_x = 1.0;
			ViewConfig {
				unit_x,
				unit_y,
				total_width: width,
				total_height: height,
			}
		} else {
			let unit_y = 1.0;
			let unit_x = height as f32 / width as f32; 
			ViewConfig {
				unit_x,
				unit_y,
				total_width: width,
				total_height: height,
			}
		}
	}
}

pub enum Scene {
	Scene0(Scene0),
}

impl Scene {
	pub fn new(scene: Scene) -> Self {
			match scene {
				Scene::Scene0(scene0_obj) => Scene::Scene0(scene0_obj),
			}
	}

	pub fn draw(&mut self, data: Option<&HashMap<String, String>>) -> Vec<ViewObject> {
    match self {
        Scene::Scene0(scene0_obj) => scene0_obj.draw(data),
    }
	}

	pub fn update_scene(&mut self, data: Option<&HashMap<String, String>>) -> Vec<ViewObject> {
		match self {
				Scene::Scene0(scene0_obj) => scene0_obj.update(data),
		}
	}

	pub fn handle_keyboard_input(
		&mut self, 
		key: KeyCode,
		data: Option<&HashMap<String, String>>
	) -> char {
		match self {
			Scene::Scene0(scene0_obj) => scene0_obj.handle_keyboard_input(key, data),
		}
	}

}
