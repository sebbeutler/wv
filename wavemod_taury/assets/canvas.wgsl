
// @group(0) @binding(0) var<uniform> elapsed_time: f32;

@vertex
fn vs_canvas_bounds(
    @location(0) pos: vec3<f32>
) -> @builtin(position) vec4<f32> {
    // return vec4<f32>(pos + vec3<f32>(sin(elapsed_time), cos(elapsed_time), 0.0), 1.0);
    return vec4<f32>(pos, 1.0);
}

/* REMOVE SURFACE_SIZE UNIFORM

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) v_uv: vec2<f32>,
};

@vertex
fn vs_main(@location(0) in_position: vec2<f32>) -> VertexOutput
{
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in_position, 0.0, 1.0);
    // in_position is already [-1,+1], so just pass along:
    out.v_uv = in_position;
    return out;
}

@fragment
fn fs_main(@location(0) v_uv: vec2<f32>) -> @location(0) vec4<f32>
{
    // v_uv is in [-1,1], no need for surface_size
    let x = v_uv.x;
    let y = v_uv.y;
    // ... do your fractal or other math ...
    return vec4<f32>(some_color, 1.0);
}
 */