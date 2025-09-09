use leptos::prelude::*;
use crate::state::AppState;
use thaw::{Card, Input, Flex};

#[component]
pub fn MemoryViewer(state: AppState) -> impl IntoView {
    let start_address = RwSignal::new("0x0".to_string());
    
    let parser = move |v: String| {
        if !v.starts_with("0x") || u16::from_str_radix(v.strip_prefix("0x").unwrap(), 16).is_err() {
            None
        } else {
            Some(v)
        }
    };
    
    view! {
        <Card>
            <thaw::CardHeader>
                <Flex>
                    <h5 style="font-size: 16px; font-weight: 500">"Memory Viewer"</h5>
                    <Input
                        placeholder="Start address"
                        value=start_address
                        parser
                    />
                </Flex>
            </thaw::CardHeader>
            <thaw::CardPreview>
                <div style="max-height: 300px; overflow-y: auto; font-family: monospace; font-size: 12px;">
                    <thaw::Grid cols=4 x_gap=2 y_gap=2>
                        {move || {
                            let start = usize::from_str_radix(&*start_address.read().strip_prefix("0x").unwrap(), 16).unwrap();
                            (start..start + 32).map(|addr| {
                                let value = state.machine.read().memory[addr as usize];
                                view! {
                                    <div style="display: flex; justify-content: space-between;">
                                        <span style="color: var(--color-text-secondary)">
                                            {format!("0x{:04X}: 0x{:04X}", addr, value)}
                                        </span>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </thaw::Grid>
                </div>
            </thaw::CardPreview>
        </Card>
    }
}
