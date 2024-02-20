

#[derive(Component, GltfComponent)]
pub struct GltfRoot;

#[derive(Component, GltfComponent)]
pub struct ExtensionsUsed{
    pub extensions: Vec<String>,
}

#[derive(Component, GltfComponent)]
pub struct ExtensionsRequired{
    pub extensions: Vec<String>,
}

#[derive(Component, GltfComponent)]
pub struct Copyright{
    pub copyright: String,
}

#[derive(Component, GltfComponent)]
pub struct Generator{
    pub generator: String,
}

#[derive(Component, GltfComponent)]
pub struct Version{
    pub version: String,
}

#[derive(Component, GltfComponent)]
pub struct MinVersion{
    pub min_version: String,
}

#[derive(Component, GltfComponent)]
pub struct DefaultScene{
    pub default_scene: u32,
}

#[derive(Component, GltfComponent)]
pub struct GltfNode;


#[derive(Component, GltfComponent)]
pub struct Rotation{
    pub rotation: [f32; 4],
}

#[derive(Component, GltfComponent)]
pub struct Scale{
    pub scale: [f32; 4],
}