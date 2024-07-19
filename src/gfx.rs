#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub position: glam::Vec3,
    pub size: glam::Vec2,
    pub color: Color,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pub position: glam::Vec3,
    pub color: Color,
}

impl Vertex {
    const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
    };
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Index(pub u16);

impl Index {
    const FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint16;
}

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,

    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices: u32,
}

impl Renderer {
    pub fn new<S: Into<wgpu::SurfaceTarget<'static>>>(surface: S) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::empty(),
            ..Default::default()
        });

        let surface = instance.create_surface(surface).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            },
            None,
        ))
        .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,

            // These should be changed by a resize after creation
            width: 0,
            height: 0,

            // Enable Vsync
            present_mode: wgpu::PresentMode::Fifo,

            desired_maximum_frame_latency: 1,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
        };

        let shader = device.create_shader_module(wgpu::include_wgsl!("gfx/shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Vertex::BUFFER_LAYOUT],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertex_buffer"),
            size: 1024 * std::mem::size_of::<Vertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("index_buffer"),
            size: 1024 * std::mem::size_of::<Index>() as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            device,
            queue,
            surface,
            config,

            pipeline,
            vertex_buffer,
            index_buffer,
            indices: 0,
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.config.width = new_width;
        self.config.height = new_height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn batch_rects(&mut self, rects: &[Rect]) {
        let num_rects = rects.len();

        let mut vertices = Vec::<Vertex>::with_capacity(num_rects * 4);
        let mut indices = Vec::<Index>::with_capacity(num_rects * 6);

        for rect in rects.iter() {
            let i = vertices.len().try_into().unwrap();

            vertices.push(Vertex {
                position: rect.position + glam::Vec3::new(rect.size.x, rect.size.y, 0.0),
                color: rect.color,
            });
            vertices.push(Vertex {
                position: rect.position + glam::Vec3::new(rect.size.x, -rect.size.y, 0.0),
                color: rect.color,
            });
            vertices.push(Vertex {
                position: rect.position + glam::Vec3::new(-rect.size.x, -rect.size.y, 0.0),
                color: rect.color,
            });
            vertices.push(Vertex {
                position: rect.position + glam::Vec3::new(-rect.size.x, rect.size.y, 0.0),
                color: rect.color,
            });

            // First triangle
            indices.push(Index(i));
            indices.push(Index(i + 1));
            indices.push(Index(i + 2));

            // Second triangle
            indices.push(Index(i + 2));
            indices.push(Index(i + 3));
            indices.push(Index(i));
        }

        self.load_model(&vertices, &indices);
    }

    pub fn draw(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), Index::FORMAT);
            render_pass.draw_indexed(0..self.indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

impl Renderer {
    fn load_model(&mut self, vertices: &[Vertex], indices: &[Index]) {
        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertices));
        self.queue
            .write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(indices));

        self.indices = indices.len() as u32;
    }
}
