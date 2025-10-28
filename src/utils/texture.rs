use std::rc::Rc;
use image::{GenericImageView, ImageBuffer};

#[derive(Debug, Clone)]
pub struct ImageData {
    pub img: image::DynamicImage,
    pub rgba: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub dimensions: (u32, u32),
}

pub fn load_image(bytes: Vec<u8>) -> Result<ImageData, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(&bytes.as_slice())?;
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();
    
    Ok(ImageData {
        img,
        rgba,
        dimensions,
    })
}

pub fn load_texture(
    device: Rc<wgpu::Device>,
    queue: Rc<wgpu::Queue>,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
    image_data: Option<ImageData>,
) -> Result<wgpu::BindGroup, Box<dyn std::error::Error>> {
    let texture_size = if let Some(image_data) = &image_data {
        // log::info!("Image dimensions: {:?}", image_data.dimensions);
        wgpu::Extent3d {
            width: image_data.dimensions.0,
            height: image_data.dimensions.1,
            depth_or_array_layers: 1,
        }
    } else {
        wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        }
    };

    let format = wgpu::TextureFormat::Rgba8UnormSrgb;

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Image Texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    if let Some(image_data) = image_data {
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &image_data.rgba,
            // The layout of the texture
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * image_data.dimensions.0),
                rows_per_image: Some(image_data.dimensions.1),
            },
            texture_size,
        );
    } else {
        // If no image data is provided, we can create a 1x1 white texture
        let white_pixel = [255u8, 255u8, 255u8, 255u8]; // RGBA white pixel
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &white_pixel,
            // The layout of the texture
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            texture_size,
        );
    }

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: Some("texture_bind_group"),
    });

    Ok(texture_bind_group)

}
