use crate::components::field_of_vision::FieldOfVision;
use crate::components::name::Name;
use crate::components::position::Position;
use crate::components::role::Player;
use crate::resources::map::Map;
use crate::resources::CharsetAsset;
use bevy::prelude::*;

pub fn setup_charset_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let size: u32 = 12;
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

pub fn setup_world_map(world: &mut World) {
    world.insert_resource(Map::new_map());
}

pub fn setup_player(mut commands: Commands, map: Res<Map>) {
    let first_room = map.rooms[0];
    let (player_x, player_y, _) = first_room.center().to_tuple();
    commands.spawn((
        Player,
        Name("Tom".to_string()),
        FieldOfVision { visible_tiles: vec![], range: 8, invalid: true },
        Position { x: player_x, y: player_y, z: 1 },
    ));
}
