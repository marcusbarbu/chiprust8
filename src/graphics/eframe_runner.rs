
use crate::{core::Chip8DisplayData, graphics::graphics_adapter::GraphicsAdapter};
use eframe::{egui::{self, Widget}, epi};
use log::error;

pub struct Chip8EframeApp {
    fname: String,
    pub display_data: Chip8DisplayData,
    frame: Option<epi::Frame>,
    adapter: GraphicsAdapter,
}

struct Chip8EframeDisplayData {
    dd: Chip8DisplayData,
    width_sim_pixels: usize,
    height_sim_pixels: usize,
    sim_pixel_dim: (f32, f32),
}

// impl Default for Chip8EframeDisplayData {
//     fn default() -> Self {
//         let dd = Chip8EframeDisplayData::default();
//         let w: usize = dd.dis[0].len();
//         let h: usize = dd.len();

//         Self { dd: Default::default(), width_sim_pixels: dd[0].len(), height_sim_pixels: Default::default(), sim_pixel_dim: Default::default() }
//     }
// }

impl Chip8EframeApp {
    pub fn new(adapter: &GraphicsAdapter) -> Chip8EframeApp {
        println!("Generating eframe app!");
        Chip8EframeApp {
            fname: String::from(""),
            display_data: Chip8DisplayData::default(),
            frame: None,
            adapter: adapter.clone(),
        }
    }

    fn check_for_updates(&mut self) -> bool {
        match self
            .adapter
            .display_state_receiver
            .recv_timeout(std::time::Duration::from_micros(1))
        {
            Ok(v) => {
                self.display_data = v;
                true
            }
            Err(_) => false,
        }
    }
}

impl epi::App for Chip8EframeApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        // println!("update!");
        let _ = self.check_for_updates();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Hello world!");
                ui.label(self.fname.as_str());
                ui.label(egui::RichText::new(format!("{}", self.display_data)).monospace());
                for k in &ctx.input().keys_down {
                    ui.label(format!("Key {:?} down", k));
                }
            })
        });

        if ctx.input().key_pressed(egui::Key::Q) {
            _frame.quit();
        }

    }

    fn name(&self) -> &str {
        "Chip8 eFrame-based Graphics"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        println!("Setting up!");
        self.frame = Some(_frame.clone());
        let bf: Box<epi::Frame> = Box::new(_frame.clone());
        std::thread::spawn(move || loop {
            bf.request_repaint();
            // println!("From that thread");
            std::thread::sleep(std::time::Duration::from_millis(10));
        });
    }
}
