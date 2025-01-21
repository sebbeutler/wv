pub trait Renderer: 'static + Sized {
	const SRGB: bool = true;

	fn optional_features() -> wgpu::Features {
		wgpu::Features::empty()
	}

	fn required_features() -> wgpu::Features {
		wgpu::Features::empty()
	}

	fn required_downlevel_capabilities() -> wgpu::DownlevelCapabilities {
		wgpu::DownlevelCapabilities {
			flags: wgpu::DownlevelFlags::empty(),
			shader_model: wgpu::ShaderModel::Sm5,
			..wgpu::DownlevelCapabilities::default()
		}
	}

	fn required_limits() -> wgpu::Limits {
		wgpu::Limits::downlevel_webgl2_defaults() // These downlevel limits will allow the code to run on all possible hardware
	}

	fn init(
		config: &wgpu::SurfaceConfiguration,
		adapter: &wgpu::Adapter,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
	) -> Self;

	fn resize(
		&mut self,
		config: &wgpu::SurfaceConfiguration,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
	);

	fn update(&mut self, event: winit::event::WindowEvent);

	fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue);
}
