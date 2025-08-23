use dioxus::prelude::*;
use crate::{components::{SettingsCategory, SettingsItem}, hooks, Route};

#[component]
pub fn SettingsSidebar() -> Element {
    let mut server = hooks::use_server();
    let mut config = hooks::use_server_config();

    rsx! {
        div { 
            class: "min-h-screen w-80 min-w-80 bg-gray-700/20 mx-5 my-5 flex flex-col rounded-md gap-1 items-center",
            div {
                class: "flex flex-row content-between mt-2 mb-4 gap-12 items-center",
                Link {
                    class: "cursor-pointer select-none",
                    to: Route::Home {  },
                    "Back"
                }
                h1 { class: "text-2xl select-none",
                    "Settings"
                }
                a {
                    onclick: {
                        move |_| {
                            config.set(None);
                            server.set(None);
                        }
                    },
                    class: "cursor-pointer select-none text-red-900/50",
                    href: "/login",
                    "Logout"
                    }
            }
            div {
                class: "w-[95%]",
                SettingsCategory{
                    title: "User",

                    SettingsItem{
                        title: "Profile",
                        to: Route::SettingsCatalogView {},
                    },

                    SettingsItem{
                        title: "Quick Connect",
                        to: Route::SettingsCatalogView {},
                    },

                    SettingsItem{
                        title: "Display",
                        to: Route::SettingsCatalogView {},
                    },

                    SettingsItem{
                        title: "Home",
                        to: Route::SettingsCatalogView {},
                    },

                    SettingsItem{
                        title: "Playback",
                        to: Route::SettingsCatalogView {},
                    },

                    SettingsItem{
                        title: "Subtitles",
                        to: Route::SettingsCatalogView {},
                    },

                    SettingsItem{
                        title: "Controls",
                        to: Route::SettingsCatalogView {},
                    },
                }
            }
         }
    }
}