use belt_interpreter::BeltMachine;
use leptos::prelude::*;
use crate::state::AppState;
use thaw::{Card, Flex, Grid, GridItem};

#[component]
pub fn BeltViewer(machine: ReadSignal<BeltMachine>) -> impl IntoView {
    view! {
        <Flex vertical=true>
                <h5 style="font-size: 16px; font-weight: 500">"Registers"</h5>
                <Grid cols=16>
                    {move || machine.read().belt.iter().enumerate().map(|(i, val)| {
                        let value_text = format!("0x{:04X}", val);
                        
                        view! {
                            <GridItem>
                                <div style="text-align: center">
                                    <div style="font-size: 12px; color: var(--color-text-secondary)">
                                        b{i}
                                    </div>
                                    <div style="font-family: monospace; font-size: 14px">
                                        {value_text}
                                    </div>
                                </div>
                            </GridItem>
                        }
                    }).collect::<Vec<_>>()}
                </Grid>
        </Flex>
    }
}
