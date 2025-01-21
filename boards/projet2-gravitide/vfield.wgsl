
const PI: f32 = 3.141592653589793;
const TAU: f32 = 2.0 * PI;

@group(0) @binding(0) var<uniform> elapsed_time: f32;

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let uv_raw =  vec4<f32>(position.xy / surface_size, 0., 1.)-.5;
	let len = length(uv_raw.xy);
	
    let t = 0.05*elapsed_time;
	let time = t  +  (5.+sin(t))*.11 / (len+.07); // spiraling
	let si = sin(time);
    let co = cos(time);
	let uv = uv_raw.xy * mat2x2<f32>(co, si, -si, co);                    // rotation

    var v1=0.;
    var v2=0.;
	var p: vec3<f32>;

	for (var i: u32 = 0u; i < 100u; i = i + 1) {
		p = .035 * f32(i) * vec3(uv, 1.) + vec3(.22,  .3,  -1.5 -sin(t*1.3)*.1);
		p = abs(p) / dot(p,p) - 0.659;

		for (var j: u32 = 0u; j < 8u; j = j + 1) {               // IFS
			p = abs(p) / dot(p,p) - 0.659;
        }

		let p2 = dot(p,p)*.0015;
		v1 += p2 * ( 1.8 + sin(len*13.0  +.5 -t*2.) );
		v2 += p2 * ( 1.5 + sin(len*13.5 +2.2 -t*3.) );
	}
	
	let c = length(p.xy) * .175;
	v1 *= smoothstep(.7 , .0, len);
	v2 *= smoothstep(.6 , .0, len);
	let v3  = smoothstep(.15, .0, len);

	var col = vec3(c,  (v1+c)*.25,  v2);
	col = col  +  v3*.9;                      // useless: clamp(col, 0.,1.)


    // let color = vec4<f32>(px_pos.x, px_pos.y, sin(elapsed_time), 1.0);
    return vec4(col, 1.);
}