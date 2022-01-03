use chiprust8::{core::Chip8Core, graphics};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Chip8LauncherArgs {
    #[clap(short, long, default_value_t=String::from("testfile"))]
    fname: String,
    #[clap(short, long)]
    no_eframe: bool,
    #[clap(short, long)]
    verbose: bool
}


fn main() {
    let args = Chip8LauncherArgs::parse();
    let log_level: log::LevelFilter = match args.verbose{
        true => log::LevelFilter::Debug,
        false => log::LevelFilter::Info
    };
    let _ = env_logger::builder()
        .filter_level(log_level)
        .is_test(true)
        .try_init();

    let adapter = graphics::graphics_adapter::GraphicsAdapter::default();
    let mut core: Chip8Core = Chip8Core::new(&args.fname, true, &adapter);

    std::thread::spawn(move || {
        core.run_loop();
    });

    if !args.no_eframe {
        let app = graphics::eframe_runner::Chip8EframeApp::new(&adapter);
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(Box::new(app), native_options);
    }
}
