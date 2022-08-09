
struct Uniform {
   mouse: vec2<f32>,
   time: f32,
}
@group(0) @binding(0) 
var<uniform> my_uniform: Uniform;

// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position_xy: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.position_xy = model.position.xy;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let c = (my_uniform.mouse - vec2<f32>(0.5, 0.5)) * 2.0;
    // let c: vec2<f32> = vec2<f32>(-0.75, 0.0);

    // z -> z * z + c | let z = (a + ib) and c = (c + id) then z * z + c = (a*a - b*b + c) + i(2*a*b + d)
    var z: vec2<f32> = (in.position_xy - vec2<f32>(0.0, 0.0)) * 2.0;

    let iterations_max: i32 = 100;
    var i: i32 = 0;
    for (; i < iterations_max; i = i + 1) {
        if (dot(z, z) > 4.0) {break;}
        z = vec2<f32>((z.x * z.x) - (z.y * z.y) + c.x, (2.0 * z.x * z.y) + c.y);
    }

    let fraction: f32 = f32(i) / f32(iterations_max);

    var color = vec3<f32>(0.0, 0.0, 0.0);
    if (i < iterations_max) {
        let pi: f32 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062;
        let freq: f32 = 1.0;
        let time_dependant_1 = (sin(my_uniform.time * freq * (1.0 / 3.0) + 0.0) + 1.0 ) / 2.0 ;
        let time_dependant_2 = (sin(my_uniform.time * freq * 0.5 + 1.0 * pi / 3.0) + 1.0 ) / 2.0 ;
        let time_dependant_3 = (sin(my_uniform.time * freq * 0.25 + 2.0 * pi / 3.0) + 1.0 ) / 2.0 ;

        let color_end: vec3<f32> = vec3<f32>(time_dependant_1, time_dependant_2, time_dependant_3);
        let color_begin: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0) - color_end;

        color = color_begin * fraction + color_end * (1.0 - fraction);
    }
    return vec4<f32>(color, 1.0);
}
