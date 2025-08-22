use dioxus::prelude::*;
use crate::{components::{SettingsCategory, SettingsItem}, Route};

#[component]
pub fn SettingsSidebar() -> Element {
    rsx! {
        div { 
            class: "min-h-screen w-100 bg-gray-700/20 mx-5 my-5 flex flex-col rounded-md gap-1 items-center",
            h1 { class: "self-center text-2xl mt-2",
                "Settings"
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