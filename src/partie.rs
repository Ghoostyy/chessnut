use crate::echiquier::Echiquier;
use crate::piece::{Couleur, Piece, TypePiece};

pub struct Partie {
    pub echiquier: Echiquier,
    pub joueur_noir: bool,
    pub historique_coups: Vec<((usize, usize), (usize, usize), Option<Piece>)>,
    pub dernier_coup: Option<((usize, usize), (usize, usize))>,
}

impl Partie {
    pub fn nouvelle() -> Self {
        Partie {
            echiquier: Echiquier::initialiser(),
            joueur_noir: true, // Le joueur noir commence
            historique_coups: vec![],
            dernier_coup: None,
        }
    }
    pub fn est_en_echec(&self, couleur: Couleur) -> bool {
        // Chercher la position du roi de la couleur donnée
        let (roi_x, roi_y) = self.trouver_roi(couleur);

        // Vérifier si une pièce de l'adversaire peut attaquer le roi
        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = self.echiquier.plateau[y][x] {
                    if piece.couleur != couleur {
                        if self.deplacement_valide(x, y, roi_x, roi_y) {
                            return true; // Le roi est en échec
                        }
                    }
                }
            }
        }

        false
    }

    fn trouver_roi(&self, couleur: Couleur) -> (usize, usize) {
        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = self.echiquier.plateau[y][x] {
                    if piece.couleur == couleur && piece.type_piece == TypePiece::Roi {
                        return (x, y);
                    }
                }
            }
        }
        panic!("Roi non trouvé sur le plateau"); // Cela ne devrait jamais arriver si le plateau est correctement initialisé
    }

    pub fn est_echec_et_mat(&self, couleur: Couleur) -> bool {
        if !self.est_en_echec(couleur) {
            return false;
        }

        // Vérifier si le roi peut se déplacer ou s'il peut être protégé
        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = self.echiquier.plateau[y][x] {
                    if piece.couleur == couleur {
                        if self.deplacement_valide(x, y, x, y) {
                            return false;
                        }
                    }
                }
            }
        }

        // Si aucune des pièces ne peut protéger ou déplacer le roi, c'est un échec et mat
        true
    }

    pub fn deplacement(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        println!(
            "Tentative de déplacement de ({}, {}) vers ({}, {})",
            x1, y1, x2, y2
        );

        // Vérifier si le déplacement est valide
        if self.deplacement_valide(x1, y1, x2, y2) {
            if let Some(piece) = self.echiquier.plateau[y1][x1] {
                // Vérification "en passant"
                if let Some(((dernier_x1, dernier_y1), (dernier_x2, dernier_y2))) =
                    self.dernier_coup
                {
                    if piece.type_piece == TypePiece::Pion
                        && dernier_y2 == y1
                        && dernier_x2 == x2
                        && (dernier_y2 as isize - dernier_y1 as isize).abs() == 2
                    {
                        // Capture "en passant"
                        let pion_capture = self.echiquier.plateau[dernier_y2][dernier_x2];
                        self.historique_coups
                            .push(((x1, y1), (x2, y2), pion_capture)); // Capture du pion en passant

                        // Supprimer le pion capturé
                        self.echiquier.plateau[dernier_y2][dernier_x2] = None;
                        println!("Capture en passant effectuée !");
                    }
                }

                // Sauvegarder le dernier coup
                let piece_capturee = self.echiquier.plateau[y2][x2];
                self.dernier_coup = Some(((x1, y1), (x2, y2)));

                // Déplacer la pièce
                self.echiquier.plateau[y2][x2] = Some(piece);
                self.echiquier.plateau[y1][x1] = None;

                // Sauvegarder l'historique
                self.historique_coups
                    .push(((x1, y1), (x2, y2), piece_capturee));

                // Changer de joueur
                self.joueur_noir = !self.joueur_noir;

                // Vérifier si le roi est en échec après le déplacement
                if self.est_en_echec(piece.couleur) {
                    println!("Erreur : Le roi est en échec après le déplacement.");
                    // Annuler le coup
                    self.annuler_deplacement();
                    return false;
                }

                println!("Plateau après déplacement : {:?}", self.echiquier);
                return true;
            }
        }
        false
    }

    pub fn annuler_deplacement(&mut self) -> bool {
        if let Some(((x1, y1), (x2, y2), piece_capturee)) = self.historique_coups.pop() {
            self.echiquier.plateau[y1][x1] = self.echiquier.plateau[y2][x2];
            self.echiquier.plateau[y2][x2] = piece_capturee;

            self.joueur_noir = !self.joueur_noir;
            return true;
        }
        false
    }

    fn deplacement_valide(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        if let Some(piece) = self.echiquier.plateau[y1][x1] {
            // Vérifier si la pièce appartient au joueur actuel
            if (piece.couleur == Couleur::Noir && !self.joueur_noir)
                || (piece.couleur == Couleur::Blanc && self.joueur_noir)
            {
                println!("Erreur : tentative de déplacement d'une pièce adverse.");
                return false; // Le joueur ne peut pas déplacer une pièce adverse
            }

            // Vérifier si la case cible est occupée par une pièce alliée
            if let Some(cible) = self.echiquier.plateau[y2][x2] {
                if cible.couleur == piece.couleur {
                    println!("Erreur : tentative de capturer une pièce alliée.");
                    return false; // Impossible de capturer une pièce alliée
                }
            }

            // Si le mouvement concerne le roi et que c'est un roque
            if piece.type_piece == TypePiece::Roi {
                // Vérification du roque
                return self.deplacement_roi(x1, y1, x2, y2);
            }

            // Valider le déplacement selon le type de pièce
            match piece.type_piece {
                TypePiece::Pion => self.deplacement_pion(x1, y1, x2, y2, piece.couleur),
                TypePiece::Tour => self.deplacement_tour(x1, y1, x2, y2),
                TypePiece::Cavalier => self.deplacement_cavalier(x1, y1, x2, y2),
                TypePiece::Fou => self.deplacement_fou(x1, y1, x2, y2),
                TypePiece::Reine => self.deplacement_reine(x1, y1, x2, y2),
                TypePiece::Roi => self.deplacement_roi(x1, y1, x2, y2),
            }
        } else {
            println!("Erreur : aucune pièce à déplacer en ({}, {}).", x1, y1);
            false
        }
    }

    fn deplacement_pion(
        &self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        couleur: Couleur,
    ) -> bool {
        let direction = if couleur == Couleur::Blanc { -1 } else { 1 };
        let position_initiale = if couleur == Couleur::Blanc { 6 } else { 1 };

        // Vérification du déplacement d'une case en avant
        if x1 == x2 && (y2 as isize - y1 as isize) == direction {
            if self.echiquier.plateau[y2][x2].is_none() {
                return true;
            }
        }

        // Vérification du déplacement initial de deux cases
        if x1 == x2 && (y2 as isize - y1 as isize) == 2 * direction && y1 == position_initiale {
            if self.echiquier.plateau[(y1 as isize + direction) as usize][x1].is_none()
                && self.echiquier.plateau[y2][x2].is_none()
            {
                return true;
            }
        }

        // Vérification de la capture en diagonale
        if (x2 as isize - x1 as isize).abs() == 1 && (y2 as isize - y1 as isize) == direction {
            // Capture normale
            if let Some(piece_cible) = self.echiquier.plateau[y2][x2] {
                if piece_cible.couleur != couleur {
                    return true;
                }
            }

            // Capture "en passant"
            if let Some(((dernier_x1, dernier_y1), (dernier_x2, dernier_y2))) = self.dernier_coup {
                if dernier_y2 == y1
                    && dernier_x2 == x2
                    && (dernier_y2 as isize - dernier_y1 as isize).abs() == 2
                {
                    return true;
                }
            }
        }

        false
    }

    fn deplacement_tour(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        if x1 != x2 && y1 != y2 {
            return false; // La Tour se déplace en ligne droite, soit horizontalement, soit verticalement
        }

        // Vérifier si la voie est dégagée (sur une ligne ou une colonne)
        if x1 == x2 {
            let (start, end) = if y1 < y2 { (y1 + 1, y2) } else { (y2 + 1, y1) };
            for i in start..end {
                if self.echiquier.plateau[i][x1].is_some() {
                    return false; // Une pièce bloque la voie
                }
            }
        } else {
            let (start, end) = if x1 < x2 { (x1 + 1, x2) } else { (x2 + 1, x1) };
            for i in start..end {
                if self.echiquier.plateau[y1][i].is_some() {
                    return false; // Une pièce bloque la voie
                }
            }
        }

        true
    }

    fn deplacement_cavalier(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        // Le Cavalier fait un mouvement en "L"
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = (y2 as isize - y1 as isize).abs();

        (dx == 2 && dy == 1) || (dx == 1 && dy == 2)
    }

    fn deplacement_fou(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        // Le Fou se déplace en diagonale
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = (y2 as isize - y1 as isize).abs();

        if dx != dy {
            return false; // Le mouvement n'est pas une diagonale
        }

        let step_x = if x2 > x1 { 1 } else { -1 };
        let step_y = if y2 > y1 { 1 } else { -1 };

        // Vérifier si les cases intermédiaires sont libres
        let mut x = x1 as isize + step_x;
        let mut y = y1 as isize + step_y;

        while x != x2 as isize && y != y2 as isize {
            if self.echiquier.plateau[y as usize][x as usize].is_some() {
                return false; // Une pièce bloque la trajectoire
            }
            x += step_x;
            y += step_y;
        }

        true
    }

    fn deplacement_reine(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        // La Reine se déplace comme une Tour ou un Fou
        self.deplacement_tour(x1, y1, x2, y2) || self.deplacement_fou(x1, y1, x2, y2)
    }

    fn deplacement_roi(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        // Le Roi se déplace d'une case dans n'importe quelle direction
        (x2 as isize - x1 as isize).abs() <= 1 && (y2 as isize - y1 as isize).abs() <= 1
    }
}