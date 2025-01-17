pub(crate) mod app;

pub fn main() {
    leptos::mount::mount_to_body(app::App)
}
