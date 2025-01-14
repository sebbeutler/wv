#![allow(unused)]

use std::collections::HashMap;
use std::time::{Duration, Instant};
use tauri::Manager;
use wgpu::util::DeviceExt;
use wgpu::*;

const VERTICES: &[[f32; 3]; 4] = &[
    [0.0, 0.5, 0.0],
    [-0.5, -0.5, 0.0],
    [0.5, -0.5, 0.0],
    [0.5, -0.5, 0.0],
];

pub enum BufferBindingType {
    Vertex,
    BindGroup,
}

pub struct BufferGroup {
    id: u32,
    binding_type: BufferBindingType,
    bind_group: Option<BindGroup>,
    buffers: HashMap<String, Buffer>,
}

impl BufferGroup {
    pub fn new(id: u32, bind_group: Option<BindGroup>) -> Self {
        BufferGroup {
            id,
            binding_type: match (&bind_group) { Some(_) => BufferBindingType::BindGroup, None => BufferBindingType::Vertex },
            bind_group,
            buffers: HashMap::new(),
        }
    }

    pub fn with_buffer(mut self, label: String, buffer: Buffer) -> Self {
        self.buffers.insert(label, buffer);
        self
    }
}

pub struct Shader {
    pipeline: RenderPipeline,
    buffers: Vec<BufferGroup>,
    time_start: std::time::Instant,
}

impl Shader {
    // TODO: Return an error if we cannot find the buffer
    fn update_buffer(&self, queue: &wgpu::Queue, label: String, data: &[u8], offset: u64) {
        let buffer = self.buffers.iter().find_map(|bg| bg.buffers.get(&label))
            .expect(format!("Unkwon buffer '{}'", label).as_str());
        queue.write_buffer(buffer, offset, data);
    }

    fn ellapsed(&self) -> f32 {
        (std::time::Instant::now() - self.time_start).as_secs_f32()
    }
}

pub struct GraphicsHandle {
    pub surface: Surface<'static>,
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub swapchain_info: SurfaceCapabilities,
    pub shaders: Vec<Shader>,
}
pub type GraphicsHandleMutex = std::sync::Mutex<GraphicsHandle>;

// pub fn create_graphics_surface<M, R>(window: tauri::WebviewWindow) -> GraphicsHandle where R : tauri::Runtime, M: tauri::Manager<R> {
pub fn create_graphics(window: tauri::WebviewWindow) -> GraphicsHandle {
    let size = tao::dpi::LogicalSize::new(800, 800);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    let surface = unsafe { instance
        .create_surface(window) }
        .expect("Failed to create surface");
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None, // Some(&surface),
        ..Default::default()
    }))
    .expect("Failed to find suitable adapter");
    let swapchain_info = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_info.formats[0];
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            // Make sure we use the texture resolution limits from the adapter
            // so we can support images the size of the swapchain.
            required_limits:
                wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::Performance,
        },
        None,
    ))
    .expect("Failed to create device");

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 2,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
    };

    surface.configure(
        &device,
        &config
    );

    GraphicsHandle {
        surface,
        instance,
        adapter,
        device,
        queue,
        swapchain_info,
        shaders: Vec::new(),
    }
}

pub fn create_shader_pipeline(
    device: &Device,
    swapchain_format: TextureFormat,
    source: &str,
) -> Shader {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });


    let primitive = wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleStrip, // Switch this to TriangleStrip or TriangleList
        strip_index_format: None, // Some(wgpu::IndexFormat::Uint16),     // Required for strips
        front_face: wgpu::FrontFace::Ccw,               // Counter-clockwise is front face
        cull_mode: Some(wgpu::Face::Back),              // Back-face culling
        unclipped_depth: false,
        polygon_mode: wgpu::PolygonMode::Fill,
        conservative: false,
    };

    let (pipeline_layout, mut shader_buffers) = create_pipeline_layout(device);

    let canvas_vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Canvas Vertex Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../assets/canvas.wgsl").into()),
    });
    let canvas_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, // Total size of one vertex
        step_mode: wgpu::VertexStepMode::Vertex, // Data is fetched per vertex
        attributes: &[
            // Position attribute
            wgpu::VertexAttribute {
                offset: 0, // Position starts at the beginning of the vertex
                shader_location: 0, // Matches @location(0) in the shader
                format: wgpu::VertexFormat::Float32x3, // vec3<f32>
            },
        ],
    };
    let canvas_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Canvas Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES), // Convert vertices to raw bytes
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });
    shader_buffers.push(BufferGroup::new(0, None).with_buffer("Canvas Vertex Buffer".to_string(), canvas_buffer));

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &canvas_vs,
            entry_point: Some("vs_canvas_bounds"),
            buffers: &[canvas_buffer_layout],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: swapchain_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive,
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    Shader {
        pipeline: render_pipeline,
        buffers: shader_buffers,
        time_start: std::time::Instant::now()
    }
}

pub fn create_pipeline_layout(device: &Device) -> (wgpu::PipelineLayout, Vec<BufferGroup>) {
    let mut bind_groups: Vec<BufferGroup> = Vec::new();
    // Create the time buffer
    let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Time Buffer"),
        contents:  bytemuck::cast_slice(&[0.0f32]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Define screen size
    let surface_size = [800.0f32, 628.0f32];

    // Create uniform buffer
    let surface_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Surface Size Buffer"),
        contents: bytemuck::cast_slice(&surface_size),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Create a bind group layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    // Create a bind group
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: time_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: surface_size_buffer.as_entire_binding(),
            }
        ],
    });
    bind_groups.push(
        BufferGroup::new(0, Some(bind_group))
        .with_buffer("Time Buffer".to_string(), time_buffer)
        .with_buffer("Surface Size Buffer".to_string(), surface_size_buffer),
    );

    // Create a pipeline layout
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    (pipeline_layout, bind_groups)
}

impl GraphicsHandle {
    fn render(&self, shader: &Shader) {
        // info!("RENDERING");
        // Update ellapsed time:
        shader.update_buffer(
            &self.queue,
            "Time Buffer".to_string(),
            bytemuck::cast_slice(&[shader.ellapsed()]),
            0);

        let texture = self.surface.get_current_texture()
            .expect("Cannot get current texture");
        let view = texture.texture.create_view(
            &wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
            render_pass.set_pipeline(&shader.pipeline);

            for buffer_group in &shader.buffers {
                let _ = match buffer_group.binding_type {
                    BufferBindingType::BindGroup =>
                        render_pass.set_bind_group(buffer_group.id.clone(), &buffer_group.bind_group, &[]),
                    BufferBindingType::Vertex => {
                        for (i, (_, vertex_buffer)) in buffer_group.buffers.iter().enumerate() {
                            render_pass.set_vertex_buffer(i as u32, vertex_buffer.slice(..));
                        }
                    },
                };
            }

            render_pass.draw(0..4, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        texture.present();
    }

    pub fn render_shader(&self, shader_id: usize) {
        let shader = self
            .shaders
            .get(shader_id)
            .expect("Unkown pipeline id, cannot render.");
        self.render(shader);
    }

    pub fn render_all(&self) {
        for shader in &self.shaders {
            self.render(shader);
        }
    }

    pub fn with_shader(mut self, source: &str) -> Self {
        let shader =
            create_shader_pipeline(&self.device, self.swapchain_info.formats[0], source.into());
        self.shaders.push(shader);
        self
    }

    pub fn apply_shader(self, shader_id: usize) -> Self {
        self.render_shader(shader_id);
        self
    }

    pub fn spawn_render_loop(self) {
        tauri::async_runtime::spawn( async move {
            loop {
                self.render_all();
                let shad = self.shaders.get(0).unwrap();
                let ell = shad.ellapsed();
                let new_vert = VERTICES
                .iter()
                .map(|v| [v[0] + f32::sin(ell), v[1] + f32::cos(ell), 0.0])
                .collect::<Vec<_>>();

                shad.update_buffer(&self.queue, "Canvas Vertex Buffer".to_string(), bytemuck::cast_slice(&new_vert), 0);
                // self.render_shader(0);
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        }
        );
    }

    pub fn as_mutex(self) -> GraphicsHandleMutex {
        std::sync::Mutex::new(self)
    }
}

// Threaded Render
// tauri::async_runtime::spawn(async move {
//     let frame_duration = Duration::from_millis(16); // ~60 FPS
//     loop {
//         let start = Instant::now();

//         {
//             let texture = surface.get_current_texture().expect("Cannot get current texture");
//             render_state.render(texture);
//         }

//         let elapsed = start.elapsed();
//         if elapsed < frame_duration {
//             tokio::time::sleep(frame_duration - elapsed).await;
//         }
//     }
// });

