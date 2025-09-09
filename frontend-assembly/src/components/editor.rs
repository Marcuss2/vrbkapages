use itertools::Itertools;
use leptos::prelude::*;
use leptos::{component, ev, html};
use thaw::{Flex, FlexGap};

///
/// Simple code editor component which always updates on each change.
/// Not very performant, but good enough for now.
///
#[component]
pub fn CodeEditor(content: RwSignal<String>) -> impl IntoView {
    let line_numbers_ref = NodeRef::<html::Div>::new();

    let line_numbers = Memo::new(move |_| {
        let count = content.with(|text| text.lines().count() + text.ends_with('\n') as usize);
        (1..=count.max(1)).map(|n| n.to_string()).join("\n")
    });

    let sync_scroll = move |ev: ev::Event| {
        let textarea = event_target::<web_sys::HtmlTextAreaElement>(&ev);
        let scroll_top = textarea.scroll_top();
        if let Some(ref div) = line_numbers_ref.get() {
            div.set_scroll_top(scroll_top);
        }
    };

    view! {
        <Flex vertical=false gap=FlexGap::Small
            style="
                border: 5px solid black;
                border-radius: 10px;">
            <div
                node_ref=line_numbers_ref
                style="
                    padding: 10px;
                    border-right: 1px solid #ddd;
                    text-align: right;
                    color: #666;
                    overflow-y: hidden;
                    user-select: none;
                    white-space: pre;
                    box-sizing: border-box;
                "
                aria-hidden="true"
            >
                <pre style="margin: 0">{line_numbers}</pre>
            </div>

            <textarea
                style="
                    padding: 10px;
                    border: none;
                    outline: none;
                    resize: none;
                    background: transparent;
                    overflow-y: auto;
                    box-sizing: border-box;
                    white-space: pre;
                    font-family: inherit;
                    font-size: inherit;
                    line-height: inherit;
                    color: var(--color-text-secondary);
                "
                rows=20
                cols=80
                on:input=move |ev| content.set(event_target_value(&ev))
                on:scroll=sync_scroll
                prop:value=content.get_untracked()
                spellcheck="false"
            />
        </Flex>
    }
}
