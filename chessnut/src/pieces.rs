use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

pub struct ChessPiece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Movable;