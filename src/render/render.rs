use libvktypes::{
    swapchain,
    shader,
    graphics,
    memory,
    surface,
    cmd
};

const MAT_F32_4x4_SIZE: u64 = (std::mem::size_of::<f32>() as u64)*16;

use crate::scene::Scene;
use crate::render::shaders;
use crate::gpu_backend::GPUBackend;

pub struct Render
{
    m_swapchain: swapchain::Swapchain,
    m_images: Vec<memory::ImageMemory>,
    m_frames: Vec<memory::Framebuffer>,
    m_render_pass: graphics::RenderPass,
    m_vert_shader: shader::Shader,
    m_frag_shader: shader::Shader,
    m_descriptor: graphics::PipelineDescriptor,
    m_pipeline: graphics::Pipeline,
    m_cmd_buffers: Vec<Vec<cmd::ExecutableBuffer>>,
}

impl Render {
    pub fn new(backend: &GPUBackend) -> Render {
        let capabilities = surface::Capabilities::get(backend.hw(), backend.surface()).expect("Failed to get capabilities");

        assert!(capabilities.is_mode_supported(swapchain::PresentMode::FIFO));
        assert!(capabilities.is_flags_supported(memory::UsageFlags::COLOR_ATTACHMENT));

        let surf_format = capabilities.formats().next().expect("No available formats").format;

        let swp_type = swapchain::SwapchainCfg {
            num_of_images: capabilities.min_img_count(),
            format: surf_format,
            color: memory::ColorSpace::SRGB_NONLINEAR,
            present_mode: swapchain::PresentMode::FIFO,
            flags: memory::UsageFlags::COLOR_ATTACHMENT,
            extent: capabilities.extent2d(),
            transform: capabilities.pre_transformation(),
            alpha: capabilities.first_alpha_composition().expect("No alpha composition")
        };

        let swapchain = swapchain::Swapchain::new(backend.lib(), backend.device(), backend.surface(), &swp_type)
            .expect("Failed to create swapchain");

        let vert_shader_type = shader::ShaderCfg {
            path: "VERT_DATA",
            entry: "main",
        };

        let vert_shader =
            shader::Shader::from_glsl(backend.device(), &vert_shader_type, shaders::VERT_SHADER, shader::Kind::Vertex)
            .expect("Failed to create vertex shader module");

        let frag_shader_type = shader::ShaderCfg {
            path: "FRAG_DATA",
            entry: "main",
        };

        let frag_shader =
            shader::Shader::from_glsl(backend.device(), &frag_shader_type, shaders::FRAG_SHADER, shader::Kind::Fragment)
            .expect("Failed to create fragment shader module");

        let descs = graphics::PipelineDescriptor::allocate(backend.device(), &[&[
            // model matrix, binding = 0
            graphics::BindingCfg {
                resource_type: graphics::DescriptorType::UNIFORM_BUFFER,
                stage: graphics::ShaderStage::VERTEX,
                count: 1,
            },
            // camera matrix, binding = 1
            graphics::BindingCfg {
                resource_type: graphics::DescriptorType::UNIFORM_BUFFER,
                stage: graphics::ShaderStage::VERTEX,
                count: 1,
            }
        ]]).expect("Failed to allocate resources");

        let render_pass = graphics::RenderPass::single_subpass(backend.device(), surf_format)
            .expect("Failed to create render pass");

        let vertex_cfg = [
            graphics::VertexInputCfg {
                location: 0,
                binding: 0,
                format: memory::ImageFormat::R16G16B16A16_SFLOAT,
                offset: 0,
            }
        ];

        let pipe_type = graphics::PipelineCfg {
            vertex_shader: &vert_shader,
            vertex_size: std::mem::size_of::<[f32; 2]>() as u32,
            vert_input: &vertex_cfg,
            frag_shader: &frag_shader,
            geom_shader: None,
            topology: graphics::Topology::TRIANGLE_LIST,
            extent: capabilities.extent2d(),
            push_constant_size: 0,
            render_pass: &render_pass,
            subpass_index: 0,
            enable_depth_test: true,
            enable_primitive_restart: false,
            cull_mode: graphics::CullMode::BACK,
            descriptor: &descs,
        };

        let pipeline = graphics::Pipeline::new(backend.device(), &pipe_type).expect("Failed to create pipeline");

        let images = swapchain.images().expect("Failed to get images");

        let mut cmd_buffers: Vec<Vec<cmd::ExecutableBuffer>> = Vec::new();

        let frames: Vec<memory::Framebuffer> = images.iter()
            .map(|image| {
                let frames_cfg = memory::FramebufferCfg {
                    render_pass: &render_pass,
                    images: &[image.view(0)],
                    extent: capabilities.extent2d(),
                };

                cmd_buffers.push(Vec::new());

                memory::Framebuffer::new(backend.device(), &frames_cfg).expect("Failed to create framebuffers")
            })
            .collect();

        Render {
            m_swapchain: swapchain,
            m_images: images,
            m_frames: frames,
            m_render_pass: render_pass,
            m_vert_shader: vert_shader,
            m_frag_shader: frag_shader,
            m_descriptor: descs,
            m_pipeline: pipeline,
            m_cmd_buffers: cmd_buffers,
        }
    }

    pub fn write_cmds(&mut self, backend: &GPUBackend, scene: &Scene) {
        self.m_frames.iter().map(|frame| {
            let mut frame_cmd_buffer: Vec<cmd::ExecutableBuffer> = Vec::new();

            for item in scene.items() {
                let cmd_buffer = backend.cmd_buffer();

                cmd_buffer.begin_render_pass(&self.m_render_pass, &frame);
                cmd_buffer.bind_graphics_pipeline(&self.m_pipeline);
                cmd_buffer.bind_vertex_buffers(&[data.vertex_view(0, vertex_cfg[0].offset)]);
                cmd_buffer.bind_index_buffer(data.view(1), 0, memory::IndexBufferType::UINT32);
                cmd_buffer.bind_resources(&self.m_pipeline, &self.m_descriptor, &[]);
                cmd_buffer.draw_indexed(INDICES.len() as u32, 1, 0, 0, 0);
                cmd_buffer.end_render_pass();

                frame_cmd_buffer.push(cmd_buffer.commit().expect("Failed to commit buffer"));
            }

            self.m_cmd_buffers.push(frame_cmd_buffer);
        })
        .collect();
    }
}