use dioxus::prelude::*;
use crate::{hooks, Route};

#[component]
pub fn DevNav() -> Element {
    let mut server = hooks::use_server();
    let mut config = hooks::use_server_config();

    rsx! {
        div { 
            class: "z-[1000] select-none group/devnav fixed bottom-6 right-6 h-8 max-w-8 min-w-8 w-fit hover:max-w-fit rounded-full bg-cyan-900/90 hover:bg-cyan-800/80 transition-all transition- overflow-hidden",
            div {
                class: "group/devnav select-none w-full h-full ml-2 transition-all flex flex-row-reverse text-white/40 hover:text-white gap-3 items-center",
                span { 
                    class: "font-bold mr-2 select-none group-hover/devnav:mr-4",
                    "Nav"
                }
                a {
                    onclick: {
                        move |_| {
                            config.set(None);
                            server.set(None);
                        }
                    },
                    class: "cursor-pointer select-none text-red-700",
                    href: "/login",
                    "Logout"
                    }
                Link {
                    class: "select-none",
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    class: "select-none",
                    to: Route::Settings {},
                    "Settings"
                }
            }
         }
    }
}