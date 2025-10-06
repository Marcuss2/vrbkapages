use crate::state::ExecutionMode;
use leptos::prelude::*;

#[component]
pub fn StatusBar(execution_mode: ReadSignal<ExecutionMode>) -> impl IntoView {
    view! {
        // Replacing thaw::Flex with HTML and Tailwind
        <div class="flex justify-between p-2 bg-gray-800">
            <div class="flex space-x-4">
                <span class="text-sm">
                    "Status: " {move || match *execution_mode.read() {
                        ExecutionMode::Stopped => "Stopped",
                        ExecutionMode::Running => "Running",
                        ExecutionMode::Step => "Step",
                    }}
                </span>
                <span class="text-sm">
                    "Errors: None"
                </span>
            </div>
            <span class="text-sm text-gray-400">
                "When backend developer writes UI"
            </span>
        </div>
    }
}
