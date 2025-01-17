use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;

#[component]
fn BasicInfo() -> impl IntoView {
    view! {
        <div class="p-8 bg-gradient-to-br from-gray-50 to-white rounded-lg shadow-sm">
            <div class="max-w-4xl mx-auto">
                <h1 class="text-4xl font-bold mb-4 text-gray-800 border-b pb-2">"Marek Vrbka"</h1>
                <div class="space-y-6">
                    <p class="text-xl text-blue-600 font-medium">"Software Developer based in Brno"</p>

                    <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-100">
                        <h2 class="text-2xl font-semibold mb-3 text-gray-800 flex items-center">
                            <span class="mr-2">"About Me"</span>
                            <div class="h-1 flex-grow bg-blue-100 rounded"></div>
                        </h2>
                        <p class="text-gray-600 leading-relaxed">
                            "I'm a passionate software developer with 2 years of professional experience in systems programming."
                        </p>
                    </div>

                    <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-100">
                        <h2 class="text-2xl font-semibold mb-4 text-gray-800 flex items-center">
                            <span class="mr-2">"Contact"</span>
                            <div class="h-1 flex-grow bg-blue-100 rounded"></div>
                        </h2>
                        <ul class="space-y-3">
                            <li class="flex items-center text-gray-600 hover:text-blue-600 transition-colors">
                                <div class="w-24 font-medium text-gray-800">"Email:"</div>
                                <a href="mailto:marekvrbka@gmai.com">"marekvrbka@gmail.com"</a>
                            </li>
                            <li class="flex items-center text-gray-600 hover:text-blue-600 transition-colors">
                                <div class="w-24 font-medium text-gray-800">"GitHub:"</div>
                                <a href="https://github.com/Marcuss2">"github.com/Marcuss2"</a>
                            </li>
                            <li class="flex items-center text-gray-600 hover:text-blue-600 transition-colors">
                                <div class="w-24 font-medium text-gray-800">"LinkedIn:"</div>
                                <a href="https://linkedin.com/in/marek-vrbka/">"linkedin.com/in/marek-vrbka/"</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

// CV Component
#[component]
fn CV() -> impl IntoView {
    view! {
        <div class="p-8 bg-gradient-to-br from-gray-50 to-white rounded-lg shadow-sm">
            <div class="max-w-4xl mx-auto">
                <h1 class="text-4xl font-bold mb-6 text-gray-800 border-b pb-2">"Curriculum Vitae"</h1>

                <div class="space-y-8">
                    <section class="bg-white p-6 rounded-lg shadow-sm border border-gray-100">
                        <h2 class="text-2xl font-semibold mb-4 text-gray-800 flex items-center">
                            <span class="mr-2">"Work Experience"</span>
                            <div class="h-1 flex-grow bg-blue-100 rounded"></div>
                        </h2>
                        <div class="space-y-6">
                            <div>
                                <div class="flex justify-between items-center mb-2">
                                    <h3 class="text-xl font-medium text-gray-800">"Software Developer"</h3>
                                    <span class="text-blue-600 font-medium">"2023 - Present"</span>
                                </div>
                                <p class="text-gray-600 mb-2">"Codasip s.r.o."</p>
                                <ul class="list-disc ml-6 space-y-1 text-gray-600">
                                    <li>"C, C++, Python"</li>
                                    <li>"Maintained and developed OpenOCD"</li>
                                    <li>"Developed tooling for generating SystemVerilog"</li>
                                </ul>
                            </div>
                            <div>
                                <div class="flex justify-between items-center mb-2">
                                    <h3 class="text-xl font-medium text-gray-800">"Teaching Assisant"</h3>
                                    <span class="text-blue-600 font-medium">"2022 - Present"</span>
                                </div>
                                <p class="text-gray-600 mb-2">"Brno Faculty of Informatics"</p>
                                <ul class="list-disc ml-6 space-y-1 text-gray-600">
                                    <li>"Rust, Python, C"</li>
                                    <li>"Teaching the Rust Programming Language and embedded systems"</li>
                                    <li>"Managing small groups of students"</li>
                                    <li>"Wrote automated homework testing systems"</li>
                                </ul>
                            </div>

                        </div>
                    </section>

                    <section class="bg-white p-6 rounded-lg shadow-sm border border-gray-100">
                        <h2 class="text-2xl font-semibold mb-4 text-gray-800 flex items-center">
                            <span class="mr-2">"Education"</span>
                            <div class="h-1 flex-grow bg-blue-100 rounded"></div>
                        </h2>
                        <div class="space-y-6">
                            <div>
                                <h3 class="text-xl font-medium text-gray-800 mb-1">"Applied Systems"</h3>
                                <h3 class="text-xl font-medium text-gray-800 mb-1">"Bachelor's Degree"</h3>
                                <p class="text-gray-600">"Brno Faculty of Informatics • 2017 - 2020"</p>
                                <p class="text-gray-600">"Thesis: "<a href="https://is.muni.cz/th/ego6j/">"Encrypted Network Traffic Classification and Analysis"</a></p>
                            </div>
                            <div>
                                <h3 class="text-xl font-medium text-gray-800 mb-1">"Computer Systems, Communication and Security: Hardware Systems"</h3>
                                <h3 class="text-xl font-medium text-gray-800 mb-1">"Master's Degree"</h3>
                                <p class="text-gray-600">"Brno Faculty of Informatics • 2020 - 2023"</p>
                                <p class="text-gray-600">"Thesis: "<a href="https://is.muni.cz/th/m6ad7/">"Testing of Student Assignments in Embedded Course"</a></p>
                            </div>
                        </div>

                    </section>

                    <section class="bg-white p-6 rounded-lg shadow-sm border border-gray-100">
                        <h2 class="text-2xl font-semibold mb-4 text-gray-800 flex items-center">
                            <span class="mr-2">"Skills"</span>
                            <div class="h-1 flex-grow bg-blue-100 rounded"></div>
                        </h2>
                        <div class="grid md:grid-cols-2 gap-6">
                            <div>
                                <h3 class="text-xl font-medium mb-3 text-gray-800">"Languages"</h3>
                                <ul class="space-y-2">
                                    <li class="flex items-center space-x-2">
                                        <div class="w-2 h-2 bg-blue-500 rounded-full"></div>
                                        <span class="text-gray-600">"Rust"</span>
                                    </li>
                                    <li class="flex items-center space-x-2">
                                        <div class="w-2 h-2 bg-blue-500 rounded-full"></div>
                                        <span class="text-gray-600">"C/C++"</span>
                                    </li>
                                    <li class="flex items-center space-x-2">
                                        <div class="w-2 h-2 bg-blue-500 rounded-full"></div>
                                        <span class="text-gray-600">"Python"</span>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        </div>
    }
}

// Main App Component
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="max-w-5xl mr-0">
                <nav class="border-b">
                    <ul class="flex space-x-6 p-4">
                        <li>
                            <A
                                href="/"
                            >
                                "Basic Info"
                            </A>
                        </li>
                        <li>
                            <A
                                href="/cv"
                            >
                                "CV"
                            </A>
                        </li>
                    </ul>
                </nav>

                <Routes fallback=|| "This page could not be found.">
                    <Route path=path!("/") view=|| view! { <BasicInfo/> }/>
                    <Route path=path!("/cv") view=|| view! { <CV/> }/>
                </Routes>
            </main>
        </Router>
    }
}