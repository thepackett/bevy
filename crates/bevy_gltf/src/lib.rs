//! Plugin providing an [`AssetLoader`](bevy_asset::AssetLoader) and type definitions
//! for loading glTF 2.0 (a standard 3D scene definition format) files in Bevy.
//!
//! The [glTF 2.0 specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html) defines the format of the glTF files.

#![warn(missing_docs)]

use std::any::TypeId;

#[cfg(feature = "bevy_animation")]
use bevy_animation::AnimationClip;
use bevy_utils::{hashbrown::Equivalent, HashMap};

mod loader;
mod saver;
mod vertex_attributes;
mod extensions;
pub use loader::*;
pub use saver::*;


use bevy_app::prelude::*;
use bevy_asset::{Asset, AssetApp, Handle, UntypedHandle};
use bevy_ecs::{prelude::Component, reflect::ReflectComponent, world::World};
use bevy_pbr::StandardMaterial;
use bevy_reflect::{Reflect, TypePath};
use bevy_render::{
    mesh::{Mesh, MeshVertexAttribute},
    renderer::RenderDevice,
    texture::CompressedImageFormats,
};
use bevy_scene::Scene;

/// Adds support for glTF file loading to the app.
#[derive(Default)]
pub struct GltfPlugin {
    //extensions: Vec<dyn GltfExtension>,
    custom_vertex_attributes: HashMap<String, MeshVertexAttribute>,
}

impl GltfPlugin {
    /// Register a custom vertex attribute so that it is recognized when loading a glTF file with the [`GltfLoader`].
    ///
    /// `name` must be the attribute name as found in the glTF data, which must start with an underscore.
    /// See [this section of the glTF specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#meshes-overview)
    /// for additional details on custom attributes.
    pub fn add_custom_vertex_attribute(
        mut self,
        name: &str,
        attribute: MeshVertexAttribute,
    ) -> Self {
        self.custom_vertex_attributes
            .insert(name.to_string(), attribute);
        self
    }
}

// impl Plugin for GltfPlugin {
//     fn build(&self, app: &mut App) {
//         app.register_type::<GltfExtras>()
//             .init_asset::<Gltf>()
//             .init_asset::<GltfNode>()
//             .init_asset::<GltfPrimitive>()
//             .init_asset::<GltfMesh>()
//             .preregister_asset_loader::<GltfLoader>(&["gltf", "glb"]);
//     }

//     fn finish(&self, app: &mut App) {
//         let supported_compressed_formats = match app.world.get_resource::<RenderDevice>() {
//             Some(render_device) => CompressedImageFormats::from_features(render_device.features()),

//             None => CompressedImageFormats::NONE,
//         };
//         app.register_asset_loader(GltfLoader {
//             supported_compressed_formats,
//             custom_vertex_attributes: self.custom_vertex_attributes.clone(),
//         });
//     }
// }

/// Representation of a loaded glTF file.
#[derive(Asset, Debug, TypePath)]
pub struct RawGltf {
    pub world: World,
}


impl RawGltf {
    pub fn new() -> RawGltf{
        RawGltf { world: World::new() }
    }
}

/// Representation of a loaded glTF file's assets.
#[derive(Asset, Debug, TypePath)]
pub struct Gltf {
    // Storage of all handles that were extracted. Best Accessible by typed api. E.g. gltf.get_assets::<Scene>() -> Vec<Handle<Scene>>.
    pub assets: Vec<UntypedHandle>,
    // Hash map of strings to vector of untyped handles since names are not guaranteed to be unique.
    pub named_assets: HashMap<String, Vec<UntypedHandle>>,
}

impl Gltf {
    pub fn get_assets<A: Asset>(&self) -> Vec<Handle<A>> {
        let assets = self.assets.iter().filter_map(|handle| {
            if handle.type_id() == TypeId::of::<A>() {
                Some(handle.clone().typed_unchecked::<A>())
            } else {
                None
            }
        }).collect::<Vec<_>>();
        assets
    }

    pub fn get_named_assets<A: Asset>(&self) -> Vec<(String, Handle<A>)> {
        let assets = self.named_assets.iter().filter_map(|(name, handles)| {
            let mut pairs = Vec::new();
            for handle in handles {
                if handle.type_id() == TypeId::of::<A>() {
                    pairs.push((name.clone(), handle.clone().typed_unchecked::<A>()));
                }
            }
            Some(pairs)
        }).flatten().collect::<Vec<_>>();
        assets
    }

    pub fn get_assets_by_name<A: Asset>(&self, name: &str) -> Vec<Handle<A>> {
        let assets = match self.named_assets.get(name) {
            Some(handles) => {
                handles.iter().filter_map(|handle| {
                    if handle.type_id() == TypeId::of::<A>() {
                        Some(handle.clone().typed_unchecked::<A>())
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            },
            None => Vec::new(),
        };
        assets
    }

    pub fn get_scenes(&self) -> Vec<Handle<Scene>> {
        self.get_assets::<Scene>()
    }

    pub fn get_named_scenes(&self, name: &str) -> Vec<(String, Handle<Scene>)> {
        self.get_named_assets::<Scene>()
    }

    pub fn get_scenes_by_name(&self, name: &str) -> Vec<Handle<Scene>> {
        self.get_assets_by_name::<Scene>(name)
    }

    pub fn get_meshes(&self) -> Vec<Handle<Mesh>> {
        self.get_assets::<Mesh>()
    }

    pub fn get_named_meshes(&self, name: &str) -> Vec<(String, Handle<Mesh>)> {
        self.get_named_assets::<Mesh>()
    }

    pub fn get_meshes_by_name(&self, name: &str) -> Vec<Handle<Mesh>> {
        self.get_assets_by_name::<Mesh>(name)
    }

    pub fn get_materials(&self) -> Vec<Handle<StandardMaterial>> {
        self.get_assets::<StandardMaterial>()
    }

    pub fn get_named_materials(&self, name: &str) -> Vec<(String, Handle<StandardMaterial>)> {
        self.get_named_assets::<StandardMaterial>()
    }

    pub fn get_materials_by_name(&self, name: &str) -> Vec<Handle<StandardMaterial>> {
        self.get_assets_by_name::<StandardMaterial>(name)
    }

    // Feature gate animations?
    pub fn get_animations(&self) -> Vec<Handle<AnimationClip>> {
        self.get_assets::<AnimationClip>()
    }

    pub fn get_named_animations(&self, name: &str) -> Vec<(String, Handle<AnimationClip>)> {
        self.get_named_assets::<AnimationClip>()
    }

    pub fn get_animations_by_name(&self, name: &str) -> Vec<Handle<AnimationClip>> {
        self.get_assets_by_name::<AnimationClip>(name)
    }

    // Gltf nodes?

    // Default scene?
}