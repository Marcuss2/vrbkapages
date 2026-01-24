use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
fn BasicInfo() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <div class="space-y-2">
                <h2 class="text-3xl font-bold text-gray-900 dark:text-white">"Marek Vrbka"</h2>
                <p class="text-lg text-gray-600 dark:text-gray-400">"Software Developer based in Brno"</p>
            </div>

            <div class="h-px bg-gray-200 dark:bg-gray-700 my-4"></div>

            <h2 class="text-2xl font-semibold text-gray-800 dark:text-gray-200">"Contact"</h2>

            <div class="flex flex-col md:flex-row gap-6">
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 flex-1">
                    <div class="mb-4">
                        <h3 class="text-lg font-medium text-gray-900 dark:text-white m-0">"Email"</h3>
                    </div>
                    <div class="pt-4 border-t border-gray-100 dark:border-gray-700">
                        <a href="mailto:marekvrbka@gmail.com"
                           class="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 hover:underline">
                            "marekvrbka@gmail.com"
                        </a>
                    </div>
                </div>

                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 flex-1">
                    <div class="mb-4">
                        <h3 class="text-lg font-medium text-gray-900 dark:text-white m-0">"GitHub"</h3>
                    </div>
                    <div class="pt-4 border-t border-gray-100 dark:border-gray-700">
                        <a href="https://github.com/Marcuss2"
                           class="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 hover:underline">
                            "github.com/Marcuss2"
                        </a>
                    </div>
                </div>

                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 flex-1">
                    <div class="mb-4">
                        <h3 class="text-lg font-medium text-gray-900 dark:text-white m-0">"LinkedIn"</h3>
                    </div>
                    <div class="pt-4 border-t border-gray-100 dark:border-gray-700">
                        <a href="https://linkedin.com/in/marek-vrbka/"
                           class="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 hover:underline">
                            "linkedin.com/in/marek-vrbka/"
                        </a>
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
        <div class="flex flex-col gap-6">
            <div class="space-y-2">
                <h2 class="text-3xl font-bold text-gray-900 dark:text-white">"Curriculum Vitae"</h2>
            </div>

            <div class="h-px bg-gray-200 dark:bg-gray-700 my-4"></div>

            <h2 class="text-2xl font-semibold text-gray-800 dark:text-gray-200">"Work Experience"</h2>

            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
                <div class="mb-4">
                    <h3 class="text-xl font-medium text-gray-900 dark:text-white m-0">"Brno Faculty of Informatics"</h3>
                    <p class="text-gray-600 dark:text-gray-400 mt-1">"Teaching Assistant"</p>
                </div>
                <div class="py-4 border-t border-gray-100 dark:border-gray-700">
                    <ul class="list-disc pl-5 space-y-2 text-gray-700 dark:text-gray-300">
                        <li>"Rust, Python, C"</li>
                        <li>"Teaching the Rust Programming Language and embedded systems"</li>
                        <li>"Managing small groups of students"</li>
                        <li>"Wrote automated homework testing systems"</li>
                    </ul>
                </div>
                <div class="pt-4 border-t border-gray-100 dark:border-gray-700 text-gray-500 dark:text-gray-400">
                    "January 2022 - Present (Every Autumn)"
                </div>
            </div>

            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
                <div class="mb-4">
                    <h3 class="text-xl font-medium text-gray-900 dark:text-white m-0">"Codasip s.r.o."</h3>
                    <p class="text-gray-600 dark:text-gray-400 mt-1">"Software Developer"</p>
                </div>
                <div class="py-4 border-t border-gray-100 dark:border-gray-700">
                    <ul class="list-disc pl-5 space-y-2 text-gray-700 dark:text-gray-300">
                        <li>"C, C++, Python"</li>
                        <li>"Maintained and developed OpenOCD"</li>
                        <li>"Developed tooling for generating SystemVerilog"</li>
                    </ul>
                </div>
                <div class="pt-4 border-t border-gray-100 dark:border-gray-700 text-gray-500 dark:text-gray-400">
                    "March 2023 - December 2025"
                </div>
            </div>

            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
                <div class="mb-4">
                    <h3 class="text-xl font-medium text-gray-900 dark:text-white m-0">"Innovatris s.r.o."</h3>
                    <p class="text-gray-600 dark:text-gray-400 mt-1">"Software Developer"</p>
                </div>
                <div class="py-4 border-t border-gray-100 dark:border-gray-700">
                    <ul class="list-disc pl-5 space-y-2 text-gray-700 dark:text-gray-300">
                        <li>"Rust, C++"</li>
                        <li>"Rewrite of complex biometrics software toolkit from C++ to Rust"</li>
                        <li>"Rust binding management into C++, Kotlin, Swift, Python"</li>
                    </ul>
                </div>
                <div class="pt-4 border-t border-gray-100 dark:border-gray-700 text-gray-500 dark:text-gray-400">
                    "January 2026 - Present"
                </div>
            </div>

            <div class="h-px bg-gray-200 dark:bg-gray-700 my-4"></div>

            <h2 class="text-2xl font-semibold text-gray-800 dark:text-gray-200">"Education"</h2>

            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
                <div class="mb-4">
                    <h3 class="text-xl font-medium text-gray-900 dark:text-white m-0">"Computer Systems, Communication and Security: Hardware Systems"</h3>
                    <p class="text-gray-600 dark:text-gray-400 mt-1">"Master's Degree"</p>
                </div>
                <div class="py-4 border-t border-gray-100 dark:border-gray-700">
                    <ul class="list-disc pl-5 space-y-2 text-gray-700 dark:text-gray-300">
                        <li>
                            "Thesis: "
                            <a href="https://is.muni.cz/th/m6ad7/"
                               class="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 hover:underline">
                                "Testing of Student Assignments in Embedded Course"
                            </a>
                        </li>
                    </ul>
                </div>
                <div class="pt-4 border-t border-gray-100 dark:border-gray-700 text-gray-500 dark:text-gray-400">
                    "Brno Faculty of Informatics • 2020 - 2023"
                </div>
            </div>

            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6">
                <div class="mb-4">
                    <h3 class="text-xl font-medium text-gray-900 dark:text-white m-0">"Applied Systems"</h3>
                    <p class="text-gray-600 dark:text-gray-400 mt-1">"Bachelor's Degree"</p>
                </div>
                <div class="py-4 border-t border-gray-100 dark:border-gray-700">
                    <ul class="list-disc pl-5 space-y-2 text-gray-700 dark:text-gray-300">
                        <li>
                            "Thesis: "
                            <a href="https://is.muni.cz/th/ego6j/"
                               class="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 hover:underline">
                                "Encrypted Network Traffic Classification and Analysis"
                            </a>
                        </li>
                    </ul>
                </div>
                <div class="pt-4 border-t border-gray-100 dark:border-gray-700 text-gray-500 dark:text-gray-400">
                    "Brno Faculty of Informatics • 2017 - 2020"
                </div>
            </div>
        </div>
    }
}

// Main App Component
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100 transition-colors duration-200">
            <Router>
                <div class="flex flex-col min-h-screen">
                    <header class="sticky top-0 z-50 bg-white dark:bg-gray-800 shadow-sm">
                        <div class="container mx-auto px-4 py-4">
                            <div class="flex justify-between items-center">
                                <nav class="flex space-x-4">
                                    <a href="/"
                                       class="px-3 py-2 rounded-md text-sm font-medium hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">
                                        "Basic Info"
                                    </a>
                                    <a href="/cv"
                                       class="px-3 py-2 rounded-md text-sm font-medium hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">
                                        "CV"
                                    </a>
                                </nav>
                                <div class="flex items-center space-x-4">
                                    <a href="https://github.com/Marcuss2/vrbkapages"
                                       class="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-md text-sm font-medium transition-colors">
                                        "Source"
                                    </a>
                                </div>
                            </div>
                        </div>
                        <div class="h-px bg-gray-200 dark:bg-gray-700"></div>
                    </header>

                    <main class="flex-1 container mx-auto px-4 py-8 md:px-10 lg:px-20">
                        <Routes fallback=|| "This page could not be found.">
                            <Route path=path!("/") view=|| view! { <BasicInfo/> }/>
                            <Route path=path!("/cv") view=|| view! { <CV/> }/>
                        </Routes>
                    </main>
                </div>
            </Router>
        </div>
    }
}
