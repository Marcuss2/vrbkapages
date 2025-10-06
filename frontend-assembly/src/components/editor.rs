use itertools::Itertools;
use leptos::prelude::*;
use leptos::{component, ev, html};

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
        // Replacing thaw::Flex with HTML and Tailwind
        <div class="flex gap-1 border-4 border-black rounded-lg">
            <div
                node_ref=line_numbers_ref
                class="p-2 border-r border-gray-300 text-right text-gray-500 overflow-hidden select-none whitespace-pre box-border"
                aria-hidden="true"
            >
                <pre class="m-0">{line_numbers}</pre>
            </div>

            <textarea
                class="p-2 border-none outline-none resize-none bg-transparent overflow-y-auto box-border whitespace-pre text-gray-300"
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
