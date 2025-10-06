use belt_interpreter::BeltMachine;
use leptos::prelude::*;

#[component]
pub fn BeltViewer(machine: ReadSignal<BeltMachine>) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <h5 class="text-lg font-medium mb-4">"Registers"</h5>
            <div class="grid grid-cols-16 gap-2">
                {move || {
                    let belt = &machine.read().belt;
                    (0..16).map(|i| {
                        let value = belt.peek_belt(i);
                        let value_text = format!("0x{:04X}", value);

                        view! {
                            <div class="text-center">
                                <div class="text-sm text-gray-400">
                                    b{i}
                                </div>
                                <div class="font-mono text-sm">
                                    {value_text}
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
