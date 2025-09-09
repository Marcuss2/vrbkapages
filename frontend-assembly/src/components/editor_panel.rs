use leptos::prelude::*;
use crate::components::editor::CodeEditor;
use crate::state::{AppState, ExecutionMode};

#[component]
pub fn EditorPanel(execution_state: RwSignal<ExecutionMode> , code: RwSignal<String>) -> impl IntoView {
    view! {
        <thaw::Card>
            <thaw::CardHeader>
                "Assembly Editor"
            </thaw::CardHeader>
                <CodeEditor content=code />
            <thaw::CardFooter>
                "Execution controls"//<ExecutionControls state=state.execution_mode />
            </thaw::CardFooter>
        </thaw::Card>
    }
}
