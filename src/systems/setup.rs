use crate::resources::CharsetAsset;
use bevy::prelude::*;

pub fn setup_charset_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> =
        asset_server.load("tiles/charset_8x8_transparent.png".to_string());
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(8), 128 / 8, 128 / 8, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
    commands.insert_resource(CharsetAsset {
        atlas: texture_atlas_layout_handle.clone(),
        texture: texture_handle.clone(),
    });
}
