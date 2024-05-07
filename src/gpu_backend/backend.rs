use libvktypes::{
	window,
	extensions,
	libvk,
	layers,
	surface,
	hw,
	dev,
    cmd,
    queue
};

pub struct GPUBackend {
	m_lib: libvk::Instance,
	m_surface: surface::Surface,
	m_hw: hw::HWDevice,
	m_dev: dev::Device,
    m_pool: cmd::Pool,
    m_queue: queue::Queue,
}

impl GPUBackend {
	pub fn new(window: &window::Window) -> GPUBackend {
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

        let cmd_pool_type = cmd::PoolCfg {
            queue_index: queue.index(),
        };

        let cmd_pool = cmd::Pool::new(&device, &cmd_pool_type).expect("Failed to allocate command pool");

        let queue_cfg = queue::QueueCfg {
            family_index: queue.index(),
            queue_index: 0
        };

        let cmd_queue = queue::Queue::new(&device, &queue_cfg);

		GPUBackend {
			m_lib: lib,
			m_surface: surface,
			m_hw: hw_dev.clone(),
			m_dev: device,
            m_pool: cmd_pool,
            m_queue: cmd_queue,
		}
	}

    pub fn device(&self) -> &dev::Device {
        &self.m_dev
    }

    pub fn surface(&self) -> &surface::Surface {
        &self.m_surface
    }

    pub fn hw(&self) -> &hw::HWDevice {
        &self.m_hw
    }

    pub fn lib(&self) -> &libvk::Instance {
        &self.m_lib
    }

    pub fn cmd_buffer(&self) -> cmd::Buffer {
        self.m_pool.allocate().expect("Failed to allocate command pool")
    }
}