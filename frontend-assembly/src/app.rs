use leptos::logging::log;
use leptos::prelude::*;
use thaw::{ButtonAppearance, ConfigProvider, Layout, Theme};
use thaw::Button;
use crate::components::editor::CodeEditor;

#[component]
pub fn App() -> impl IntoView {
    let editor_content = RwSignal::new(String::new());

    view! {
        <ConfigProvider theme=RwSignal::new(Theme::dark())>
        <Layout>
        <div class="flex gap-4 p-4">
            <div class="flex-1">
                <CodeEditor content=editor_content/>
            </div>
        </div>
        <Button appearance=ButtonAppearance::Primary on_click=move |_| log!("{}", editor_content.get())>Click me</Button>
        </Layout>
        </ConfigProvider>
    }
}