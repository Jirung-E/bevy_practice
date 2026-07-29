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
var<uniform> base_color: vec4<f32>;

// x: vertex on, y: fragment on, z: elapsed time, w: shear amount
@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var<uniform> options: vec4<f32>;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var output: VertexOutput;
    var local_position = vertex.position;

    let normalized_y = local_position.y / 130.0;
    let shear = normalized_y * options.w;
    let wobble = sin(options.z * 3.0 + local_position.y * 0.02) * 12.0;
    local_position.x += (shear + wobble) * options.x;

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
    let swapped_uv_color = vec4<f32>(
        input.uv.y,
        input.uv.x,
        1.0 - input.uv.x,
        1.0,
    );
    return mix(base_color, swapped_uv_color, options.y);
}

