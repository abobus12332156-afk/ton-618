use ash::vk;

pub struct Framebuffers {
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl Framebuffers {
    pub fn new(
        device: &ash::Device,
        render_pass: vk::RenderPass,
        image_views: &[vk::ImageView],
        extent: vk::Extent2D,
    ) -> Self {
        let mut framebuffers = Vec::new();

        for &image_view in image_views {
            let attachments = [image_view];

            let create_info = vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);

            let framebuffer = unsafe {
                device
                    .create_framebuffer(&create_info, None)
                    .expect("Failed to create framebuffer")
            };

            framebuffers.push(framebuffer);
        }

        println!("Framebuffer created: {}", framebuffers.len());

        Self { framebuffers }
    }
}
