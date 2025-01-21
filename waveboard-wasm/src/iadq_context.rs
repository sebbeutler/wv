use wgpu::{Instance, Surface};

use crate::{Renderer, SurfaceWrapper, SharedPtr};

use winit::window::Window;

use crate::utils::get_adapter_with_capabilities_or_from_env;

#[derive(Clone)]
pub struct IADQContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl IADQContext {
    pub(crate) async fn init_async<R: Renderer>(surface: &mut SurfaceWrapper, window: SharedPtr<Window>) -> Self {
        log::info!("Initializing wgpu...");

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
        surface.pre_adapter(&instance, window);

        let adapter = get_adapter_with_capabilities_or_from_env(
            &instance,
            &R::required_features(),
            &R::required_downlevel_capabilities(),
            &surface.get(),
        )
        .await;
        // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the surface.
        let needed_limits = R::required_limits().using_resolution(adapter.limits());

        let trace_dir = std::env::var("WGPU_TRACE");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: (R::optional_features() & adapter.features())
                        | R::required_features(),
                    required_limits: needed_limits,
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                trace_dir.ok().as_ref().map(std::path::Path::new),
            )
            .await
            .expect("Unable to find a suitable GPU adapter!");

        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }
}

async fn create_iasdq_context(
    instance: Instance,
    surface: Surface<'static>,
    physical_size: (u32, u32),
) -> IADQContext {
    let (adapter, device, queue) = request_device(&instance, &surface).await;

    let caps = surface.get_capabilities(&adapter);
    let prefered = caps.formats[0];

    let format = if cfg!(all(target_arch = "wasm32", not(feature = "webgl"))) {
        // Chrome WebGPU doesn't support sRGB:
        // unsupported swap chain format "xxxx8unorm-srgb"
        prefered.remove_srgb_suffix()
    } else {
        prefered
    };
    let view_formats = if cfg!(feature = "webgl") {
        // panicked at 'Error in Surface::configure: Validation Error
        // Caused by:
        // Downlevel flags DownlevelFlags(SURFACE_VIEW_FORMATS) are required but not supported on the device.
        vec![]
    } else if cfg!(target_os = "android") {
        // TODO:HarmonyOS 不支持 view_formats 格式
        // format 的值与 view_formats 的值一致时，configure 内部会自动忽略 view_formats 的值
        //
        // Android 不支持 view_formats:
        // Downlevel flags DownlevelFlags(SURFACE_VIEW_FORMATS) are required but not supported on the device.
        // This is not an invalid use of WebGPU: the underlying API or device does not support enough features
        // to be a fully compliant implementation. A subset of the features can still be used.
        // If you are running this program on native and not in a browser and wish to work around this issue,
        // call Adapter::downlevel_properties or Device::downlevel_properties to get a listing of the features the current platform supports.
        vec![format]
    } else if format.is_srgb() {
        vec![format, format.remove_srgb_suffix()]
    } else {
        vec![format.add_srgb_suffix(), format.remove_srgb_suffix()]
    };

    let mut config = surface
        .get_default_config(&adapter, physical_size.0, physical_size.1)
        .expect("Surface isn't supported by the adapter.");

    config.view_formats = view_formats;
    config.format = format;

    surface.configure(&device, &config);

    IADQContext {
        instance: instance,
        // surface: surface,
        // config,
        adapter: adapter,
        device: device,
        queue: queue,
    }
}

async fn request_device(
    instance: &Instance,
    surface: &Surface<'static>,
) -> (wgpu::Adapter, wgpu::Device, wgpu::Queue) {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::from_env()
                .unwrap_or(wgpu::PowerPreference::HighPerformance),
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await
        .expect("No suitable GPU adapters found on the system!");

    let adapter_info = adapter.get_info();
    println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

    let base_dir = std::env::var("CARGO_MANIFEST_DIR");
    let _trace_path = if let Ok(base_dir) = base_dir {
        Some(std::path::PathBuf::from(&base_dir).join("WGPU_TRACE_ERROR"))
    } else {
        None
    };

    let res = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: adapter.features(),
                required_limits: adapter.limits(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )
        .await;

    match res {
        Err(err) => {
            panic!("request_device failed: {err:?}");
        }
        Ok(tuple) => (adapter, tuple.0, tuple.1),
    }
}
