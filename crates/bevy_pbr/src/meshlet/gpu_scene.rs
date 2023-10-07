use super::{
    persistent_buffer::PersistentGpuBuffer,
    psb_wrappers::{
        ByteArrayPsb, MeshletMeshBoundingConesPsb, MeshletMeshBoundingSpheresPsb,
        MeshletMeshMeshletsPsb, MeshletMeshVerticesPsb,
    },
    MeshletMesh,
};
use bevy_asset::{AssetId, Assets, Handle};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query, Res, ResMut, Resource},
    world::{FromWorld, World},
};
use bevy_render::{
    render_resource::{BindGroup, BindGroupLayout, BindGroupLayoutDescriptor},
    renderer::{RenderDevice, RenderQueue},
    Extract,
};
use bevy_utils::HashMap;
use std::{ops::Range, sync::Arc};

pub fn extract_meshlet_meshes(
    mut commands: Commands,
    query: Extract<Query<(Entity, &Handle<MeshletMesh>)>>,
    assets: Extract<Res<Assets<MeshletMesh>>>,
    mut gpu_scene: ResMut<MeshletGpuScene>,
) {
    for (entity, handle) in &query {
        let scene_slice = gpu_scene.queue_meshlet_mesh_upload(handle, &assets);
        commands.entity(entity).insert(scene_slice);

        // TODO: Unload MeshletMesh asset
    }
}

pub fn perform_pending_meshlet_mesh_writes(
    mut gpu_scene: ResMut<MeshletGpuScene>,
    render_queue: Res<RenderQueue>,
    render_device: Res<RenderDevice>,
) {
    gpu_scene
        .vertex_data
        .perform_writes(&render_queue, &render_device);
    gpu_scene
        .meshlet_vertices
        .perform_writes(&render_queue, &render_device);
    gpu_scene
        .meshlet_indices
        .perform_writes(&render_queue, &render_device);
    gpu_scene
        .meshlets
        .perform_writes(&render_queue, &render_device);
    gpu_scene
        .meshlet_bounding_spheres
        .perform_writes(&render_queue, &render_device);
    gpu_scene
        .meshlet_bounding_cones
        .perform_writes(&render_queue, &render_device);
}

#[derive(Resource)]
pub struct MeshletGpuScene {
    vertex_data: PersistentGpuBuffer<ByteArrayPsb>,
    meshlet_vertices: PersistentGpuBuffer<MeshletMeshVerticesPsb>,
    meshlet_indices: PersistentGpuBuffer<ByteArrayPsb>,
    meshlets: PersistentGpuBuffer<MeshletMeshMeshletsPsb>,
    meshlet_bounding_spheres: PersistentGpuBuffer<MeshletMeshBoundingSpheresPsb>,
    meshlet_bounding_cones: PersistentGpuBuffer<MeshletMeshBoundingConesPsb>,
    meshlet_mesh_slices: HashMap<AssetId<MeshletMesh>, MeshletMeshGpuSceneSlice>,

    bind_group_layout: BindGroupLayout,
}

impl FromWorld for MeshletGpuScene {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        Self {
            vertex_data: PersistentGpuBuffer::new("meshlet_gpu_scene_vertex_data", render_device),
            meshlet_vertices: PersistentGpuBuffer::new(
                "meshlet_gpu_scene_meshlet_vertices",
                render_device,
            ),
            meshlet_indices: PersistentGpuBuffer::new(
                "meshlet_gpu_scene_meshlet_indices",
                render_device,
            ),
            meshlets: PersistentGpuBuffer::new("meshlet_gpu_scene_meshlets", render_device),
            meshlet_bounding_spheres: PersistentGpuBuffer::new(
                "meshlet_gpu_scene_meshlet_bounding_spheres",
                render_device,
            ),
            meshlet_bounding_cones: PersistentGpuBuffer::new(
                "meshlet_gpu_scene_meshlet_bounding_cones",
                render_device,
            ),
            meshlet_mesh_slices: HashMap::new(),

            bind_group_layout: render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("meshlet_gpu_scene_bind_group_layout"),
                entries: &[], // TODO
            }),
        }
    }
}

impl MeshletGpuScene {
    fn queue_meshlet_mesh_upload(
        &mut self,
        handle: &Handle<MeshletMesh>,
        assets: &Assets<MeshletMesh>,
    ) -> MeshletMeshGpuSceneSlice {
        let queue_meshlet_mesh = |asset_id: &AssetId<MeshletMesh>| {
            let meshlet_mesh = assets.get(*asset_id).expect("TODO");

            self.vertex_data
                .queue_write(ByteArrayPsb(Arc::clone(&meshlet_mesh.vertex_data)));
            self.meshlet_vertices
                .queue_write(MeshletMeshVerticesPsb(Arc::clone(
                    &meshlet_mesh.meshlet_vertices,
                )));
            self.meshlet_indices
                .queue_write(ByteArrayPsb(Arc::clone(&meshlet_mesh.meshlet_indices)));
            let slice = self
                .meshlets
                .queue_write(MeshletMeshMeshletsPsb(Arc::clone(&meshlet_mesh.meshlets)));
            self.meshlet_bounding_spheres
                .queue_write(MeshletMeshBoundingSpheresPsb(Arc::clone(
                    &meshlet_mesh.meshlet_bounding_spheres,
                )));
            self.meshlet_bounding_cones
                .queue_write(MeshletMeshBoundingConesPsb(Arc::clone(
                    &meshlet_mesh.meshlet_bounding_cones,
                )));

            MeshletMeshGpuSceneSlice((slice.start / 16)..(slice.end / 16))
        };

        self.meshlet_mesh_slices
            .entry(handle.id())
            .or_insert_with_key(queue_meshlet_mesh)
            .clone()
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn create_per_frame_bind_group(&self) -> BindGroup {
        todo!()
    }
}

#[derive(Component, Clone)]
pub struct MeshletMeshGpuSceneSlice(Range<u64>);
