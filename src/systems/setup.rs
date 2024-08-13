use bevy::prelude::*;

pub fn setup_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    let size: u32 = 12;
    let texture: Handle<Image> = asset_server.load(format!("tiles/basic_{}x{}.png", size, size));
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(size), 192 / size, 600 / size, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
}

pub fn setup() {
    println!("hello, world.")
}
