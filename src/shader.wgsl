struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera: CameraUniform;

struct TxUniform {
    tx_position: vec2<f32>,
    angle: f32,
    _padding: f32,
};
@group(0) @binding(1) var<uniform> tx_uniform: TxUniform;

// Textured Fragment Shader
@group(1) @binding(0) var my_texture: texture_2d<f32>;
@group(1) @binding(1) var my_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>, // Changed from vec3 to vec2 (UV is 2D)
    @location(2) color: vec3<f32>,
    @location(3) mode: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) v_color: vec3<f32>,
    @location(1) v_tex_coords: vec2<f32>, // Changed from vec4 to vec2
    @location(2) v_mode: u32,
    @location(3) v_tx_rotation: f32,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    if( model.mode == 0) {
        // If mode is 0, use the position as is
        output.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    } else {
        output.clip_position = vec4<f32>( model.position.xy + tx_uniform.tx_position, 0.0, 1.0);
    }

    // output.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    output.v_color = model.color;
    // Apply texture transformation in vertex shader (more efficient)
     
    output.v_tex_coords = (vec3<f32>(model.tex_coords, 1.0)).xy;    
    output.v_mode = model.mode;
    output.v_tx_rotation = tx_uniform.angle;
    return output;
}

// @fragment
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//  let brightness = clamp(in.v_tx_rotation / 360.0, 0.0, 1.0);
//     return vec4<f32>(brightness, 0.0, 0.0, 1.0); // red color based on angle
// }

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // return vec4<f32>(in.v_color, 1.0);

    if (in.v_mode == 0) {   
        return vec4<f32>(in.v_color, 1.0);
    } 
    else {
        let center = vec2(0.5, 0.5); // Rotate around center
        let cos_a = cos(in.v_tx_rotation);
        let sin_a = sin(in.v_tx_rotation);

        // Translate to origin, rotate, translate back
        let rotated_uv = mat2x2(
            cos_a, -sin_a,
            sin_a,  cos_a
        ) * (in.v_tex_coords - center) + center;
        
        let color = textureSample(my_texture, my_sampler, rotated_uv);

        // discard pixel if alpha is 0
        if (color.a < 0.1) {
            discard;
        }
       
       return color;
    }
}


// @fragment 
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//   let white = vec4f(1, 1, 1, 0.4);
//   let color = vec4f(in.v_color, 1.0);

//   let grid = vec2u(in.clip_position.xy) / 8;
//   let checker = (grid.x + grid.y) % 2 == 1;

//   return select(white, color, checker);
// }
