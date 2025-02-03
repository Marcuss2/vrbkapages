use std::sync::{Arc, Mutex};
use reactive_stores::Store;

#[derive(Store)]
pub struct EditorContext {
    pub get_val: Option<Box<dyn Fn() -> String>>,
}