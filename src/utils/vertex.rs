#![allow(dead_code)]
use bytemuck:: {Pod, Zeroable};
use std:: { mem };

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
  pub position: [f32; 3],
  pub tex_coords: [f32; 2],
	pub color: [f32; 3],
	// mode: 0 for color, 1 for texture
	// this is used to determine how to draw the vertex
	// in the shader, we can use this to set the mode of the vertex
	pub mode: u32,
}

impl Vertex {

  const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
		0=>Float32x3,
		1=>Float32x2,
		2=>Float32x3,
		3=>Uint32,
	];
        
	pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		wgpu::VertexBufferLayout {
			array_stride  : mem::size_of::<Vertex>() as wgpu::BufferAddress,
			step_mode     : wgpu::VertexStepMode::Vertex,
			attributes    : &Self::ATTRIBUTES,
		}
	}
}
