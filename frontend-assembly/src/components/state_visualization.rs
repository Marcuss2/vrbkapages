use crate::components::memory_viewer::MemoryViewer;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn StateVisualization(state: AppState) -> impl IntoView {
    view! {
        // Replacing thaw components with HTML and Tailwind
        <div class="flex flex-col p-4 h-full w-3/5">
            <h5 class="text-lg font-medium mb-4">"Machine State"</h5>

            <div class="flex space-x-4 mb-6">
                // Program Counter Card
                <div class="bg-gray-800 rounded-lg shadow-lg border border-gray-700 flex-1">
                    <div class="px-4 py-2 border-b border-gray-700">
                        "Program Counter"
                    </div>
                    <div class="px-4 py-3">
                        {move || format!("0x{:04X}", state.machine.read().pc)}
                    </div>
                </div>

                // Stack Pointer Card
                <div class="bg-gray-800 rounded-lg shadow-lg border border-gray-700 flex-1">
                    <div class="px-4 py-2 border-b border-gray-700">
                        "Stack Pointer"
                    </div>
                    <div class="px-4 py-3">
                        {move || format!("0x{:04X}", state.machine.read().pc)}
                    </div>
                </div>
            </div>

            <MemoryViewer state=state.clone() />
        </div>
    }
}
