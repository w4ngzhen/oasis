use crate::resources::CharsetAsset;
use bevy::prelude::*;

pub fn setup_charset_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let size = 12_u32;
    let texture_handle: Handle<Image> =
        asset_server.load(format!("tiles/basic_{}x{}.png", size, size));
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(size), 192 / size, 600 / size, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
    commands.insert_resource(CharsetAsset {
        atlas: texture_atlas_layout_handle.clone(),
        texture: texture_handle.clone(),
    });
}
