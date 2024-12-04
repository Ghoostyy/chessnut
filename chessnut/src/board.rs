use bevy::prelude::*;
use crate::pieces::{PieceType, PieceColor, Position, Movable};

const BOARD_SIZE: usize = 8;
const DARK_COLOR: Color = Color::rgb(0.6, 0.4, 0.2);
const LIGHT_COLOR: Color = Color::rgb(1.0, 0.8, 0.6);

pub fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup the camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            (BOARD_SIZE as f32 * 50.0) - 50.0,
            (BOARD_SIZE as f32 * 50.0) - 50.0,
            1000.0,
        ),
        ..Default::default()
    });

    // Setup the board
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let color = if (x + y) % 2 == 0 { LIGHT_COLOR } else { DARK_COLOR };
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x as f32 * 100.0, y as f32 * 100.0, 0.0),
                ..Default::default()
            });
        }
    }

    // Setup pieces
    let piece_positions = vec![
        (PieceType::Rook, PieceColor::White, 0, 0),
        (PieceType::Knight, PieceColor::White, 1, 0),
        (PieceType::Bishop, PieceColor::White, 2, 0),
        (PieceType::Queen, PieceColor::White, 3, 0),
        (PieceType::King, PieceColor::White, 4, 0),
        (PieceType::Bishop, PieceColor::White, 5, 0),
        (PieceType::Knight, PieceColor::White, 6, 0),
        (PieceType::Rook, PieceColor::White, 7, 0),
        (PieceType::Pawn, PieceColor::White, 0, 1),
        (PieceType::Pawn, PieceColor::White, 1, 1),
        (PieceType::Pawn, PieceColor::White, 2, 1),
        (PieceType::Pawn, PieceColor::White, 3, 1),
        (PieceType::Pawn, PieceColor::White, 4, 1),
        (PieceType::Pawn, PieceColor::White, 5, 1),
        (PieceType::Pawn, PieceColor::White, 6, 1),
        (PieceType::Pawn, PieceColor::White, 7, 1),
        (PieceType::Rook, PieceColor::Black, 0, 7),
        (PieceType::Knight, PieceColor::Black, 1, 7),
        (PieceType::Bishop, PieceColor::Black, 2, 7),
        (PieceType::Queen, PieceColor::Black, 3, 7),
        (PieceType::King, PieceColor::Black, 4, 7),
        (PieceType::Bishop, PieceColor::Black, 5, 7),
        (PieceType::Knight, PieceColor::Black, 6, 7),
        (PieceType::Rook, PieceColor::Black, 7, 7),
        (PieceType::Pawn, PieceColor::Black, 0, 6),
        (PieceType::Pawn, PieceColor::Black, 1, 6),
        (PieceType::Pawn, PieceColor::Black, 2, 6),
        (PieceType::Pawn, PieceColor::Black, 3, 6),
        (PieceType::Pawn, PieceColor::Black, 4, 6),
        (PieceType::Pawn, PieceColor::Black, 5, 6),
        (PieceType::Pawn, PieceColor::Black, 6, 6),
        (PieceType::Pawn, PieceColor::Black, 7, 6),
    ];

    for (piece_type, piece_color, x, y) in piece_positions {
        let piece_char = match piece_type {
            PieceType::King => "K",
            PieceType::Queen => "Q",
            PieceType::Rook => "R",
            PieceType::Bishop => "B",
            PieceType::Knight => "N",
            PieceType::Pawn => "P",
        };

        let color = match piece_color {
            PieceColor::White => Color::rgb(1.0, 1.0, 1.0),
            PieceColor::Black => Color::rgb(0.0, 0.0, 0.0),
        };

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    piece_char,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 80.0,
                        color,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(x as f32 * 100.0, y as f32 * 100.0, 1.0),
                ..Default::default()
            },
            Position { x, y },
            Movable,
        ));
    }
}