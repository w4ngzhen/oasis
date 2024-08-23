use crate::components::field_of_vision::FieldOfVision;
use crate::components::name::Name;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::resources::map::Map;
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

pub fn setup_world_map(world: &mut World) {
    world.insert_resource(Map::new_map());
}

pub fn setup_player(mut commands: Commands, map: Res<Map>) {
    let first_room = map.rooms[0];
    let (player_x, player_y) = first_room.center().to_tuple();
    commands.spawn((
        Player,
        Name("Tom".to_string()),
        FieldOfVision { visible_tiles: vec![], range: 8, invalid: true },
        Position { x: player_x, y: player_y },
    ));
}
