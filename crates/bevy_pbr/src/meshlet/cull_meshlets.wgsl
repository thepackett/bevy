#import bevy_pbr::meshlet_bindings

@compute(8, 8, 1)
fn cull_meshlets(@builtin(global_invocation_id) global_id: vec3<u32>) {

}
