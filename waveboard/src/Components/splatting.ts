// ------------------ 1) WebGPU Initialization ------------------
async function initWebGPU() {
  if (!navigator.gpu) {
    throw new Error("WebGPU not supported in this browser.");
  }

  // Request an adapter and a device
  const adapter = await navigator.gpu.requestAdapter();
  if (!adapter) {
    throw new Error("Failed to get GPU adapter.");
  }
  const device = await adapter.requestDevice();
  const queue = device.queue;

  // Create a canvas context for rendering
  const canvas = document.querySelector("canvas");
  const context = canvas.getContext("webgpu");
  const format = navigator.gpu.getPreferredCanvasFormat();
  context.configure({
    device,
    format,
    alphaMode: "premultiplied",
  });

  return { device, queue, context, format };
}

// ------------------ 2) WGSL Shaders ------------------
// We can store the entire WGSL code in a single string or separate strings.
// For clarity, let's define them separately.

// Compute WGSL (only the relevant part):
const computeShaderWGSL = /* wgsl */ `
  // --------------------- Compute Shader: Generate 3D Grid ---------------------
  struct Gaussian {
      position: vec3<f32>,  // Center of the Gaussian kernel
      color: vec3<f32>,     // RGB color of the Gaussian
      sigma: f32            // Standard deviation (spread of the Gaussian)
  };

  struct Uniforms {
      gridSize: vec3<u32>,  // Dimensions of the 3D grid (e.g., 1000x1000x1000)
      numKernels: u32       // Number of Gaussian kernels
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

  @compute @workgroup_size(4, 4, 4) // Optimized for GPU parallelism
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
`;

// Render WGSL (slicing or raymarching). Let's pick SLICING for brevity.
// A simple fragment shader that samples a 3D texture at a fixed z-slice (z=0.5).
const sliceRenderWGSL = /* wgsl */ `
  struct Slice {
    axis: u32,
    position: f32
  }
  @group(0) @binding(0) var<uniform> resolution: vec2<f32>;
  @group(0) @binding(1) var gridTexture: texture_3d<f32>;
  @group(0) @binding(2) var gridSampler: sampler;
  @group(0) @binding(3) var<uniform> sliceBuffer: Slice;

  @fragment
  fn fs_main(@builtin(position) fragCoord: vec4<f32>) -> @location(0) vec4<f32> {
      let uv = fragCoord.xy / resolution;

      // Compute the texture coordinate based on the selected slice axis
      var texCoord: vec3<f32>;
      if (sliceBuffer.axis == 0u) {
          texCoord = vec3<f32>(sliceBuffer.position, uv.y, uv.x); // X slice
      } else if (sliceBuffer.axis == 1u) {
          texCoord = vec3<f32>(uv.x, sliceBuffer.position, uv.y); // Y slice
      } else {
          texCoord = vec3<f32>(uv.x, uv.y, sliceBuffer.position); // Z slice
      }

      // Sample the texture
      return textureSample(gridTexture, gridSampler, texCoord);
  }
  @vertex
  fn vs_main(@builtin(vertex_index) vertexIndex: u32) -> @builtin(position) vec4<f32> {
      // Full-screen triangle (2D)
      let x = f32((vertexIndex << 1) & 2);
      let y = f32((vertexIndex & 2));
      return vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
  }
`;

// ------------------ 3) Pipeline Setup & Execution ------------------
async function runWebGPU() {
  const { device, queue, context, format } = await initWebGPU();

  // ------------------ 3.1) Create Buffers ------------------
  // Example grid size: smaller than 1000³ to keep the demo feasible
  const gridX = 64,
    gridY = 64,
    gridZ = 64;
  const gridElementCount = gridX * gridY * gridZ;
  const gridBufferSize = gridElementCount * 4 * 4; // vec4<f32> = 16 bytes each

  // Uniform data
  const uniformData = new Uint32Array([gridX, gridY, gridZ, 2]); // 2 Gaussians, for example
  const uniformBuffer = device.createBuffer({
    size: uniformData.byteLength,
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
  });

  // Gaussian kernels (2 examples)
  // Each Gaussian: (position.x, position.y, position.z, color.x, color.y, color.z, sigma)
  // We'll store them in a float32-friendly format for easy copying.
  // position, color, sigma => total 7 floats, but struct aligns to 8 floats for "vec4" alignment.
  const kernelArray = new Float32Array([
    // Gaussian 1
    0.3,
    0.3,
    0.3, // position
    1.0,
    0.0,
    0.0, // color (red)
    0.05, // sigma
    0.0, // padding
    // Gaussian 2
    0.7,
    0.7,
    0.7, // position
    0.0,
    0.0,
    1.0, // color (blue)
    0.05, // sigma
    0.0, // padding
  ]);
  const kernelsBuffer = device.createBuffer({
    size: kernelArray.byteLength,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
  });

  // Grid storage buffer
  const gridBuffer = device.createBuffer({
    size: gridBufferSize,
    usage:
      GPUBufferUsage.STORAGE |
      GPUBufferUsage.COPY_SRC |
      GPUBufferUsage.COPY_DST,
  });

  // Upload data to GPU
  device.queue.writeBuffer(uniformBuffer, 0, uniformData);
  device.queue.writeBuffer(kernelsBuffer, 0, kernelArray);

  // ------------------ 3.2) Create Compute Pipeline ------------------
  const computeModule = device.createShaderModule({ code: computeShaderWGSL });
  const computePipeline = device.createComputePipeline({
    layout: "auto",
    compute: {
      module: computeModule,
      entryPoint: "compute_main",
    },
  });

  // Bind group for the compute pipeline
  const computeBindGroup = device.createBindGroup({
    layout: computePipeline.getBindGroupLayout(0),
    entries: [
      { binding: 0, resource: { buffer: uniformBuffer } },
      { binding: 1, resource: { buffer: kernelsBuffer } },
      { binding: 2, resource: { buffer: gridBuffer } },
    ],
  });

  // ------------------ 3.3) Create 3D Texture to hold final data ------------------
  // We'll copy from gridBuffer -> this 3D texture for rendering.
  const gridTexture = device.createTexture({
    size: { width: gridX, height: gridY, depthOrArrayLayers: gridZ },
    format: "rgba8unorm",
    usage:
      GPUTextureUsage.TEXTURE_BINDING |
      GPUTextureUsage.COPY_DST |
      GPUTextureUsage.RENDER_ATTACHMENT,
    dimension: "3d",
  });

  // ------------------ 3.4) Run the Compute Pass ------------------
  {
    const commandEncoder = device.createCommandEncoder();
    const pass = commandEncoder.beginComputePass();
    pass.setPipeline(computePipeline);
    pass.setBindGroup(0, computeBindGroup);

    // Dispatch: Each dimension is (gridX/8, gridY/8, gridZ/8) if 8x8x8 is the group size
    const workgroupCountX = Math.ceil(gridX / 8);
    const workgroupCountY = Math.ceil(gridY / 8);
    const workgroupCountZ = Math.ceil(gridZ / 8);
    pass.dispatchWorkgroups(workgroupCountX, workgroupCountY, workgroupCountZ);

    pass.end();
    device.queue.submit([commandEncoder.finish()]);
  }

  // ------------------ 3.5) Copy Buffer -> 3D Texture ------------------
  // The grid buffer contains vec4<f32> (RGBA float). For simplicity,
  // we can reinterpret those as RGBA8 if values are in [0,1].
  {
    // We’ll do a texture copy row by row (and slice by slice).
    // Each “row” in the buffer is gridX elements, each element is 16 bytes.
    const bytesPerRow = gridX * 4 /*floats*/ * 4; /*bytes per float*/
    const rowsPerImage = gridY;

    const commandEncoder = device.createCommandEncoder();
    commandEncoder.copyBufferToTexture(
      {
        buffer: gridBuffer,
        bytesPerRow,
        rowsPerImage,
      },
      {
        texture: gridTexture,
      },
      {
        width: gridX,
        height: gridY,
        depthOrArrayLayers: gridZ,
      },
    );
    device.queue.submit([commandEncoder.finish()]);
  }

  // ------------------ 3.6) Create Render Pipeline (Slicing Example) ------------------
  const renderModule = device.createShaderModule({ code: sliceRenderWGSL });
  const renderPipeline = device.createRenderPipeline({
    layout: "auto",
    vertex: {
      module: renderModule,
      entryPoint: "vs_main",
    },
    fragment: {
      module: renderModule,
      entryPoint: "fs_main",
      targets: [{ format }],
    },
    primitive: {
      topology: "triangle-list",
    },
  });

  // Create a bind group for rendering
  const resolutionBuffer = device.createBuffer({
    size: 8, // 2 floats = 8 bytes
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
  });
  device.queue.writeBuffer(
    resolutionBuffer,
    0,
    new Float32Array([context.canvas.width, context.canvas.height]),
  );

  // Create a sampler
  const sampler = device.createSampler({
    magFilter: "linear", // Linear filtering
    minFilter: "linear",
    addressModeU: "clamp-to-edge", // Clamp coordinates outside [0, 1]
    addressModeV: "clamp-to-edge",
    addressModeW: "clamp-to-edge",
  });

  const sliceUniformBuffer = device.createBuffer({
    size: 8, // sliceAxis (u32) + slicePosition (f32) = 8 bytes
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
  });

  document.getElementById("axis")?.addEventListener("change", (event) => {
    const axis = parseInt(event.target.value, 10);
    device.queue.writeBuffer(sliceUniformBuffer, 0, new Uint32Array([axis]));
  });

  document.getElementById("position")?.addEventListener("input", (event) => {
    const position = parseFloat(event.target.value);
    device.queue.writeBuffer(
      sliceUniformBuffer,
      4,
      new Float32Array([position]),
    );
  });

  // Update sliceUniformBuffer to control the axis and position
  device.queue.writeBuffer(sliceUniformBuffer, 0, new Uint32Array([2])); // Slice along Z-axis
  device.queue.writeBuffer(sliceUniformBuffer, 4, new Float32Array([0.5])); // Middle slice (Z=0.5)

  // Update the bind group to include the slice uniform
  const renderBindGroup = device.createBindGroup({
    layout: renderPipeline.getBindGroupLayout(0),
    entries: [
      { binding: 0, resource: { buffer: resolutionBuffer } },
      { binding: 1, resource: gridTexture.createView() },
      { binding: 2, resource: sampler },
      { binding: 3, resource: { buffer: sliceUniformBuffer } }, // Slice axis
    ],
  });

  // ------------------ 3.7) Render Pass ------------------
  function frame() {
    // Encode draw commands
    const commandEncoder = device.createCommandEncoder();

    const textureView = context.getCurrentTexture().createView();
    const renderPass = commandEncoder.beginRenderPass({
      colorAttachments: [
        {
          view: textureView,
          clearValue: { r: 0, g: 0, b: 0, a: 1 },
          loadOp: "clear",
          storeOp: "store",
        },
      ],
    });

    renderPass.setPipeline(renderPipeline);
    renderPass.setBindGroup(0, renderBindGroup);
    // Drawing a full-screen triangle with vertex_index trick:
    renderPass.draw(3, 1, 0, 0);
    renderPass.end();

    device.queue.submit([commandEncoder.finish()]);
    requestAnimationFrame(frame);
  }
  requestAnimationFrame(frame);
}

// Kick off the entire process
runWebGPU().catch(console.error);
