use std::sync::Arc;

use belt_interpreter::BeltMachine;
use leptos::prelude::RwSignal;

pub enum ExecutionMode {
    Stopped,
    Running,
    Step,
}

#[derive(Clone)]
pub struct AppState {
    pub machine: RwSignal<BeltMachine>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            machine: RwSignal::new(BeltMachine {
                belt: [0; 16],
                memory: [0; 65536],
            }),
        }
    }
}
