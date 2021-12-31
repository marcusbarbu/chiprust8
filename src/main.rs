use chiprust8::graphics;

fn main() {
    println!("test");
    let adapter = graphics::graphics_adapter::GraphicsAdapter::default();
    let app = graphics::eframe_runner::Chip8EframeApp::new(adapter);
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options);
}
