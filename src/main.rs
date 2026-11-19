mod engine;

use engine::image_view::ImageViews;
use engine::swapchain::*;
use engine::window::AppWindow;
use winit::event_loop::ControlFlow;

use crate::engine::renderer::Renderer;

fn main() {
    println!("TON-618 Renderer");

    let app = AppWindow::new();

    let vulkan = engine::device::VulkanDevice::new(&app.window);

    vulkan.check_surface_support();
    vulkan.print_surface_capabilities();
    vulkan.print_surface_formats();
    vulkan.print_present_modes();

    let swapchain = Swapchain::new(
        &vulkan.instance,
        &vulkan.device,
        vulkan.physical_device,
        &vulkan.surface_loader,
        vulkan.surface,
        &app.window,
        vulkan.graphics_queue_family,
    );

    let image_views = ImageViews::new(&vulkan.device, &swapchain.images, swapchain.format);

    let render_pass = engine::render_pass::RenderPass::new(&vulkan.device, swapchain.format);

    let framebuffers = engine::framebuffer::Framebuffers::new(
        &vulkan.device,
        render_pass.render_pass,
        &image_views.views,
        swapchain.extent,
    );

    let pipeline = engine::pipeline::GraphicsPipeline::new(
        &vulkan.device,
        render_pass.render_pass,
        swapchain.extent,
    );

    let renderer = Renderer;

    let start_time = std::time::Instant::now();

    println!("Starting event loop");

    app.event_loop
        .run(move |event, window_target| {
            window_target.set_control_flow(ControlFlow::Poll);

            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Closing");
                    window_target.exit();
                }

                winit::event::Event::AboutToWait => {
                    renderer.draw_frame(
                        &vulkan,
                        &swapchain.loader,
                        swapchain.swapchain,
                        vulkan.command_buffer,
                        render_pass.render_pass,
                        &framebuffers.framebuffers,
                        swapchain.extent,
                        pipeline.pipeline,
                        pipeline.layout,
                        start_time.elapsed().as_secs_f32(),
                    );
                }

                _ => {}
            }
        })
        .expect("Event loop failed");
}
