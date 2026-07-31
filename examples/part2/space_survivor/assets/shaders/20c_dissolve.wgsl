#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> effect: vec4<f32>;

fn hash21(value: vec2<f32>) -> f32 {
    return fract(sin(dot(value, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn value_noise(uv: vec2<f32>) -> f32 {
    let cell = floor(uv);
    let local = fract(uv);
    let smooth_local = local * local * (vec2<f32>(3.0) - 2.0 * local);

    let bottom = mix(hash21(cell), hash21(cell + vec2<f32>(1.0, 0.0)), smooth_local.x);
    let top = mix(
        hash21(cell + vec2<f32>(0.0, 1.0)),
        hash21(cell + vec2<f32>(1.0, 1.0)),
        smooth_local.x,
    );
    return mix(bottom, top, smooth_local.y);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let coarse = value_noise(input.uv * 8.0);
    let detail = value_noise(input.uv * 23.0 + vec2<f32>(effect.x * 0.18, 0.0));
    let noise = coarse * 0.72 + detail * 0.28;
    let remaining = noise - effect.y;

    if remaining < 0.0 {
        discard;
    }

    let edge = 1.0 - smoothstep(0.0, effect.z, remaining);
    let body = vec3<f32>(0.95, 0.04, 0.14);
    let glowing_edge = vec3<f32>(1.0, 0.78, 0.08) * 2.2;
    let color = mix(body, glowing_edge, edge);
    return vec4<f32>(color, 1.0);
}
