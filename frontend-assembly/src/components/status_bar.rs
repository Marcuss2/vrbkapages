use crate::state::{AppState, ExecutionMode};
use leptos::prelude::*;
use thaw::Flex;

#[component]
pub fn StatusBar(execution_mode: ReadSignal<ExecutionMode>) -> impl IntoView {
    view! {
        <Flex
            style="padding: 8px 16px; background-color: var(--color-bg-2)"
            justify=thaw::FlexJustify::SpaceBetween
        >
            <Flex>
                <span style="font-size: 14px">
                    "Status: " {move || match *execution_mode.read() {
                        ExecutionMode::Stopped => "Stopped",
                        ExecutionMode::Running => "Running",
                        ExecutionMode::Step => "Step",
                    }}
                </span>
                <span style="font-size: 14px">
                    "Errors: None" 
                </span>
            </Flex>
            <span style="font-size: 14px; color: var(--color-text-secondary)">
                "When backend developer writes UI"
            </span>
        </Flex>
    }
}
