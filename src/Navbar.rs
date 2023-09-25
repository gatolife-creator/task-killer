use dioxus::{ prelude::* };

pub fn navbar(cx: Scope) -> Element {
    cx.render(
        rsx! {
            div {
                class: "navbar bg-base-100",
                div {
                    class: "flex-none",
                    button {
                        class: "btn btn-square btn-ghost",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            class: "inline-block w-5 h-5 stroke-current",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 6h16M4 12h16M4 18h16",
                            }
                        }
                    }
                },
                div {
                    class: "flex-1",
                    a {
                        class: "btn btn-ghost normal-case text-xl",
                        "daisyUI",
                    }
                },
                div {
                    class: "flex-none",
                    a {
                        class: "btn btn-square btn-ghost normal-case text-xl"
                    }
                },
                div {
                    class: "flex-none",
                    button {
                        class: "btn btn-square btn-ghost",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            class: "inline-block w-5 h-5 stroke-current",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z",
                            }
                        }
                    }
                }
            }
        }
    )
}
