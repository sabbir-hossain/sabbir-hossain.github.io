use wgpu::util::DeviceExt;
use bytemuck::{cast_slice};
use std::rc::Rc;

use crate::option::ViewObject;
use super::vertex::Vertex;
use super::texture::{load_texture};

#[derive(Debug)]
pub struct Coordinate {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u16>,
	pub indices_len: u32,
}

pub struct DrawObject {
	pub uniform_buffer: wgpu::Buffer,
	pub uniform_bind_group: wgpu::BindGroup,
	pub texture_bind_group: wgpu::BindGroup,
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
	// pub model_matrix: cgmath::Matrix4<f32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TxUniform {
    pub offset: [f32; 2], // vec2<f32>
    // Uniforms are aligned to 16 bytes (even for vec2), so pad to 16 bytes
		pub angle: f32,
		pub _padding: f32
}


pub trait Draw {
	fn new(self, device: Rc<wgpu::Device>) -> Vec<ViewObject>;
	fn update(self) -> Vec<ViewObject>;
	fn generate_draw_vector() -> Vec<ViewObject>;
}

pub fn generate_draw_object(
	device: Rc<wgpu::Device>, 
	queue: Rc<wgpu::Queue>, 
	uniform_bind_group_layout: &wgpu::BindGroupLayout,
	texture_bind_group_layout: &wgpu::BindGroupLayout,
	view_object: ViewObject,
) -> Result<DrawObject, Box<dyn std::error::Error>>  {

	let mvp_ref:&[f32; 16] = view_object.model_matrix.as_ref();

	let uniform_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
					label: Some("Uniform Buffer"),
					contents: cast_slice(mvp_ref),
					usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			}
	);

	let tx_transform = TxUniform {
    offset: view_object.texture_offset.into(), 
		angle: view_object.tx_rotation,
		_padding: 0.0,
	};

	let texture_uniform_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
					label: Some("Texture Uniform Buffer"),
					contents: bytemuck::bytes_of(&tx_transform),
					usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			}
	);
	
	let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &uniform_bind_group_layout,
			entries: &[
					wgpu::BindGroupEntry {
							binding: 0,
							resource: uniform_buffer.as_entire_binding(),
					},
					wgpu::BindGroupEntry {
							binding: 1,
							resource: texture_uniform_buffer.as_entire_binding(),
					},
			],
			label: Some("uniform_bind_group"),
	});

	let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Vertex Buffer"),
			contents: cast_slice(&view_object.vertices),
			usage: wgpu::BufferUsages::VERTEX,
	});

	let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Index Buffer"),
			contents: cast_slice(&view_object.indices),
			usage: wgpu::BufferUsages::INDEX,
	});

	let texture_bind_group = load_texture(
		device.clone(),
		queue.clone(),
		texture_bind_group_layout,
		view_object.image_data,
	).expect("Failed to create texture bind group");

	Ok(DrawObject { 
		uniform_buffer, 
		uniform_bind_group, 
		texture_bind_group,
		vertex_buffer,
		index_buffer, 
	})
}


pub fn transform_draw_object(
	device: Rc<wgpu::Device>, 
	view_object: ViewObject,
) ->  Result<(wgpu::Buffer, wgpu::Buffer), Box<dyn std::error::Error>>  {

	let mvp_ref:&[f32; 16] = view_object.model_matrix.as_ref();

	let uniform_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
					label: Some("Uniform Buffer for Transform"),
					contents: cast_slice(mvp_ref),
					usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			}
	);

	let new_tx_transform = TxUniform {
			offset: view_object.texture_offset.into(),
			angle: view_object.tx_rotation,
			_padding: 0.0,
			// _padding: [0.0; 2],
	};
	let texture_uniform_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
					label: Some("Texture Uniform Buffer for Transform"),
					contents: bytemuck::bytes_of(&new_tx_transform),
					usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			}
	);

	Ok((uniform_buffer, texture_uniform_buffer))
}
