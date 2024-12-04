use bevy::prelude::*;
use crate::pieces::{Position, Movable};

#[derive(Resource)]
pub struct GameState {
    pub selected_piece: Option<Entity>,
    pub moves: Vec<(usize, usize, usize, usize)>, // (from_x, from_y, to_x, to_y)
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            selected_piece: None,
            moves: Vec::new(),
        }
    }
}

pub fn select_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    query: Query<(Entity, &Position), With<Movable>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (entity, position) in query.iter() {
            if position.x == 0 && position.y == 0 { // Example condition to select a piece
                game_state.selected_piece = Some(entity);
                break;
            }
        }
    }
}

pub fn move_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Position), With<Movable>>,
    mut game_state: ResMut<GameState>,
) {
    if let Some(selected_piece) = game_state.selected_piece {
        if let Ok((mut transform, mut position)) = query.get_mut(selected_piece) {
            let mut moved = false;
            let (old_x, old_y) = (position.x, position.y);

            if keyboard_input.just_pressed(KeyCode::Up) && position.y < 7 {
                position.y += 1;
                moved = true;
            }
            if keyboard_input.just_pressed(KeyCode::Down) && position.y > 0 {
                position.y -= 1;
                moved = true;
            }
            if keyboard_input.just_pressed(KeyCode::Left) && position.x > 0 {
                position.x -= 1;
                moved = true;
            }
            if keyboard_input.just_pressed(KeyCode::Right) && position.x < 7 {
                position.x += 1;
                moved = true;
            }

            if moved {
                game_state.moves.push((old_x, old_y, position.x, position.y));
                transform.translation = Vec3::new(position.x as f32 * 100.0, position.y as f32 * 100.0, 1.0);
            }
        }
    }
}

pub fn reset_board(mut commands: Commands, asset_server: Res<AssetServer>, mut game_state: ResMut<GameState>) {
    game_state.moves.clear();
    game_state.selected_piece = None;
    commands.insert_resource(GameState::default());
    crate::board::setup_board(commands, asset_server);
}