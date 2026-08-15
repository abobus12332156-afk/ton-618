use crate::engine::device::VulkanDevice;
use ash::vk;
use std::slice;

pub struct Renderer;

impl Renderer {
    fn record_command_buffer(
        device: &ash::Device,
        command_buffer: vk::CommandBuffer,
        render_pass: vk::RenderPass,
        framebuffer: vk::Framebuffer,
        extent: vk::Extent2D,
        pipeline: vk::Pipeline,
    ) {
        let begin_info = vk::CommandBufferBeginInfo::default();

        unsafe {
            device
                .begin_command_buffer(command_buffer, &begin_info)
                .unwrap();
        }

        let clear = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.02, 0.02, 0.03, 1.0],
            },
        };

        let render_pass_info = vk::RenderPassBeginInfo::default()
            .render_pass(render_pass)
            .framebuffer(framebuffer)
            .render_area(
                vk::Rect2D::default()
                    .offset(vk::Offset2D { x: 0, y: 0 })
                    .extent(extent),
            )
            .clear_values(std::slice::from_ref(&clear));

        unsafe {
            device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );

            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_draw(command_buffer, 3, 1, 0, 0);
            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer).unwrap();
        }
    }

    pub fn draw_frame(
        &self,
        vulkan: &VulkanDevice,
        swapchain_loader: &ash::khr::swapchain::Device,
        swapchain: vk::SwapchainKHR,
        command_buffer: vk::CommandBuffer,
        render_pass: vk::RenderPass,
        framebuffers: &[vk::Framebuffer],
        extent: vk::Extent2D,
        pipeline: vk::Pipeline,
    ) {
        unsafe {
            vulkan
                .device
                .reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())
                .expect("Failed to reset command buffer");
        }

        let (image_index, _) = unsafe {
            swapchain_loader
                .acquire_next_image(
                    swapchain,
                    u64::MAX,
                    vulkan.semaphore, // ИСПРАВЛЕНО: берём семафор из переданного экземпляра vulkan
                    vk::Fence::null(),
                )
                .expect("Failed to acquire image")
        };

        let framebuffer = framebuffers[image_index as usize];

        // ИСПРАВЛЕНО: Передаем device, обращаясь к нему через vulkan.device
        Self::record_command_buffer(
            &vulkan.device,
            command_buffer,
            render_pass,
            framebuffer,
            extent,
            pipeline,
        );

        println!("Image acquired: {}", image_index);

        unsafe {
            vulkan
                .device
                .wait_for_fences(std::slice::from_ref(&vulkan.fence), true, u64::MAX)
                .expect("Failed to wait for fences");
        }

        unsafe {
            vulkan
                .device
                .reset_fences(std::slice::from_ref(&vulkan.fence))
                .expect("Failed to reset fences");
        }

        let wait_semaphores = [vulkan.semaphore];
        let signal_semaphores = [vulkan.render_finished_semaphore];
        let command_buffers = [command_buffer];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        unsafe {
            vulkan
                .device
                .queue_submit(
                    vulkan.graphics_queue,
                    std::slice::from_ref(&submit_info),
                    vulkan.fence,
                )
                .expect("Failed to submit queue");
        }
        println!("Command buffer submitted to GPU");

        let swapchain = [swapchain];
        let image_indices = [image_index];
        let signal_semaphores_present = [vulkan.render_finished_semaphore];

        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores_present)
            .swapchains(&swapchain)
            .image_indices(&image_indices);

        unsafe {
            swapchain_loader
                .queue_present(vulkan.graphics_queue, &present_info)
                .expect("Failed to present image");
        }

        println!("Frame presented to screen");
    }
}
