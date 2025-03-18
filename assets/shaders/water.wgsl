#import bevy_pbr::{
    mesh_view_bindings::globals,
    mesh_bindings::mesh,
    mesh_functions,
    forward_io::{
        Vertex,
        VertexOutput,
    },
    view_transformations::position_world_to_clip,

}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var noise_texture_1: texture_2d<f32>;
@group(2) @binding(2) var noise_sampler_1: sampler;
@group(2) @binding(3) var noise_texture_2: texture_2d<f32>;
@group(2) @binding(4) var noise_sampler_2: sampler;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Use world position's x and z axes for UVs
    let world_position = mesh.world_position + globals.time;

    // Map x and z from world coordinates to UV range (0.0, 1.0)
    let uv_world_space = vec2<f32>(
        fract(world_position.x * 0.1), // Example scaling factor for tiling effect
        fract(world_position.z * 0.1)
    );

    // Sample the texture using world space UVs
    let texture_color = textureSample(noise_texture_1, noise_sampler_1, uv_world_space);

    // Return final color
    return material_color * texture_color;
}
