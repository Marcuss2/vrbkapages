use crate::{components::{belt_viewer::BeltViewer, editor_panel::EditorPanel, state_visualization::StateVisualization, status_bar::StatusBar}, state::{AppState, ExecutionMode}};
use leptos::prelude::*;
use thaw::Flex;

#[component]
pub fn App() -> impl IntoView {
    let app_state = AppState::new();
    let code = RwSignal::new(String::new());
    let execution_state = RwSignal::new(ExecutionMode::Stopped);

    view! {
        <thaw::ConfigProvider theme=RwSignal::new(thaw::Theme::dark())>
            <thaw::Flex vertical=true>
                    <BeltViewer machine=app_state.machine.read_only() />
                <thaw::Flex>
                    <thaw::Flex vertical=true style="width: 30%;">
                        <EditorPanel execution_state=execution_state code=code />
                        <StatusBar execution_mode=execution_state.read_only() />
                    </thaw::Flex>
                    <StateVisualization state=app_state.clone() />
                </thaw::Flex>
            </thaw::Flex>
        </thaw::ConfigProvider>
    }
}
