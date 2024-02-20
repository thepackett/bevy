use bevy_asset::{meta::Settings, saver::AssetSaver, VisitAssetDependencies};
use bevy_render::texture::CompressedImageFormats;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{extensions::GltfExtension, GltfLoader, RawGltf};



/// Saves the bevy GLTF asset to an asset source.
pub struct GlftSaver {
    /// List of compressed image formats handled by the saver.
    pub supported_compressed_formats: CompressedImageFormats,
    pub extensions: Vec<dyn GltfExtension>, //Maybe hashmap<String, dyn GltfExtension> instead
}

#[derive(Serialize, Deserialize)]
pub enum GltfFileType {
    JSON,
    Binary,
}

#[derive(Serialize, Deserialize)]
pub struct GltfSaverSettings {
    // Manually specified Extensions to use when saving. Some extensions, such as compressors, cannot be automatically detected and must be indicated here.
    pub manual_extensions: Vec<String>, // Individual extension may have their own settings.

    // Manually specified extras to save.
    pub manual_extras: Vec<String>,

    // Return an error on saving if an extension is required.
    pub no_required_extensions: bool,

    // Should the saver save everything into a single file using base64 encoding for binary and image data, or save into multiple files.
    pub single_file: bool,

    pub save_format: GltfFileType,
}

impl Default for GltfSaverSettings {
    fn default() -> Self {
        Self {
            manual_extensions: Vec::new(),
            manual_extras: Vec::new(),
            no_required_extensions: false,
            single_file: true,
            save_format: GltfFileType::JSON,
        }
    }
}

#[derive(Error, Debug)]
pub enum GltfSaverError {

}

impl AssetSaver for GlftSaver {
    type Asset = RawGltf;
    type Settings = GltfSaverSettings;
    type OutputLoader = GltfLoader;
    type Error = GltfSaverError;

    fn save<'a>(
        &'a self,
        writer: &'a mut bevy_asset::io::Writer,
        asset: bevy_asset::saver::SavedAsset<'a, Self::Asset>, //Store more complex saver objects inside the RawGltf as a resource mapping strings to savers?
        settings: &'a Self::Settings,
    ) -> bevy_utils::BoxedFuture<'a, Result<<Self::OutputLoader as bevy_asset::AssetLoader>::Settings, Self::Error>> {
        asset.visit_dependencies(&mut |a: bevy_asset::UntypedAssetId| {
            
        });
        
        // Get the extensions + extras needed to save the object
        // Use them to construct JSON + Bin + Images
        // Save
        todo!()
    }
}