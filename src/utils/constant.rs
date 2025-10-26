#![allow(dead_code)]


pub const START: f32 =-1.0;
pub  const END: f32   = 1.0;
pub const TOTAL: f32 = 2.0;

pub const IS_PERSPECTIVE: bool = false;
//pub const ANIMATION_SPEED: f32 = 0.0;
pub const ANIMATION_SPEED: f32 = 0.01;

//pub const PRIMITIVE_TOPOLOGY: PrimitiveTopology = wgpu::PrimitiveTopology::LineStrip;
pub const PRIMITIVE_TOPOLOGY: wgpu::PrimitiveTopology = wgpu::PrimitiveTopology::TriangleList;

// pub const TRANSLATION: [f32; 3] = [ 0.0, 0.0, 0.0 ];
pub const RADIUS: f64 = 0.1;
pub const TRANSLATION: [f32; 3] = [ 0.0, 0.0, 0.0 ];
pub const ROTATION: [f32; 3] 		= [ 0.0, 0.0, 0.0 ];
pub const SCALING: [f32; 3] 		= [ 1.0, 1.0, 1.0 ];

pub const TRANSLATION_TX: [f32; 2] = [ 0.0, 0.0 ];
pub const SCALING_TX: [f32; 2] = [ 0.0, 0.0 ];
pub const ROTATION_TX:f32 = 0.0;


pub const GRID_SIZE: f32 = 32.0;
pub const INSTANCE_LIMIT: u32 = 1;
// pub const INSTANCE_LIMIT: u32 = (GRID_SIZE * GRID_SIZE) as u32;

pub const WINDOW_BG_COLOR_R: f64 = 0.2;
pub const WINDOW_BG_COLOR_G: f64 = 0.247;
pub const WINDOW_BG_COLOR_B: f64 = 0.314;
pub const WINDOW_BG_COLOR_A: f64 = 1.0;

