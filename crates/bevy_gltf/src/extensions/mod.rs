pub mod core;

use std::marker::PhantomData;

use bevy_asset::{Asset, Handle};
use bevy_ecs::{component::Component, entity::Entity, world::World};
pub use bevy_gltf_macros::GltfComponent;
use serde::{Deserialize, Serialize};


pub trait GltfExtension
{
    /// Return the name of the glTF extension. 
    /// This name should follow glTF 2.0 naming conventions: https://github.com/KhronosGroup/glTF/blob/main/extensions/README.md#naming
    fn name(&self) -> &str;

    /// An array of extension that this extension requires.
    fn dependencies(&self) -> &[&str];

    /// An array of extensions that this extension is incompatible with.
    fn exclusions(&self) -> &[&str];

    /// Indicates whether this extension is a required extension.
    /// An extension is required if a glTF file will not be able to be rendered without it.
    fn is_required(&self) -> bool;

    fn max_version(&self);
    fn min_version(&self);

    fn load(&self);

    // After all data is loaded, extensions have the option to prepare the ECS world.
    // This is useful for extenstions that want to replace data from the core specification with other data.
    // E.g. a mesh compression extension may replace a mesh's vertex data with data that was decompressed.
    // This is so that asset extractors don't need to account for extensions that replace data they need.
    fn prepare(&self, world: &mut World);

    // If an extension validates itself, that extension must be used.
    fn validate(&self);
    // If an extension validates its parent, then that extension must be required.
    fn validate_parent(&self);

    // Save the data of this extension to json / binary.
    // If it's allowed to have required extensions, trim replaced data in parent.
    // Validate before saving.
    fn save(&self);
}

pub trait GltfAsset {
    type AssetType: Asset;

    fn required_extensions(&self) -> Vec<&str>;

    fn extract(&self, gltf_world: World) -> Handle<Self::AssetType>;

    fn insert(&self, gltf_world: World, asset: Self::AssetType);

    fn is_present(&self, gltf_world: World, asset: &Self::AssetType);
}

pub trait GltfComponent{}

#[derive(Component, GltfComponent)]
pub struct Is<T>{
    pub index: u32,
    _marker: PhantomData<fn()->T>,
}

// A vector of entity ids that contain Is<T>. Represents a reference. For instance, a node with Has<Mesh> should point to some Is<Mesh> entity.
// This in turn represents a gltf index into a root level array. In this instance, it represents an index into the Mesh array.
#[derive(Component, GltfComponent)]
pub struct Has<T> {
    // If we had relations then this could be better represended as a relation that points to a single entity. Then, you could simply add more of these to different entities.
    pub has: Vec<Entity>,
    _marker: PhantomData<fn()->T>,
}

// This component is added when a component on an entity has been Validated as Gltf compliant. An entity is Gltf compliant when all of its components are validated.
// The extension responsible for the components is responsible for validating them.
// If an extension is required, then it may also validate its parent's components.
#[derive(Component, GltfComponent)]
pub struct Validated<T> 
where
    T: Component + GltfComponent
{
    _marker: PhantomData<fn()->T>,
}