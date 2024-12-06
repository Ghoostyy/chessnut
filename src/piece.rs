#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Couleur {
    Blanc,
    Noir,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypePiece {
    Pion,
    Tour,
    Cavalier,
    Fou,
    Roi,
    Reine,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub couleur: Couleur,      // Champs public pour accéder à la couleur
    pub type_piece: TypePiece, // Champs public pour accéder au type de pièce
}

impl Piece {
    // Méthode pour créer une nouvelle pièce avec une couleur et un type
    pub fn creer(couleur: Couleur, type_piece: TypePiece) -> Self {
        Piece {
            couleur,
            type_piece,
        }
    }

    // Méthode pour obtenir le caractère représentant la pièce
    pub fn caractere(&self) -> char {
        match (self.type_piece, self.couleur) {
            (TypePiece::Pion, Couleur::Blanc) => '♙',
            (TypePiece::Pion, Couleur::Noir) => '♟',
            (TypePiece::Tour, Couleur::Blanc) => '♖',
            (TypePiece::Tour, Couleur::Noir) => '♜',
            (TypePiece::Cavalier, Couleur::Blanc) => '♘',
            (TypePiece::Cavalier, Couleur::Noir) => '♞',
            (TypePiece::Fou, Couleur::Blanc) => '♗',
            (TypePiece::Fou, Couleur::Noir) => '♝',
            (TypePiece::Roi, Couleur::Blanc) => '♔',
            (TypePiece::Roi, Couleur::Noir) => '♚',
            (TypePiece::Reine, Couleur::Blanc) => '♕',
            (TypePiece::Reine, Couleur::Noir) => '♛',
        }
    }
}
