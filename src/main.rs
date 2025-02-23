use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use game::GamePlugin;
use player::PlayerPlugin;
use walls::WallsPlugin;

mod game;
mod player;
mod walls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WallsPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDtk/tile-based-game.ldtk").into(),
        ..Default::default()
    });
}
