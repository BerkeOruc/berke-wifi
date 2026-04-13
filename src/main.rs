mod app;
mod ui;
mod wifi;

use app::App;

fn main() {
    let mut app = App::new();

    if let Err(e) = app.refresh() {
        eprintln!("Failed to scan networks: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = ui::run_ui(&mut app) {
        eprintln!("UI error: {}", e);
        std::process::exit(1);
    }
}
