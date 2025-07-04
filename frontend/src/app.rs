use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use thaw::{
    Body1, Button, Caption1, Card, CardFooter, CardHeader, CardHeaderDescription, CardPreview,
    ConfigProvider, Divider, Flex, FlexAlign, FlexGap, FlexJustify, Layout, LayoutHeader, Link,
    NavDrawer, NavItem, Theme,
};

#[component]
fn BasicInfo() -> impl IntoView {
    view! {
        <Flex vertical=true gap=FlexGap::Small>
            <CardHeader>
                <Body1>
                    <h2>"Marek Vrbka"</h2>
                </Body1>
                <CardHeaderDescription slot>
                    <Caption1>"Software Developer based in Brno"</Caption1>
                </CardHeaderDescription>
            </CardHeader>
            <Divider attr:style="padding: 0.2em 0; background-color: var(--colorNeutralBackground1);"/>
            <h2>"Contact"</h2>
            <Flex justify=FlexJustify::SpaceBetween gap=FlexGap::Large>
                <Card attr:style="width: 100%;">
                    <CardHeader>
                        <Body1>
                            <h3 style="margin: 0;">"email"</h3>
                        </Body1>
                    </CardHeader>
                    <CardFooter>
                        <Link href="mailto:marekvrbka@gmail.com">"marekvrbka@gmail.com"</Link>
                    </CardFooter>
                </Card>
                <Card attr:style="width: 100%;">
                    <CardHeader>
                        <Body1>
                            <h3 style="margin: 0;">"GitHub"</h3>
                        </Body1>
                    </CardHeader>
                    <CardFooter>
                        <Link href="https://github.com/Marcuss2">"github.com/Marcuss2"</Link>
                    </CardFooter>
                </Card>
                <Card attr:style="width: 100%;">
                    <CardHeader>
                        <Body1>
                            <h3 style="margin: 0;">"LinkedIn"</h3>
                        </Body1>
                    </CardHeader>
                    <CardFooter>
                        <Link href="https://linkedin.com/in/marek-vrbka/">"linkedin.com/in/marek-vrbka/"</Link>
                    </CardFooter>
                </Card>
            </Flex>
        </Flex>
    }
}

// CV Component
#[component]
fn CV() -> impl IntoView {
    view! {
        <Flex vertical=true gap=FlexGap::Small>
            <CardHeader>
                <Body1>
                    <h2>"Curiculum Vitae"</h2>
                </Body1>
            </CardHeader>
            <Divider attr:style="padding: 0.2em 0; background-color: var(--colorNeutralBackground1);"/>
            <h2>"Work Experience"</h2>
            <Card attr:style="width: 100%;">
                <CardHeader>
                    <Body1>
                        <h3 style="margin: 0;">"Brno Faculty of Informatics"</h3>
                    </Body1>
                    <CardHeaderDescription slot>
                        "Teaching Assistant"
                    </CardHeaderDescription>
                </CardHeader>
                <CardPreview>
                    <ul>
                        <li>"Rust, Python, C"</li>
                        <li>"Teaching the Rust Programming Language and embedded systems"</li>
                        <li>"Managing small groups of students"</li>
                        <li>"Wrote automated homework testing systems"</li>
                    </ul>
                </CardPreview>
                <CardFooter>
                    "January 2022 - Present (Every Autumn)"
                </CardFooter>
            </Card>
            <Card attr:style="width: 100%;">
                <CardHeader>
                    <Body1>
                        <h3 style="margin: 0;">"Codasip s.r.o."</h3>
                    </Body1>
                    <CardHeaderDescription slot>
                        "Software Developer"
                    </CardHeaderDescription>
                </CardHeader>
                <CardPreview>
                    <ul>
                        <li>"C, C++, Python"</li>
                        <li>"Maintained and developed OpenOCD"</li>
                        <li>"Developed tooling for generating SystemVerilog"</li>
                    </ul>
                </CardPreview>
                <CardFooter>
                    "March 2023 - Present"
                </CardFooter>
            </Card>
            <Divider attr:style="padding: 0.2em 0; background-color: var(--colorNeutralBackground1);"/>
            <h2>"Education"</h2>
            <Card attr:style="width: 100%;">
                <CardHeader>
                    <Body1>
                        <h3 style="margin: 0;">"Computer Systems, Communication and Security: Hardware Systems"</h3>
                    </Body1>
                    <CardHeaderDescription slot>
                        "Master's Degree"
                    </CardHeaderDescription>
                </CardHeader>
                <CardPreview>
                    <ul>
                        <li>"Thesis: " <Link href="https://is.muni.cz/th/m6ad7/">"Testing of Student Assignments in Embedded Course"</Link></li>
                    </ul>
                </CardPreview>
                <CardFooter>
                    "Brno Faculty of Informatics • 2020 - 2023"
                </CardFooter>
            </Card>
            <Card attr:style="width: 100%;">
                <CardHeader>
                    <Body1>
                        <h3 style="margin: 0;">"Applied Systems"</h3>
                    </Body1>
                    <CardHeaderDescription slot>
                        "Bachelor's Degree"
                    </CardHeaderDescription>
                </CardHeader>
                <CardPreview>
                    <ul>
                        <li>"Thesis: " <Link href="https://is.muni.cz/th/ego6j/">"Encrypted Network Traffic Classification and Analysis"</Link></li>
                    </ul>
                </CardPreview>
                <CardFooter>
                    "Brno Faculty of Informatics • 2017 - 2020"
                </CardFooter>
            </Card>
        </Flex>
    }
}

// Main App Component
#[component]
pub fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::light());

    let change_theme = move |_| {
        theme.update(|old_theme| {
            if old_theme.name == "dark" {
                *old_theme = Theme::light();
            } else {
                *old_theme = Theme::dark();
            }
        });
    };

    view! {
            <ConfigProvider theme=theme>
                <Router>
                    <Layout attr:style="displat: flex; flex-direction: column; min-height: 100vh;">
                        <LayoutHeader attr:style="padding:1em;">
                            <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Center gap=FlexGap::Large>
                                <NavDrawer>
                                    <Flex>
                                        <NavItem value="basic_info" href="/">
                                            "Basic Info"
                                        </NavItem>
                                        <NavItem value="cv" href="/cv">
                                            "CV"
                                        </NavItem>
                                    </Flex>
                                </NavDrawer>
                                <div>
                                    <Button on_click=change_theme>"Theme"</Button>
                                </div>
                            </Flex>
                        </LayoutHeader>
                        <div style="padding: 0; background-color: var(--colorNeutralBackground1);">
                            <Divider/>
                        </div>
                        <Layout attr:style="padding: 1em">
                         <div style="padding-top: 2em; padding-left: 10em; padding-right: 10em;">
                            <Routes fallback=|| "This page could not be found.">
                                <Route path=path!("/") view=|| view! { <BasicInfo/> }/>
                                <Route path=path!("/cv") view=|| view! { <CV/> }/>
                            </Routes>
                        </div>
                        </Layout>
                    </Layout>
                </Router>
            </ConfigProvider>
    }
}
