use winit::{
    event::*,
    event_loop::EventLoop,
    window::Window,
};
use std::sync::Arc;

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(
    wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: wgpu::InstanceFlags::default(),
        memory_budget_thresholds:
            wgpu::MemoryBudgetThresholds::default(),
        backend_options: Default::default(),
        display: None,
    }
);

        let surface = instance
            .create_surface(window)
            .unwrap();

        let adapter = instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference:
                        wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                    ..Default::default()
                }
            )
            .await
            .unwrap();

        println!(
            "GPU: {}",
            adapter.get_info().name
        );

        let (device, queue) =
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor::default()
                )
                .await
                .unwrap();


        let config =
            surface
                .get_default_config(
                    &adapter,
                    size.width,
                    size.height,
                )
                .unwrap();

        surface.configure(
            &device,
            &config
        );


        Self {
            surface,
            device,
            queue,
            config,
        }
    }


    fn render(&mut self) {

        let frame = self.surface.get_current_texture();

let texture = match frame {
    wgpu::CurrentSurfaceTexture::Success(texture) => texture,

    wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,

    _ => return,
};

let view = texture
    .texture
    .create_view(
        &Default::default()
    );


        let mut encoder =
            self.device
                .create_command_encoder(
                    &Default::default()
                );


        {
            let _pass =
                encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("Space"),
                        color_attachments:
                        &[Some(
                            wgpu::RenderPassColorAttachment {
                                view: &view,
                                depth_slice: None,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load:
                                    wgpu::LoadOp::Clear(
                                        wgpu::Color {
                                            r: 0.001,
                                            g: 0.002,
                                            b: 0.01,
                                            a: 1.0,
                                        }
                                    ),
                                    store:
                                    wgpu::StoreOp::Store,
                                },
                            }
                        )],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    }
                );
        }


        self.queue.submit(
    Some(encoder.finish())
);
    }
}


fn main() {

    env_logger::init();

    let event_loop =
        EventLoop::new().unwrap();


    let window = Arc::new(
    event_loop
        .create_window(
            Window::default_attributes()
                .with_title("TON 618 Renderer")
        )
        .unwrap()
);


    let mut state =
    pollster::block_on(
        State::new(&window)
    );

    let window_clone = window.clone();

    event_loop.run(
        move |event, target| {

            match event {

                Event::WindowEvent {
                    event:
                    WindowEvent::CloseRequested,
                    ..
                } => {
                    target.exit();
                }


                Event::WindowEvent {
                    event:
                    WindowEvent::RedrawRequested,
                    ..
                } => {
                    state.render();
                }


                Event::AboutToWait => {
                    window_clone.request_redraw();
                }


                _ => {}
            }

        }
    ).unwrap();
}