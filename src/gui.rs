use crate::partie::Partie;
use eframe::egui;
use std::process; // Importation pour utiliser process::exit

pub struct ChessGame {
    partie: Partie,
    selected: Option<(usize, usize)>,
    message: String,
    game_status: String,
}

impl ChessGame {
    pub fn new() -> Self {
        ChessGame {
            partie: Partie::nouvelle(),
            selected: None,
            message: String::new(),
            game_status: String::new(),
        }
    }
}

impl eframe::App for ChessGame {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Joueur actuel :");
                if self.partie.joueur_noir {
                    ui.colored_label(egui::Color32::BLACK, "Noir");
                } else {
                    ui.colored_label(egui::Color32::WHITE, "Blanc");
                }
                if ui.button("Annuler dernier coup").clicked() {
                    self.message = if self.partie.annuler_deplacement() {
                        "Dernier coup annulé.".to_string()
                    } else {
                        "Aucun coup à annuler.".to_string()
                    };
                }
                if ui.button("Quitter").clicked() {
                    // Utilisation de std::process::exit(0) pour quitter l'application
                    process::exit(0); // Cela ferme immédiatement l'application
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Jeu d'Échecs");
            });

            // Afficher l'échiquier avec coordonnées
            ui.add_space(10.0);
            self.render_board(ui);

            // Afficher un message ou une notification
            if !self.message.is_empty() {
                ui.label(&self.message);
            }
        });

        ctx.request_repaint();
    }
}

impl ChessGame {
    fn render_board(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("chess_board")
            .spacing([4.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                for y in (0..8).rev() {
                    // Ajout de l'étiquette de la ligne
                    ui.label(format!("{}", y + 1));

                    for x in 0..8 {
                        let piece = self.partie.echiquier.plateau[y][x];

                        let button_label = match piece {
                            Some(p) => p.caractere().to_string(),
                            None => "".to_string(),
                        };

                        let base_color = if (x + y) % 2 == 0 {
                            egui::Color32::DARK_GRAY
                        } else {
                            egui::Color32::DARK_BLUE
                        };

                        let color = if self.selected == Some((x, y)) {
                            egui::Color32::GOLD
                        } else {
                            base_color
                        };

                        if ui
                            .add_sized([40.0, 40.0], egui::Button::new(button_label).fill(color))
                            .clicked()
                        {
                            self.handle_click(x, y);
                        }
                    }
                    ui.end_row();
                }

                // Ajouter les étiquettes des colonnes
                ui.horizontal(|ui| {
                    ui.label(" ");
                    for x in 0..8 {
                        ui.label(format!("{}", ('A' as u8 + x as u8) as char));
                    }
                });
            });
    }

    fn handle_click(&mut self, x: usize, y: usize) {
        if let Some((sx, sy)) = self.selected {
            if self.partie.deplacement(sx, sy, x, y) {
                self.message = format!(
                    "Déplacement effectué : {}{} vers {}{}",
                    (sx as u8 + b'A') as char,
                    sy + 1,
                    (x as u8 + b'A') as char,
                    y + 1
                );
            } else {
                self.message = "Déplacement invalide.".to_string();
            }
            self.selected = None;
        } else {
            self.selected = Some((x, y));
            self.message = format!("Case sélectionnée : {}{}", (x as u8 + b'A') as char, y + 1);
        }
    }
}
