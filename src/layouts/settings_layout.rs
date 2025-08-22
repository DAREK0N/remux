use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn SettingsLayout() -> Element {
    rsx! {
        div { Outlet::<Route> {} }
    }
}