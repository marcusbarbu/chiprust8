use crate::core::Chip8DisplayData;
use crossbeam::channel::{unbounded, Receiver, Sender};

#[derive(Clone)]
pub struct GraphicsAdapter {
    pub display_state_receiver: Receiver<Chip8DisplayData>,
    pub display_state_sender: Sender<Chip8DisplayData>,
    pub key_state_receiver: Receiver<[u8;16]>,
    pub key_state_sender: Sender<[u8;16]>,
}

impl Default for GraphicsAdapter {
    fn default() -> Self {
        GraphicsAdapter::new()
    }
}

impl GraphicsAdapter {
    pub fn new() -> GraphicsAdapter {
        let (dss, dsr) = unbounded::<Chip8DisplayData>();
        let (kss, ksr) = unbounded::<[u8;16]>();
        GraphicsAdapter{
            display_state_receiver: dsr,
            display_state_sender: dss,
            key_state_receiver: ksr,
            key_state_sender: kss
        }
    }
}