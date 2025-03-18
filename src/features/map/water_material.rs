use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy::color::palettes::css::SKY_BLUE;

pub(super) struct WaterMaterialPlugin;

impl Plugin for WaterMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<WaterMaterial>::default());
    }
}

const SHADER_ASSET_PATH: &str = "shaders/water.wgsl";
pub(super) const NOISE_TEXTURE_1_PATH: &str = "textures/gradient_noise.png";
pub(super) const NOISE_TEXTURE_2_PATH: &str = "textures/gradient_noise_2.png";

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(super) struct WaterMaterial {
    #[uniform(0)]
    pub color_1: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub noise_texture_1: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub noise_texture_2: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
