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
