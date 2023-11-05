#import bevy_pbr::meshlet_bindings::{meshlet_thread_meshlet_ids, meshlets, draw_command_buffer, draw_index_buffer, meshlet_thread_instance_ids, meshlet_instance_uniforms, meshlet_bounding_spheres, view}
#import bevy_render::maths::affine_to_square

@compute
@workgroup_size(128, 1, 1)
fn cull_meshlets(@builtin(global_invocation_id) thread_id: vec3<u32>) {
    if thread_id.x >= arrayLength(&meshlet_thread_meshlet_ids) { return; }
    let meshlet_id = meshlet_thread_meshlet_ids[thread_id.x];

    let instance_id = meshlet_thread_instance_ids[thread_id.x];
    let instance_uniform = meshlet_instance_uniforms[instance_id];
    let model = affine_to_square(instance_uniform.model);
    let model_scale = max(length(model[0]), max(length(model[1]), length(model[2])));

    var meshlet_visible = true;

    // TODO: Faster method from https://vkguide.dev/docs/gpudriven/compute_culling/#frustum-culling-function
    let bounding_sphere = meshlet_bounding_spheres[meshlet_id];
    let bounding_sphere_center = model * vec4(bounding_sphere.center, 1.0);
    let bounding_sphere_radius = model_scale * -bounding_sphere.radius;
    for (var i = 0u; i < 6u; i++) {
        meshlet_visible &= dot(view.frustum[i], bounding_sphere_center) > bounding_sphere_radius;
        if !meshlet_visible { break; }
    }

    if meshlet_visible {
        let meshlet = meshlets[meshlet_id];
        let meshlet_vertex_count = meshlet.triangle_count * 3u;
        let draw_index_buffer_start = atomicAdd(&draw_command_buffer.vertex_count, meshlet_vertex_count);

        let packed_thread_id = thread_id.x << 8u;
        for (var index_id = 0u; index_id < meshlet_vertex_count; index_id++) {
            draw_index_buffer[draw_index_buffer_start + index_id] = packed_thread_id | index_id;
        }
    }
}
