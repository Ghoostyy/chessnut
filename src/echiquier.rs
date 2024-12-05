use crate::piece::{Couleur, Piece, TypePiece};

#[derive(Debug)]
pub struct Echiquier {
    pub plateau: [[Option<Piece>; 8]; 8],
}

impl Echiquier {
    pub fn initialiser() -> Self {
        let mut plateau = [[None; 8]; 8];

        // Initialiser les pièces noires
        plateau[0] = [
            Some(Piece::creer(Couleur::Noir, TypePiece::Tour)),
            Some(Piece::creer(Couleur::Noir, TypePiece::Cavalier)),
            Some(Piece::creer(Couleur::Noir, TypePiece::Fou)),
            Some(Piece::creer(Couleur::Noir, TypePiece::Reine)),
            Some(Piece::creer(Couleur::Noir, TypePiece::Roi)),
            Some(Piece::creer(Couleur::Noir, TypePiece::Fou)),
            Some(Piece::creer(Couleur::Noir, TypePiece::Cavalier)),
            Some(Piece::creer(Couleur::Noir, TypePiece::Tour)),
        ];

        plateau[1] = [Some(Piece::creer(Couleur::Noir, TypePiece::Pion)); 8];

        // Initialiser les pièces blanches
        plateau[6] = [Some(Piece::creer(Couleur::Blanc, TypePiece::Pion)); 8];

        plateau[7] = [
            Some(Piece::creer(Couleur::Blanc, TypePiece::Tour)),
            Some(Piece::creer(Couleur::Blanc, TypePiece::Cavalier)),
            Some(Piece::creer(Couleur::Blanc, TypePiece::Fou)),
            Some(Piece::creer(Couleur::Blanc, TypePiece::Reine)),
            Some(Piece::creer(Couleur::Blanc, TypePiece::Roi)),
            Some(Piece::creer(Couleur::Blanc, TypePiece::Fou)),
            Some(Piece::creer(Couleur::Blanc, TypePiece::Cavalier)),
            Some(Piece::creer(Couleur::Blanc, TypePiece::Tour)),
        ];

        Echiquier { plateau }
    }

    pub fn afficher(&self) {
        for ligne in &self.plateau {
            for case in ligne {
                match case {
                    Some(piece) => print!("{} ", piece.caractere()),
                    None => print!(". "),
                }
            }
            println!();
        }
    }
}
