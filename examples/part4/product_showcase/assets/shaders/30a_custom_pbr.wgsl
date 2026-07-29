#import bevy_pbr::{
    mesh_bindings::mesh,
    mesh_functions,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
    view_transformations::position_world_to_clip,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> effect: vec4<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var<uniform> tint: vec4<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var mask_texture: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(103)
var mask_sampler: sampler;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var local_position = vertex.position;

    let wave = sin(vertex.position.y * 7.0 + effect.x * 3.0);
    local_position += vertex.normal * wave * effect.y;

    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);

#ifdef VERTEX_NORMALS
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        vertex.instance_index,
    );
#endif

#ifdef VERTEX_POSITIONS
    out.world_position = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(local_position, 1.0),
    );
    out.position = position_world_to_clip(out.world_position.xyz);
#endif

#ifdef VERTEX_UVS_A
    out.uv = vertex.uv;
#endif

#ifdef VERTEX_UVS_B
    out.uv_b = vertex.uv_b;
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        vertex.instance_index,
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    out.instance_index = vertex.instance_index;
#endif

#ifdef VISIBILITY_RANGE_DITHER
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex.instance_index,
        world_from_local[3],
    );
#endif

    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color,
    );

    let mask_rgb = textureSample(mask_texture, mask_sampler, in.uv).rgb;
    let mask = max(mask_rgb.r, max(mask_rgb.g, mask_rgb.b));
    let pulse = 0.5 + 0.5 * sin(effect.x * 4.0 + in.uv.y * 18.0);

    let tinted_base_color = pbr_input.material.base_color.rgb * mix(
        vec3<f32>(1.0),
        tint.rgb,
        mask * effect.w,
    );
    pbr_input.material.base_color = vec4<f32>(
        tinted_base_color,
        pbr_input.material.base_color.a,
    );
    pbr_input.material.emissive = vec4<f32>(
        pbr_input.material.emissive.rgb + tint.rgb * mask * pulse * effect.z,
        pbr_input.material.emissive.a,
    );

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
