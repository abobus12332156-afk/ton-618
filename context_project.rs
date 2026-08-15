
// File: src/gpu/buffer.rs



// File: src/gpu/shader.rs



// File: src/gpu/mod.rs

pub mod buffer;
pub mod descriptor;
pub mod image;
pub mod pipeline;
pub mod shader;

// File: src/gpu/pipeline.rs



// File: src/gpu/image.rs



// File: src/gpu/descriptor.rs



// File: src/main.rs

mod engine;

use engine::image_view::ImageViews;
use engine::swapchain::*;
use engine::window::AppWindow;

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

    vulkan.record_command_buffer(
        vulkan.command_buffer,
        render_pass.render_pass,
        framebuffers.framebuffers[0],
        swapchain.extent,
        pipeline.pipeline,
    );

    vulkan.submit_commands();

    let renderer = Renderer;
    renderer.draw_frame(
        &vulkan,
        &swapchain.loader,
        swapchain.swapchain,
        vulkan.command_buffer,
        render_pass.render_pass,
        &framebuffers.framebuffers,
        swapchain.extent,
        pipeline.pipeline,
    );
    println!("Starting event loop");

    app.event_loop
        .run(move |event, window_target| match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing");
                window_target.exit();
            }

            event => {
                println!("{event:#?}");
            }
        })
        .expect("Event loop failed");
}

// File: src/physics/kerr.rs



// File: src/physics/mod.rs

pub mod events;
pub mod kerr;
pub mod spacetime;

// File: src/physics/events.rs



// File: src/physics/spacetime.rs



// File: src/renderer/effects.rs



// File: src/renderer/mod.rs

pub mod accretion_disk;
pub mod camera;
pub mod effects;
pub mod particles;
pub mod ton618;

// File: src/renderer/ton618.rs



// File: src/renderer/particles.rs



// File: src/renderer/camera.rs



// File: src/renderer/accretion_disk.rs



// File: src/engine/image_view.rs

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

// File: src/engine/device.rs

use ash::vk::{self};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::engine::render_pass;

pub struct VulkanDevice {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub graphics_queue_family: u32,
    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,
    pub command_buffer_recording: bool,
    pub fence: vk::Fence,
    pub semaphore: vk::Semaphore,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: ash::khr::surface::Instance,
}

impl VulkanDevice {
    pub fn new(window: &winit::window::Window) -> Self {
        let entry = unsafe { ash::Entry::load().expect("Failed to load Vulkan") };

        let app_name = std::ffi::CString::new("TON-618 Renderer").unwrap();

        let app_info = vk::ApplicationInfo::default()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&app_name)
            .engine_version(0)
            .api_version(vk::API_VERSION_1_3);

        let instance_extensions =
            ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw())
                .unwrap();

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(instance_extensions);

        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create Vulkan instance")
        };

        println!("Vulkan instance created");

        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("No Vulkan devices found")
        };

        let physical_device = physical_devices[0];

        let properties = unsafe { instance.get_physical_device_properties(physical_device) };

        let name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) };

        println!("GPU found: {:?}", name);

        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let mut graphics_queue_family = None;

        for (index, queue) in queue_families.iter().enumerate() {
            println!("Queue {}: {:?}", index, queue.queue_flags);

            if queue.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                graphics_queue_family = Some(index as u32);
                break;
            }
        }

        let graphics_queue_family = graphics_queue_family.expect("No graphics queue found");

        let priorities = [1.0_f32];

        let queue_create_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(graphics_queue_family)
            .queue_priorities(&priorities);

        let device_extensions = [ash::khr::swapchain::NAME.as_ptr()];

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_create_info))
            .enabled_extension_names(&device_extensions);

        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device")
        };

        println!("Logical device created");

        let graphics_queue = unsafe { device.get_device_queue(graphics_queue_family, 0) };

        println!("Graphics queue acquired");

        let command_pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(graphics_queue_family)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let command_pool = unsafe {
            device
                .create_command_pool(&command_pool_info, None)
                .expect("Failed to create command pool")
        };

        println!("Command pool created");

        let allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffer = unsafe {
            device
                .allocate_command_buffers(&allocate_info)
                .expect("Failed to allocate command buffer")[0]
        };

        unsafe {
            device
                .reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())
                .expect("Failed to reset command buffer");
        }

        println!("Command buffer allocated");

        let begin_info = vk::CommandBufferBeginInfo::default();

        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        let fence = unsafe {
            device
                .create_fence(&fence_info, None)
                .expect("Failed to create fence")
        };

        println!("Fence created");

        let semaphore_info = vk::SemaphoreCreateInfo::default();

        let semaphore = unsafe {
            device
                .create_semaphore(&semaphore_info, None)
                .expect("Failed to create semaphore")
        };

        println!("Semaphore created");

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
            .expect("Failed to create surface")
        };

        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);

        Self {
            entry,
            instance,
            surface,
            surface_loader,
            physical_device,
            device,
            graphics_queue,
            graphics_queue_family,
            command_pool,
            command_buffer,
            command_buffer_recording: false,
            fence,
            semaphore,
        }
    }

    pub fn submit_commands(&self) {
        let command_buffers = [self.command_buffer];

        let submit_info = vk::SubmitInfo::default()
            .command_buffers(&command_buffers)
            .signal_semaphores(std::slice::from_ref(&self.semaphore));

        unsafe {
            self.device
                .reset_fences(std::slice::from_ref(&self.fence))
                .expect("Failed to reset fence");
        }

        unsafe {
            self.device
                .queue_submit(
                    self.graphics_queue,
                    std::slice::from_ref(&submit_info),
                    self.fence,
                )
                .expect("Failed to submit command buffer");
        }

        unsafe {
            self.device
                .wait_for_fences(std::slice::from_ref(&self.fence), true, u64::MAX)
                .expect("Failed to wait for fence");
        }

        println!("GPU finished rendering");

        println!("Command buffer submitted");
    }

    pub fn check_surface_support(&self) {
        let support = unsafe {
            self.surface_loader
                .get_physical_device_surface_support(
                    self.physical_device,
                    self.graphics_queue_family,
                    self.surface,
                )
                .expect("Failed to check surface support")
        };

        println!("Surface support: {}", support);
    }

    pub fn print_surface_capabilities(&self) {
        let capabilities = unsafe {
            self.surface_loader
                .get_physical_device_surface_capabilities(self.physical_device, self.surface)
                .expect("Failed to get surface capabilities")
        };

        println!("Surface capabilites:");
        println!("Min images: {}", capabilities.min_image_count);
        println!("Max images: {}", capabilities.max_image_count);
        println!(
            "Current extent: {}x{}",
            capabilities.current_extent.width, capabilities.current_extent.height
        );
    }

    pub fn print_surface_formats(&self) {
        let formats = unsafe {
            self.surface_loader
                .get_physical_device_surface_formats(self.physical_device, self.surface)
                .expect("Failed to get surface formats")
        };

        println!("Surface formats:");

        for format in formats {
            println!("{:?} {:?}", format.format, format.color_space);
        }
    }

    pub fn print_present_modes(&self) {
        let modes = unsafe {
            self.surface_loader
                .get_physical_device_surface_present_modes(self.physical_device, self.surface)
                .expect("Failed to get present modes")
        };

        println!("Present modes:");

        for mode in modes {
            println!("{:?}", mode);
        }
    }

    pub fn record_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        render_pass: vk::RenderPass,
        framebuffer: vk::Framebuffer,
        extent: vk::Extent2D,
        pipeline: vk::Pipeline,
    ) {
        let begin_info = vk::CommandBufferBeginInfo::default();

        unsafe {
            self.device
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
            self.device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );
        }

        unsafe {
            self.device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline,
            );
        }

        unsafe {
            self.device.cmd_draw(command_buffer, 3, 1, 0, 0);
        }

        println!("Triangle drawn");

        unsafe {
            self.device.cmd_end_render_pass(command_buffer);
        }
        println!("Render pass ended");

        unsafe {
            self.device.end_command_buffer(command_buffer).unwrap();
        }

        println!("Command buffer ended");
    }

    /*
    pub fn pick_gpu(&self) -> vk::PhysicalDevice {
        let devices = unsafe {
            self.instance
                .enumerate_physical_devices()
                .expect("No Vulkan devices found")
        };

        for device in &devices {
            let properties = unsafe { self.instance.get_physical_device_properties(*device) };

            let name = unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) };

            println!("GPU found: {:?}", name);
        }

        devices[0]
    }

    pub fn find_graphics_queue(&self, physical_device: vk::PhysicalDevice) -> u32 {
        let queues = unsafe {
            self.instance
                .get_physical_device_queue_family_properties(physical_device)
        };

        for (index, queue) in queues.iter().enumerate() {
            println!("Queue {}: {:?}", index, queue.queue_flags);

            if queue.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                println!("Graphics queue: {}", index);

                return index as u32;
            }
        }

        panic!("No graphics queue fond");
    }

    pub fn create_logical_device(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        graphics_queue_family: u32,
    ) -> (ash::Device, vk::Queue) {
        let queue_priorities = [1.0_f32];

        let queue_create_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(graphics_queue_family)
            .queue_priorities(&queue_priorities);

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_create_info));

        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device")
        };

        let graphics_queue = unsafe { device.get_device_queue(graphics_queue_family, 0) };

        (device, graphics_queue)
    }*/
}

// File: src/engine/shader.rs

use ash::util;
use ash::vk;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;

pub struct ShaderModule {
    pub module: vk::ShaderModule,
}

impl ShaderModule {
    pub fn new(device: &ash::Device, path: &str) -> Self {
        let mut file = File::open(path).expect("Failed to open shader");

        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes).expect("Failed to read shader");

        let code = util::read_spv(&mut Cursor::new(bytes)).expect("Invalid SPIR-V");

        let create_info = vk::ShaderModuleCreateInfo::default().code(&code);

        let module = unsafe {
            device
                .create_shader_module(&create_info, None)
                .expect("Failed to create shader module")
        };

        println!("Shader loaded: {}", path);

        Self { module }
    }
}

// File: src/engine/window.rs

use winit::{
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

pub struct AppWindow {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

impl AppWindow {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().expect("Failed to create event loop");

        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("TON-618 Renderer")
                    .with_inner_size(winit::dpi::LogicalSize::new(1280, 720)),
            )
            .expect("Failed to create window");

        println!("Window created");

        Self { event_loop, window }
    }
}

// File: src/engine/sync.rs



// File: src/engine/mod.rs

pub mod app;
pub mod commands;
pub mod device;
pub mod framebuffer;
pub mod image_view;
pub mod pipeline;
pub mod render_pass;
pub mod renderer;
pub mod shader;
pub mod swapchain;
pub mod sync;
pub mod window;

// File: src/engine/swapchain.rs

use ash::vk;
use winit::window::Window;

use crate::engine::device;

pub struct Swapchain {
    pub loader: ash::khr::swapchain::Device,
    pub swapchain: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}
impl Swapchain {
    pub fn choose_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
        *formats
            .iter()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or(&formats[0])
    }

    pub fn choose_present_mode(modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
        if modes.contains(&vk::PresentModeKHR::MAILBOX) {
            vk::PresentModeKHR::MAILBOX
        } else {
            vk::PresentModeKHR::FIFO
        }
    }

    fn choose_extent(window: &Window, caps: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        if caps.current_extent.width != u32::MAX {
            caps.current_extent
        } else {
            let size = window.inner_size();

            vk::Extent2D {
                width: size
                    .width
                    .clamp(caps.min_image_extent.width, caps.max_image_extent.width),
                height: size
                    .height
                    .clamp(caps.min_image_extent.height, caps.max_image_extent.height),
            }
        }
    }

    pub fn acquire_next_image(&self, semaphore: vk::Semaphore) -> u32 {
        let (image_index, _) = unsafe {
            self.loader
                .acquire_next_image(self.swapchain, u64::MAX, semaphore, vk::Fence::null())
                .expect("Failed To acquire swapchain image")
        };

        println!("Acquired image {}", image_index);

        image_index
    }

    pub fn new(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: vk::PhysicalDevice,
        surface_loader: &ash::khr::surface::Instance,
        surface: vk::SurfaceKHR,
        window: &winit::window::Window,
        graphics_queue_family: u32,
    ) -> Self {
        let capabilities = unsafe {
            surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface)
                .expect("Failed to get surface capabilites")
        };

        let formats = unsafe {
            surface_loader
                .get_physical_device_surface_formats(physical_device, surface)
                .expect("Failed to get surface formats")
        };

        let present_modes = unsafe {
            surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface)
                .expect("Failed to get present modes")
        };

        let surface_format = Self::choose_surface_format(&formats);

        let present_mode = Self::choose_present_mode(&present_modes);

        let extent = Self::choose_extent(window, capabilities);

        let mut image_count = capabilities.min_image_count + 1;

        if capabilities.max_image_count > 0 && image_count > capabilities.max_image_count {
            image_count = capabilities.max_image_count;
        }

        let create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let loader = ash::khr::swapchain::Device::new(instance, device);

        let swapchain = unsafe { loader.create_swapchain(&create_info, None).unwrap() };

        let images = unsafe { loader.get_swapchain_images(swapchain).unwrap() };

        println!("Swapchain created");
        println!("Images: {}", images.len());
        println!("Format: {:?}", surface_format.format);
        println!("Extent: {}x{}", extent.width, extent.height);

        Self {
            loader,
            swapchain,
            images,
            format: surface_format.format,
            extent,
        }
    }
}

// File: src/engine/framebuffer.rs

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

// File: src/engine/pipeline.rs

use ash::vk;
use std::ffi::CString;

use crate::engine::shader::ShaderModule;

pub struct GraphicsPipeline {
    pub layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}

impl GraphicsPipeline {
    pub fn new(device: &ash::Device, render_pass: vk::RenderPass, extent: vk::Extent2D) -> Self {
        let layout_info = vk::PipelineLayoutCreateInfo::default();

        let layout = unsafe {
            device
                .create_pipeline_layout(&layout_info, None)
                .expect("Failed to create pipeline layout")
        };

        println!("Pipeline layout created");

        let vertex_shader = ShaderModule::new(device, "src/shaders/triangle.vert.spv");

        let fragment_shader = ShaderModule::new(device, "src/shaders/triangle.frag.spv");

        let entry_point = CString::new("main").unwrap();

        let vertex_stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vertex_shader.module)
            .name(&entry_point);

        let fragment_stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(fragment_shader.module)
            .name(&entry_point);

        let shader_stages = [vertex_stage, fragment_stage];

        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_attribute_descriptions(&[])
            .vertex_binding_descriptions(&[]);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport = vk::Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(extent.width as f32)
            .height(extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);

        let scissor = vk::Rect2D::default()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(extent);

        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(std::slice::from_ref(&viewport))
            .scissors(std::slice::from_ref(&scissor));

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .min_sample_shading(1.0)
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false);

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .blend_enable(false)
            .src_color_blend_factor(vk::BlendFactor::ONE)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD)
            .color_write_mask(vk::ColorComponentFlags::RGBA);

        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(std::slice::from_ref(&color_blend_attachment))
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let dynamic_state = vk::PipelineDynamicStateCreateInfo::default();

        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .dynamic_state(&dynamic_state)
            .layout(layout)
            .render_pass(render_pass)
            .subpass(0);

        let pipeline = unsafe {
            device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    std::slice::from_ref(&pipeline_info),
                    None,
                )
                .expect("Failed to create graphics pipeline")[0]
        };

        println!("Graphics pipeline created");

        unsafe {
            device.destroy_shader_module(vertex_shader.module, None);
            device.destroy_shader_module(fragment_shader.module, None);
        }

        Self { layout, pipeline }
    }
}

// File: src/engine/app.rs



// File: src/engine/render_pass.rs

use ash::vk;

pub struct RenderPass {
    pub render_pass: vk::RenderPass,
}

impl RenderPass {
    pub fn new(device: &ash::Device, format: vk::Format) -> Self {
        let color_attachment = vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let color_attachment_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&color_attachment_ref));

        let dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

        let create_info = vk::RenderPassCreateInfo::default()
            .attachments(std::slice::from_ref(&&color_attachment))
            .subpasses(std::slice::from_ref(&subpass))
            .dependencies(std::slice::from_ref(&dependency));

        let render_pass = unsafe {
            device
                .create_render_pass(&create_info, None)
                .expect("Failed to crate render pass")
        };

        println!("Render pass created");

        Self { render_pass }
    }
}

// File: src/engine/renderer.rs

use crate::engine::device::VulkanDevice;
use ash::vk;

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

    // ИСПРАВЛЕНО: Добавлен &self, аргумент vulkan теперь имеет правильный тип,
    // а дублирующие поля device и graphics_queue удалены, так как они есть внутри vulkan
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
    }
}

// File: src/engine/commands.rs


