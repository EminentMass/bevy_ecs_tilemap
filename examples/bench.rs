use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSettings,
};
use bevy_ecs_tilemap::prelude::*;

mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let tilemap_size = TilemapSize { x: 1280, y: 1280 };
    let mut tile_storage = TileStorage::empty(tilemap_size);
    let tilemap_entity = commands.spawn().id();

    fill_tilemap(
        TileTexture(0),
        tilemap_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            map_type,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Benchmark Example"),
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(helpers::camera::movement)
        .run();
}
