use libvktypes::{
	window,
    extensions,
    libvk,
    layers,
    surface,
    hw,
	dev,
	swapchain,
	memory,
	shader
};

mod shaders;

const MAT_F32_4x4_SIZE: u64 = (std::mem::size_of::<f32>() as u64)*16;

struct Render
{
	m_lib: libvk::Instance,
	m_surface: surface::Surface,
	m_dev: dev::Device,
	m_swapchain: swapchain::Swapchain,
	m_vert_shader: shader::Shader,
	m_frag_shader: shader::Shader,
}

impl Render {
	pub fn new(window: &window::Window) {
		let mut extensions = extensions::required_extensions(&window);
		extensions.push(extensions::DEBUG_EXT_NAME);
		extensions.push(extensions::SURFACE_EXT_NAME);

		let lib_type = libvk::InstanceType {
			debug_layer: Some(layers::DebugLayer::default()),
			extensions: &extensions,
			..libvk::InstanceType::default()
		};

		let lib = libvk::Instance::new(&lib_type).expect("Failed to load library");

		let surface = surface::Surface::new(&lib, &window).expect("Failed to create surface");

		let hw_list = hw::Description::poll(&lib, Some(&surface)).expect("Failed to list hardware");

		let (hw_dev, queue, _) = hw_list
			.find_first(
				hw::HWDevice::is_discrete_gpu,
				|q| q.is_graphics() && q.is_surface_supported(),
				|_| true
			)
			.expect("Failed to find suitable hardware device");

		let dev_type = dev::DeviceCfg {
			lib: &lib,
			hw: hw_dev,
			extensions: &[extensions::SWAPCHAIN_EXT_NAME],
			allocator: None,
		};

		let device = dev::Device::new(&dev_type).expect("Failed to create device");

		let capabilities = surface::Capabilities::get(&hw_dev, &surface).expect("Failed to get capabilities");

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

		let swapchain = swapchain::Swapchain::new(&lib, &device, &surface, &swp_type).expect("Failed to create swapchain");

		let vert_shader_type = shader::ShaderCfg {
			path: "VERT_DATA",
			entry: "main",
		};

		let vert_shader =
			shader::Shader::from_glsl(&device, &vert_shader_type, shaders::VERT_SHADER, shader::Kind::Vertex)
			.expect("Failed to create vertex shader module");

		let frag_shader_type = shader::ShaderCfg {
			path: "FRAG_DATA",
			entry: "main",
		};

		let frag_shader =
			shader::Shader::from_glsl(&device, &frag_shader_type, shaders::FRAG_SHADER, shader::Kind::Fragment)
			.expect("Failed to create fragment shader module");

		let mem_cfg = memory::MemoryCfg {
			properties: hw::MemoryProperty::HOST_VISIBLE,
			filter: &hw::any,
			buffers: &[
				// background and player
				&memory::BufferCfg {
					size: 4*std::mem::size_of::<f32>() as u64,
					usage: memory::VERTEX,
					queue_families: &[queue.index()],
					simultaneous_access: false,
					count: 2
				},
				&memory::BufferCfg {
					size: 4*3*std::mem::size_of::<u32>() as u64,
					usage: memory::INDEX,
					queue_families: &[queue.index()],
					simultaneous_access: false,
					count: 1
				},
				&memory::BufferCfg {
					size: 2*MAT_F32_4x4_SIZE,
					usage: memory::UNIFORM,
					queue_families: &[queue.index()],
					simultaneous_access: false,
					count: 1
				}
			]
		};
	}
}