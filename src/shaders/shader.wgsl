
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
    let r: f32 = dot(in.position_xy, in.position_xy);
    let vector_mouse_center = my_uniform.mouse - vec2<f32>(0.5, 0.5);
    let r_check = max(dot(vector_mouse_center, vector_mouse_center), 0.01);
    if (r > r_check) {
        discard;
    }

    let normalized = (in.position_xy + vec2<f32>(1., 1.)) / 2.0;
    let time_dependant = (sin(my_uniform.time * 5.0) + 1.0 ) / 2.0 ;
    return vec4<f32>(normalized.rg, time_dependant, 1.0);
}
