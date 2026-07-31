#import bevy_sprite::{
    mesh2d_functions as mesh_functions,
    mesh2d_vertex_output::VertexOutput,
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> effect: vec4<f32>;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var output: VertexOutput;
    var local_position = vertex.position;
    let angle = atan2(local_position.y, local_position.x);
    let idle_wave = sin(angle * 8.0 - effect.x * 2.5) * 0.012;
    let impact_wave = sin(angle * 12.0 - effect.x * 9.0) * effect.y * 0.09;
    local_position.xy *= 1.0 + idle_wave + impact_wave;

    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    output.world_position = mesh_functions::mesh2d_position_local_to_world(
        world_from_local,
        vec4<f32>(local_position, 1.0),
    );
    output.position = mesh_functions::mesh2d_position_world_to_clip(output.world_position);
    output.uv = vertex.uv;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let centered = input.uv * 2.0 - vec2<f32>(1.0);
    let radius = length(centered);
    let angle = atan2(centered.y, centered.x);
    let outer_ring =
        smoothstep(0.7, 0.82, radius) *
        (1.0 - smoothstep(0.91, 1.0, radius));
    let sectors = abs(fract(angle / 6.2831853 * 12.0 + effect.x * 0.05) - 0.5);
    let spokes = (1.0 - smoothstep(0.44, 0.5, sectors)) * smoothstep(0.28, 0.8, radius);
    let impact_ring =
        1.0 - smoothstep(0.025, 0.09, abs(radius - (0.25 + (1.0 - effect.y) * 0.58)));

    let intensity =
        outer_ring * (0.55 + effect.y * 0.7) +
        spokes * 0.08 +
        impact_ring * effect.y;
    let color = mix(
        vec3<f32>(0.05, 0.45, 1.0),
        vec3<f32>(0.35, 1.0, 1.0),
        effect.y,
    );
    return vec4<f32>(color * (0.8 + intensity), intensity * 0.72);
}
