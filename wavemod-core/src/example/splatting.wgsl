// --------------------- Compute Shader: Generate 3D Grid ---------------------
struct Gaussian {
    position: vec3<f32>,  // Center of the Gaussian kernel
    color: vec3<f32>,     // RGB color of the Gaussian
    sigma: f32           // Standard deviation (spread of the Gaussian)
};

struct Uniforms {
    gridSize: vec3<u32>,  // Dimensions of the 3D grid (e.g., 1000x1000x1000)
    numKernels: u32      // Number of Gaussian kernels
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;          // Uniform for grid size and kernel count
@group(0) @binding(1) var<storage, read> kernels: array<Gaussian>; // Gaussian kernels data
@group(0) @binding(2) var<storage, read_write> grid: array<vec4<f32>>; // 3D grid as flat array

// Compute Gaussian contribution for a single grid point
fn gaussian_contribution(position: vec3<f32>, kernel: Gaussian) -> vec3<f32> {
    let dist = distance(position, kernel.position); // Distance to Gaussian center
    let weight = exp(-0.5 * (dist / kernel.sigma) * (dist / kernel.sigma)); // Gaussian decay
    return kernel.color * weight; // Weighted color
}

@compute @workgroup_size(8, 8, 8) // Optimized for GPU parallelism
fn compute_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Calculate 1D index for the flat grid array
    let idx = global_id.x + global_id.y * uniforms.gridSize.x + global_id.z * uniforms.gridSize.x * uniforms.gridSize.y;
    if (idx >= arrayLength(&grid) ) { return; } // Bounds check

    // Map global ID to normalized grid coordinates
    let position = vec3<f32>(
        f32(global_id.x) / f32(uniforms.gridSize.x - 1),
        f32(global_id.y) / f32(uniforms.gridSize.y - 1),
        f32(global_id.z) / f32(uniforms.gridSize.z - 1)
    );

    // Compute accumulated color for this grid point
    var color: vec3<f32> = vec3(0.0);
    for (var i = 0u; i < uniforms.numKernels; i++) {
        color += gaussian_contribution(position, kernels[i]);
    }

    // Write color to the grid (adding alpha = 1.0 for full opacity)
    grid[idx] = vec4<f32>(color, 1.0);
}

// --------------------- Render Shader: Slicing ---------------------
@fragment
fn slice_render_main(@builtin(position) fragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = fragCoord.xy / resolution.xy; // Map screen to normalized coordinates
    let sliceZ = 0.5; // Middle slice (z = 0.5)
    let texCoord = vec3<f32>(uv, sliceZ); // Map to 3D grid texture
    return textureSample(gridTexture, texCoord);
}

// --------------------- Render Shader: Raymarching ---------------------
@fragment
fn raymarch_render_main(@builtin(position) fragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    var rayOrigin = vec3<f32>(fragCoord.xy / resolution.xy, 0.0); // Start at near plane
    let rayDir = vec3<f32>(0.0, 0.0, 1.0); // March along the z-axis
    var color = vec3<f32>(0.0);
    var alpha = 0.0;

    // Raymarching loop
    for (var t = 0.0; t < 1.0; t += 0.01) {
        let samplePos = rayOrigin + t * rayDir; // Move along the ray
        if (samplePos.x >= 0.0 && samplePos.x <= 1.0 &&
            samplePos.y >= 0.0 && samplePos.y <= 1.0 &&
            samplePos.z >= 0.0 && samplePos.z <= 1.0) {
            let value = textureSample(gridTexture, samplePos); // Sample from 3D texture
            let sampleColor = value.rgb;
            let sampleAlpha = value.a;
            color += (1.0 - alpha) * sampleAlpha * sampleColor; // Composite color
            alpha += (1.0 - alpha) * sampleAlpha;
            if (alpha >= 1.0) { break; } // Early ray termination
        }
    }
    return vec4<f32>(color, alpha);
}

// --------------------- Render Shader: Point Cloud ---------------------
@vertex
fn pointcloud_vertex_main(@location(0) position: vec3<f32>, @location(1) color: vec3<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 1.0); // Pass position to clip space
}

@fragment
fn pointcloud_fragment_main(@location(1) color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0); // Pass color as output
}

// --------------------- Render Shader: Cube Mapping ---------------------
@fragment
fn cubemap_render_main(@builtin(position) fragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = fragCoord.xy / resolution.xy; // Screen-space coordinates
    let texCoord = vec3<f32>(uv, 0.5); // Map to 3D grid texture
    return textureSample(gridTexture, texCoord); // Sample from the 3D texture
}
