use ash::vk::{self};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

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
        println!("Command buffer allocated");

        let begin_info = vk::CommandBufferBeginInfo::default();

        unsafe {
            device
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("Failed to begin command buffer");
        }

        println!("Command buffer recording started");

        unsafe {
            device
                .end_command_buffer(command_buffer)
                .expect("Failed to end command buffer")
        }

        println!("Command buffer recording finished");

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
                .queue_submit(
                    self.graphics_queue,
                    std::slice::from_ref(&submit_info),
                    self.fence,
                )
                .expect("Failed to submit command buffer");
        }

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
