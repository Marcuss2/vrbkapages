use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn MemoryViewer(state: AppState) -> impl IntoView {
    let start_address = RwSignal::new("0x0".to_string());

    view! {
        // Replacing thaw components with HTML and Tailwind
        <div class="bg-gray-800 rounded-lg shadow-lg border border-gray-700">
            <div class="px-4 py-2 border-b border-gray-700 flex justify-between items-center">
                <h5 class="text-lg font-medium">"Memory Viewer"</h5>
                // Simple input for start address - in a real implementation you might want a proper input component
                <input
                    type="text"
                    placeholder="Start address"
                    class="bg-gray-700 text-white px-2 py-1 rounded text-sm"
                    on:input=move |ev| start_address.set(event_target_value(&ev))
                    prop:value=start_address.get_untracked()
                />
            </div>
            <div class="p-4">
                <div style="max-height: 300px;" class="overflow-y-auto font-mono text-xs">
                    <div class="grid grid-cols-4 gap-2">
                        {move || {
                            // Note: This is a simplified version. In a real implementation, you'd want proper error handling
                            let start = usize::from_str_radix(&*start_address.read().strip_prefix("0x").unwrap_or("0"), 16).unwrap_or(0);
                            (start..start + 32).map(|addr| {
                                let value = state.machine.read().memory[addr as usize];
                                view! {
                                    <div class="flex justify-between text-gray-300">
                                        <span>
                                            {format!("0x{:04X}: 0x{:04X}", addr, value)}
                                        </span>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
