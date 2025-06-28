use crate::components::editor::CodeEditor;
use assembly_compiler::parser::parse_riscv;
use leptos::logging::log;
use leptos::prelude::*;
use thaw::Button;
use thaw::{ButtonAppearance, Card, CardHeader, ConfigProvider, Flex, Text, Theme};

#[component]
pub fn App() -> impl IntoView {
    let editor_content = RwSignal::new(String::new());
    let assembly_result = thaw_utils::Model::from(String::new());

    let click_function = move |_| {
        let assembly_text = editor_content.with(|val| val.clone());
        let parsed = parse_riscv(&assembly_text);
        let result_text = match parsed {
            Ok(program) => format!("{:?}", program),
            Err(e) => {
                // TODO: Add Ariadne error printing
                let mut string = String::new();
                for error in e {
                    string.push_str(&format!("{error}\n"));
                }
                string
            }
        };
        log!("{}", result_text);
        assembly_result.set(result_text);
    };

    view! {
        <ConfigProvider theme=RwSignal::new(Theme::dark())>
            <Flex vertical=true>
                <div class="flex gap-4 p-4">
                    <div class="flex-1">
                        <CodeEditor content=editor_content/>
                    </div>
                </div>
                <Button appearance=ButtonAppearance::Primary on_click=click_function>Parse</Button>
                <Card class="mt-4">
                    <CardHeader>
                        <b>"Parsing result"</b>
                    </CardHeader>
                    <Text style="white-space: pre-wrap; font-family: monospace">{move || assembly_result.get()}</Text>
                </Card>
            </Flex>
        </ConfigProvider>
    }
}
