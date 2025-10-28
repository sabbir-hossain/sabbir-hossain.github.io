#![allow(dead_code)]
use cgmath::{perspective, Deg, Matrix3, Matrix4, Point3, Rad, Vector3};

use super::vertex::Vertex;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
	pub eye: Point3<f32>,
	pub target: Point3<f32>,
	pub up: Vector3<f32>,
	pub aspect: f32,
	pub fovy: f32,
	pub znear: f32,
	pub zfar: f32,
}

impl Camera {

	pub fn new(width: f32, height: f32) -> Self {
		Self {
			eye		: (0.0, 1.0, 2.0).into(),
			target: (0.0, 0.0, 0.0).into(),
			up		: Vector3::unit_y(),
			aspect: width / height,
			fovy	: 45.0,
			znear	: 0.1,
			zfar	: 100.0,
		}
	}

	pub fn build_view_projection_matrix(&self) -> (
		Matrix4<f32>, Matrix4<f32>, Matrix4<f32>
	) {

		let view_mat = Matrix4::look_at_rh(
			self.eye, self.target, self.up
		);

		let project_mat = OPENGL_TO_WGPU_MATRIX * perspective(
			Deg(self.fovy), self.aspect, self.znear, self.zfar
		);	

		let view_project_mat = project_mat * view_mat;

		(view_mat, project_mat, view_project_mat)
	}

}

pub fn build_transformations(
	translation:[f32; 3], 
	rotation:[f32; 3], 
	scaling:[f32; 3]
) -> Matrix4<f32> {
	// create transformation matrices
	let trans_mat = Matrix4::from_translation(
		Vector3::new(
			translation[0], 
			translation[1], 
			translation[2]
		)
	);

	let rotate_mat_x = Matrix4::from_angle_x(Rad(rotation[0]));
	let rotate_mat_y = Matrix4::from_angle_y(Rad(rotation[1]));
	let rotate_mat_z = Matrix4::from_angle_z(Rad(rotation[2]));

	let scale_mat = Matrix4::from_nonuniform_scale(
		scaling[0], 
		scaling[1], 
		scaling[2]
	);

	// combine all transformation matrices together to form a final transform matrix: model matrix
	let model_mat = trans_mat * rotate_mat_z * rotate_mat_y * rotate_mat_x * scale_mat;

	// return final model matrix
	model_mat
}


pub fn build_texture_transformations(
    translation: [f32; 2], 
    rotation: f32,  // Only Z-axis rotation (single float)
    scaling: [f32; 2]
) -> Matrix3<f32> {
    let trans_mat = Matrix3::new(
        1.0, 0.0, translation[0],
        0.0, 1.0, translation[1],
        0.0, 0.0, 1.0
    );

    let rotate_mat = Matrix3::from_angle_z(Rad(rotation));

    let scale_mat = Matrix3::new(
        scaling[0], 0.0,        0.0,
        0.0,        scaling[1], 0.0,
        0.0,        0.0,        1.0
    );

    // Order matters! Usually: Scale → Rotate → Translate
    trans_mat * rotate_mat * scale_mat
}

pub fn generate_rotate_data(input: Vec<Vertex>, rotate_deg: f32) -> Vec<Vertex> {
  let rotation: Matrix3<f32> = Matrix3::from_angle_y(Deg(rotate_deg));

	let generated_data = input.iter()
       .map(|&dt| {
            let rotated = rotation * Vector3::new(dt.position[0], dt.position[1], dt.position[2]);
            Vertex {
							position: [rotated[0], rotated[1], rotated[2]],
							color: dt.color, tex_coords: [0.0, 0.0], mode: 0
						}
        })
        .collect();

	generated_data
}

// pub fn build_transformations_with_pivot(
//     translation: [f32; 3],
//     rotation: [f32; 3],
//     scaling: [f32; 3],
//     pivot: [f32; 3],
//     aspect_ratio: f32, // NEW
// ) -> Matrix4<f32> {
//     let to_origin = Matrix4::from_translation(Vector3::new(-pivot[0], -pivot[1], -pivot[2]));
//     let back_to_pivot = Matrix4::from_translation(Vector3::new(pivot[0], pivot[1], pivot[2]));

//     let rotate_x = Matrix4::from_angle_x(Rad(rotation[0]));
//     let rotate_y = Matrix4::from_angle_y(Rad(rotation[1]));
//     let rotate_z = Matrix4::from_angle_z(Rad(rotation[2]));

//     let scale = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);

//     // ✅ Aspect ratio correction: flip it if your viewport is wider than tall
//     let aspect_scale = Matrix4::from_nonuniform_scale(1.0, aspect_ratio, 1.0);

//     let translate = Matrix4::from_translation(Vector3::new(translation[0], translation[1], translation[2]));

//     // Final matrix
//     aspect_scale * translate * back_to_pivot * rotate_z * rotate_y * rotate_x * scale * to_origin
// }


pub fn build_transformations_with_pivot(
    translation: [f32; 3],   // ← this moves the object
    rotation: [f32; 3],      // ← this rotates around pivot
    scaling: [f32; 3],
    pivot: [f32; 3],         // ← rotation/scaling center
    aspect_ratio: f32,
) -> Matrix4<f32> {
    let to_origin = Matrix4::from_translation(Vector3::new(-pivot[0], -pivot[1], -pivot[2]));
    let back_to_pivot = Matrix4::from_translation(Vector3::new(pivot[0], pivot[1], pivot[2]));

    let rotate_x = Matrix4::from_angle_x(Rad(rotation[0]));
    let rotate_y = Matrix4::from_angle_y(Rad(rotation[1]));
    let rotate_z = Matrix4::from_angle_z(Rad(rotation[2]));

    let scale = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);

    let aspect_scale = Matrix4::from_nonuniform_scale(1.0, aspect_ratio, 1.0);

    let translate = Matrix4::from_translation(Vector3::new(translation[0], translation[1], translation[2]));

    aspect_scale * translate * back_to_pivot * rotate_z * rotate_y * rotate_x * scale * to_origin
}
