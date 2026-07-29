use ash::vk;

pub struct ImageViews {
    pub views: Vec<vk::ImageView>,
}

impl ImageViews {
    pub fn new(device: &ash::Device, images: &[vk::Image], format: vk::Format) -> Self {
        let mut views = Vec::new();

        for image in images {
            let create_info = vk::ImageViewCreateInfo::default()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(
                    vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1),
                );

            let view = unsafe {
                device
                    .create_image_view(&create_info, None)
                    .expect("Failed to create image view")
            };

            views.push(view);
        }

        println!("Image views created: {}", views.len());

        Self { views }
    }
}
