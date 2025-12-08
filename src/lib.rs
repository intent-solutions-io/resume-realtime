use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/app.css"/>
        <Title text="Real-Time Resume"/>
        <Router>
            <main class="container mx-auto px-4 py-8">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let increment = move |_| {
        set_count.update(|n| *n += 1);
    };

    view! {
        <div class="flex flex-col items-center justify-center min-h-[80vh] space-y-8">
            <h1 class="text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-600">
                "Real-Time Resume"
            </h1>

            <p class="text-gray-400 text-sm">
                "Intent Solutions IO"
            </p>

            <div class="bg-gray-800 rounded-2xl p-8 shadow-2xl border border-gray-700">
                <div class="text-center space-y-6">
                    <p class="text-gray-400 text-sm uppercase tracking-wider">
                        "Counter Value"
                    </p>

                    <div class="text-7xl font-bold text-white tabular-nums">
                        {move || count.get()}
                    </div>

                    <button
                        on:click=increment
                        class="px-8 py-4 bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white font-semibold rounded-xl text-xl transition-all duration-200 transform hover:scale-105 active:scale-95 shadow-lg"
                    >
                        "+1"
                    </button>
                </div>
            </div>

            <div class="text-gray-500 text-sm space-y-1 text-center">
                <p>"SSR + Hydration Demo"</p>
                <p class="text-xs">"Refresh the page to see SSR in action"</p>
            </div>
        </div>
    }
}

/// Shell for SSR rendering
#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <link rel="stylesheet" href="/app.css"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
