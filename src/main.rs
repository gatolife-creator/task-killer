#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{ prelude::* };
mod Navbar;
mod filter;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let name = use_state(cx, || "bob".to_string());

    fn on_input(value: &str) {
        println!("input: {}", value);
    }

    return cx.render(
        rsx! {
            Navbar::navbar {}
            link { rel: "stylesheet", href: "../public/tailwind.css" },
            div {
                class: "container mx-auto text-center",
                input {
                    class: "input input-bordered w-full max-w-xs",
                    value: "{name}",
                    oninput: move |evt| {
                        name.set(evt.value.clone());
                        on_input(&evt.value)
                    }
                },
                p {
                    class: "text-red-600",
                    "Hello, {name}!",
                },
                button {
                    class: "btn btn-primary",
                    "button"
                },
            }
        }
    );
}
