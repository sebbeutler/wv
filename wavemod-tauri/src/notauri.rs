use wgpu::util::DeviceExt;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::dpi::{WebViewBuilder, LogicalPosition, LogicalSize};

fn create_pipeline_layout(device: &wgpu::Device) -> (wgpu::BindGroup, wgpu::PipelineLayout) {
    // Create a buffer
    let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Time Buffer"),
        contents: bytemuck::cast_slice(&[0.0f32]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Create a bind group layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    // Create a bind group
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: time_buffer.as_entire_binding(),
        }],
    });

    // Create a pipeline layout
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    (bind_group, pipeline_layout)
}

pub fn run() {
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    {
        use gtk::prelude::DisplayExtManual;

        gtk::init().unwrap();
        if gtk::gdk::Display::default().unwrap().backend().is_wayland() {
            panic!("This example doesn't support wayland!");
        }

        // we need to ignore this error here otherwise it will be catched by winit and will be
        // make the example crash
        winit::platform::x11::register_xlib_error_hook(Box::new(|_display, error| {
            let error = error as *mut x11_dl::xlib::XErrorEvent;
            (unsafe { (*error).error_code }) == 170
        }));
    }

    let event_loop = EventLoop::new();
    let size = tao::dpi::LogicalSize::new(800, 800);
    let window = WindowBuilder::new()
    .with_decorations(false)
    // There are actually three layer of background color when creating webview window.
    // The first is window background...
    .with_transparent(true)
        .with_inner_size(size)
        .build(&event_loop)
        .expect("Failed to build window");

    use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    apply_blur(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    let webview = WebViewBuilder::new()
        .with_transparent(true) // Make the WebView transparent
        .with_html(r#"
            <!DOCTYPE html>
            <html>
            <body style="background: rgba(0, 0, 0, 0); margin: 0;">
                <div style="color: white; font-size: 24px; text-align: center; margin-top: 20%;">
                    Hello, this is transparent HTML!
                </div>
            </body>
            </html>
        "#)
        .build_as_child(&window)
        .expect("Failed to build WebView");

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    let surface = instance.create_surface(&window).expect("Failed to create surface");
    
    let adapter = pollster::block_on(instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .expect("Failed to find suitable adapter");
    let (device, queue) = pollster::block_on(adapter
        .request_device(
            &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
            required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        ))
        .expect("Failed to create device");


    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
        },
    );
    
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
    });

    let (bind_group, pipeline_layout) = create_pipeline_layout(&device);

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default()
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: swapchain_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default()
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });
    
    event_loop
        .run(move |event, evl, control_flow| {
            *control_flow = ControlFlow::Poll;

            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            while gtk::events_pending() {
                gtk::main_iteration_do(false);
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
          
                    let mut encoder = 
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 1.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            ..Default::default()
                        });
                        render_pass.set_pipeline(&render_pipeline);
                        render_pass.set_bind_group(0, &bind_group, &[]);
                        render_pass.draw(0..3, 0..1);
                    }
          
                    queue.submit(std::iter::once(encoder.finish()));
                    frame.present();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => evl.exit(),
                _ => {}
            }
        })
        .unwrap();
}


