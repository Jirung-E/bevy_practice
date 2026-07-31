#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> options: vec4<f32>;

fn hash21(value: vec2<f32>) -> f32 {
    let mixed = dot(value, vec2<f32>(127.1, 311.7));
    return fract(sin(mixed) * 43758.5453);
}

fn star_layer(
    uv: vec2<f32>,
    grid_size: f32,
    drift: f32,
    density: f32,
) -> f32 {
    let aspect_uv = vec2<f32>(uv.x * 1.5, uv.y);
    let position =
        aspect_uv * grid_size + vec2<f32>(0.0, options.x * options.y * drift);
    let cell_id = floor(position);
    let local = fract(position) - vec2<f32>(0.5);
    let seed = hash21(cell_id);
    let exists = step(1.0 - density, seed);
    let radius = mix(0.055, 0.14, hash21(cell_id + vec2<f32>(4.7, 9.2)));
    let core = 1.0 - smoothstep(radius * 0.2, radius, length(local));
    let twinkle = 0.72 + 0.28 * sin(options.x * (2.0 + seed * 4.0) + seed * 20.0);
    return exists * core * twinkle;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let far_stars = star_layer(input.uv, 24.0, 0.45, options.z * 1.4);
    let near_stars = star_layer(
        input.uv + vec2<f32>(options.w, 0.17),
        13.0,
        1.35,
        options.z,
    );

    let vertical_glow = pow(max(0.0, 1.0 - abs(input.uv.x - 0.5) * 1.8), 3.0);
    let background =
        vec3<f32>(0.006, 0.012, 0.045) +
        vec3<f32>(0.02, 0.015, 0.07) * vertical_glow;
    let color =
        background +
        vec3<f32>(0.32, 0.52, 1.0) * far_stars +
        vec3<f32>(0.78, 0.92, 1.0) * near_stars;

    return vec4<f32>(color, 1.0);
}
