pub(crate) mod app;
pub(crate) mod components;
pub(crate) mod context;
pub(crate) mod state;

pub fn main() {
    leptos::mount::mount_to_body(app::App)
}
