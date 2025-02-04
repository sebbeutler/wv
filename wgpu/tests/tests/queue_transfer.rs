//! Tests for buffer copy validation.

use wgpu_test::{fail, gpu_test, GpuTestConfiguration};

#[gpu_test]
static QUEUE_WRITE_TEXTURE_OVERFLOW: GpuTestConfiguration =
    GpuTestConfiguration::new().run_sync(|ctx| {
        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: 146,
                height: 25,
                depth_or_array_layers: 192,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let data = vec![255; 128];

        fail(
            &ctx.device,
            || {
                ctx.queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d { x: 0, y: 0, z: 1 },
                        aspect: wgpu::TextureAspect::All,
                    },
                    &data,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(879161360),
                        //bytes_per_image: 4294967295,
                        rows_per_image: Some(4294967295 / 879161360),
                    },
                    wgpu::Extent3d {
                        width: 3056263286,
                        height: 64,
                        depth_or_array_layers: 4294967295,
                    },
                );
            },
            Some("end up overrunning the bounds of the destination texture"),
        );
    });
