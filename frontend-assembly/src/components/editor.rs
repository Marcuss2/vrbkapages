use leptos::prelude::*;
use leptos::{component, ev, html};
use itertools::Itertools;


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
        line_numbers_ref.get().map(|div| {
            div.set_scroll_top(scroll_top);
        });
    };

    view! {
        <div style="
            display: grid;
            grid-template-columns: auto 1fr;
            font-family: monospace;
            font-size: 14px;
            line-height: 1.5;
            background: #f6f8fa;
            border: 1px solid #ddd;
            border-radius: 4px;
        ">
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
                "
                rows=20
                cols=80
                on:input=move |ev| content.set(event_target_value(&ev))
                on:scroll=sync_scroll
                prop:value=content.get_untracked()
                spellcheck="false"
            />
        </div>
    }
}