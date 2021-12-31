use chiprust8::{graphics, core::Chip8Core};

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();
    let adapter = graphics::graphics_adapter::GraphicsAdapter::default();
    let app = graphics::eframe_runner::Chip8EframeApp::new(&adapter);
    let native_options = eframe::NativeOptions::default();
    let mut core: Chip8Core = Chip8Core::new("testfile", true, &adapter);

    std::thread::spawn(move || {
        core.run_loop();
    });

    eframe::run_native(Box::new(app), native_options);
}
