use ash::vk;

pub struct Renderer {}

impl Renderer {
    pub fn draw_frame(
        device: &ash::Device,
        command_buffer: vk::CommandBuffer,
        render_pass: vk::RenderPass,
        framebuffer: vk::Framebuffer,
        extent: vk::Extent2D,
        pipeline: vk::Pipeline,
    ) {
    }
}
