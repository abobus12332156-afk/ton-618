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
