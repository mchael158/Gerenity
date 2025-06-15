mod app;
mod tarefas;
mod persistencias;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "📝 Gerenciador de Tarefas",
        options,
        Box::new(|_cc| Ok(Box::new(app::MeuApp::new()))),
    )
}
