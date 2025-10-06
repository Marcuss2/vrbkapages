use crate::components::editor::CodeEditor;
use crate::state::ExecutionMode;
use leptos::prelude::*;

#[component]
pub fn EditorPanel(
    execution_state: RwSignal<ExecutionMode>,
    code: RwSignal<String>,
) -> impl IntoView {
    view! {
        // Replacing thaw::Card with HTML and Tailwind
        <div class="bg-gray-800 rounded-lg shadow-lg border border-gray-700 m-4">
            <div class="px-6 py-4 border-b border-gray-700">
                "Assembly Editor"
            </div>
            <div class="p-6">
                <CodeEditor content=code />
            </div>
            <div class="px-6 py-4 border-t border-gray-700">
                <ExecutionControls execution_state=execution_state />
            </div>
        </div>
    }
}


#[component]
pub fn ExecutionControls(execution_state: RwSignal<ExecutionMode>) -> impl IntoView {
    let step = move |_| {
        execution_state.update(|state| *state = ExecutionMode::Step);
    };

    let run = move |_| {
        execution_state.update(|state| *state = ExecutionMode::Running);
    };

    let stop = move |_| {
        execution_state.update(|state| *state = ExecutionMode::Stopped);
    };

    view! {
        <div class="flex items-center gap-4">
            <button
                on:click=step
                class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded"
            >
                "Step"
            </button>
            <button
                on:click=run
                class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded"
            >
                "Run"
            </button>
            <button
                on:click=stop
                class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded"
            >
                "Stop"
            </button>
        </div>
    }
}