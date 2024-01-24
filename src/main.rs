mod simulation;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([512.0, 512.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My simulation!",
        options,
        Box::new(|_cc| {
            Box::<simulation::Simulation>::default()
        }),
    )
}