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

@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var color_texture: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var color_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(3)
var<uniform> uv_rect: vec4<f32>;

// 20C hot reload 실습에서는 앱을 실행한 채 이 두 상수를 수정한다.
const WOBBLE_SCALE: f32 = 50.0;
const HIT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.25, 0.18);

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var output: VertexOutput;
    var local_position = vertex.position;

    let phase = effect.x * 5.0 + local_position.y * 0.035;
    local_position.x += sin(phase) * effect.y * WOBBLE_SCALE;

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
    let atlas_uv = uv_rect.xy + input.uv * uv_rect.zw;
    let texture_color = textureSample(color_texture, color_sampler, atlas_uv);

    if texture_color.a < 0.05 {
        discard;
    }

    let hit_color = vec4<f32>(HIT_COLOR, texture_color.a);
    return mix(texture_color, hit_color, effect.z);
}
