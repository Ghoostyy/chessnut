mod echiquier;
mod gui;
mod partie;
mod piece;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Jeu d'Échecs",
        options,
        Box::new(|_cc| Box::new(gui::ChessGame::new())),
    )
}
