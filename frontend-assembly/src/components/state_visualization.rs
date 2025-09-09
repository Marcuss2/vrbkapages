use leptos::prelude::*;
use thaw::{CardFooter, CardHeader, CardPreview};
use crate::state::AppState;
use crate::components::{belt_viewer::BeltViewer, memory_viewer::MemoryViewer};

#[component]
pub fn StateVisualization(state: AppState) -> impl IntoView {
    view! {
        <thaw::Flex vertical=true style="padding: 16px; height: 100%; width: 60%">
            <h5 style="font-size: 16px; font-weight: 500">"Machine State"</h5>
            
            <thaw::Flex>
                <thaw::Card>
                    <CardHeader>
                        "Program Counter"
                    </CardHeader>
                    <CardFooter>{move || format!("0x{:04X}", 0x1234/*state.machine.program_counter*/)}</CardFooter>
                </thaw::Card>
                <thaw::Card>
                    <thaw::CardHeader>"Stack Pointer"</thaw::CardHeader>
                    <CardPreview>
                        {move || format!("0x{:04X}", 0x6789/*state.machine.stack_pointer*/)}
                    </CardPreview>
                </thaw::Card>
            </thaw::Flex>
            

            <MemoryViewer state=state.clone() />
        </thaw::Flex>
    }
}