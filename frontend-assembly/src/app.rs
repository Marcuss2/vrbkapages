use crate::{
    components::{
        belt_viewer::BeltViewer, editor_panel::EditorPanel,
        state_visualization::StateVisualization, status_bar::StatusBar,
    },
    state::{AppState, ExecutionMode},
};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let app_state = AppState::new();
    let code = RwSignal::new(String::new());
    let execution_state = RwSignal::new(ExecutionMode::Stopped);
    
    let (exection_state_read, execution_state_write) = execution_state.split();
    
    exection_state_read.with(|execution_mode| {
        match execution_mode {
            ExecutionMode::Stopped => {},
            ExecutionMode::Running => {},
            ExecutionMode::Step => {
                app_state.machine.update(|machine| { machine.step(); });
                execution_state_write.set(ExecutionMode::Stopped);
            },
        };
    });

    view! {
        // Using dark theme via Tailwind classes
        <div class="min-h-screen bg-gray-900 text-white">
            <div class="flex flex-col">
                <BeltViewer machine=app_state.machine.read_only() />
                <div class="flex">
                    <div class="flex flex-col w-1/3">
                        <EditorPanel execution_state=execution_state code=code />
                        <StatusBar execution_mode=execution_state.read_only() />
                    </div>
                    <StateVisualization state=app_state.clone() />
                </div>
            </div>
        </div>
    }
}
