use dioxus::prelude::*;
use crate::{components::SettingsSidebar, Route};

#[component]
pub fn SettingsLayout() -> Element {
    rsx! {
        div {
            class: "flex flex-row",
            SettingsSidebar {}
            div { Outlet::<Route> {} }
        }
    }
}