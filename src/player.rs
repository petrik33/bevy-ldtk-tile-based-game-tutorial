use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::walls::LevelWalls;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player_from_input)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let movement_direction = if input.just_pressed(KeyCode::KeyW) {
        GridCoords::new(0, 1)
    } else if input.just_pressed(KeyCode::KeyA) {
        GridCoords::new(-1, 0)
    } else if input.just_pressed(KeyCode::KeyS) {
        GridCoords::new(0, -1)
    } else if input.just_pressed(KeyCode::KeyD) {
        GridCoords::new(1, 0)
    } else {
        return;
    };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
        }
    }
}
