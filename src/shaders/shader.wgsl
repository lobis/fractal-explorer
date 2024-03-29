
struct Uniform {
   domain: vec4<f32>,
   mouse: vec2<f32>,
   c: vec2<f32>,
   time: f32,
   mandelbrot: i32,
   padding: vec2<f32>,
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

fn hsv2rgb(hue: f32, saturation: f32, value: f32) -> vec3<f32> {
    // hue, saturation and value must be between 0.0 and 1.0
    // formula adapted from https://www.tlbx.app/color-converter
    let h = hue * 6.0;
    let chroma = saturation * value;
    let x = chroma * (1.0 - abs(h % 2.0 - 1.0));
    let m = value - chroma;
    if (h <= 1.0) {
        return vec3<f32>(chroma + m, x + m, m);
    } else if (h <= 2.0) {
        return vec3<f32>(x + m, chroma + m, m);
    } else if (h <= 3.0) {
        return vec3<f32>(m, chroma + m, x + m);
    } else if (h <= 4.0) {
        return vec3<f32>(m, x + m, chroma + m);
    } else if (h <= 5.0) {
        return vec3<f32>(x + m, m, chroma + m);
    } else {
        return vec3<f32>(chroma + m, m, x + m);
    }
}

fn get_color(fraction_: f32, time: f32) -> vec3<f32> {
    if (fraction_ >= 1.0) {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let pi: f32 = 3.14159265358979323846264338327950288419716939937510582097494459;
    let fraction = sin(fraction_ * pi * 5.0);

    let period: f32 = 30.0; // seconds
    let oscillating = (time / period ) % 1.0;

    let color_end: vec3<f32> = hsv2rgb(oscillating, 1.0, 1.0);
    let color_begin: vec3<f32> = hsv2rgb((oscillating + 0.5) % 1.0, 1.0, 1.0);

    return color_begin * fraction + color_end * (1.0 - fraction);
}

fn julia(z_start: vec2<f32>, c: vec2<f32>) -> f32 {

    let iterations_max: i32 = 255;
    var z: vec2<f32> = z_start;
    var i: i32 = 0;
    for (; i < iterations_max; i = i + 1) {
        if (dot(z, z) > 4.0) { break; }
        z = vec2<f32>((z.x * z.x) - (z.y * z.y) + c.x, (2.0 * z.x * z.y) + c.y);
    }

    return f32(i) / f32(iterations_max);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // z -> z * z + c | let z = (a + ib) and c = (c + id) then z * z + c = (a*a - b*b + c) + i(2*a*b + d)

    let domain_size: vec2<f32> = vec2<f32>(my_uniform.domain[1] - my_uniform.domain[0], my_uniform.domain[3] - my_uniform.domain[2]);
    let domain_center: vec2<f32> = vec2<f32>(my_uniform.domain[1] + my_uniform.domain[0], my_uniform.domain[3] + my_uniform.domain[2]) / 2.0;

    var z: vec2<f32> = vec2<f32>(in.position_xy.x * domain_size.x, in.position_xy.y * domain_size.y) / 2.0 + domain_center;

    var fraction: f32;
    if (my_uniform.mandelbrot <= 0) {
        // use julia set
        fraction = julia(z, my_uniform.c);
    } else {
        fraction = julia(my_uniform.c, z);
    }

    let color = get_color(fraction, my_uniform.time);

    return vec4<f32>(color, 1.0);
}
