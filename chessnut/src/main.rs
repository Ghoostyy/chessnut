use bevy::prelude::*;
mod board;
mod pieces;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(game::GameState::default())
        .add_systems(Startup, board::setup_board)
        .add_systems(Update, game::select_piece)
        .add_systems(Update, game::move_piece)
        .run();
}