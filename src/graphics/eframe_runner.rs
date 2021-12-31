use std::sync::Arc;

use crate::{core::Chip8DisplayData, graphics::graphics_adapter::GraphicsAdapter};
use eframe::{egui, epi};
use log::error;

pub struct Chip8EframeApp {
    fname: String,
    pub display_data: Chip8DisplayData,
    frame: Option<epi::Frame>,
    adapter: GraphicsAdapter,
}

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
        if self.check_for_updates() {
            println!("NEW DATA!");
        } else {
            // println!("No data");
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Hello world!");
                ui.label(self.fname.as_str());
                // ui.label(format!("{}", self.display_data)).unwrap().monospace();
                ui.label(egui::RichText::new(format!("{}", self.display_data)).monospace());
            })
        });
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
