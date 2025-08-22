use dioxus::prelude::*;
use crate::Route;

#[derive(Clone, Props, PartialEq)]
pub struct SettItemProps {
    title: &'static str,
    to: Route,
    // icon: Element,
}

#[component]
pub fn SettingsItem(props: SettItemProps) -> Element {
    let content = rsx! {
        div { 
            class: "group/{props.title} w-full flex items-center justify-between px-3 py-2 bg-zinc-900/0 rounded-lg hover:bg-zinc-800 transition",
            div{
                class: "flex items-center space-x-3",
                /*span {
                    class: "text-xl",
                    {props.icon},
                }*/
                span {
                    class: "group/{props.title} text-white",
                    {props.title}
                }
            }
        }
    };
    rsx!(
        Link{ to: {props.to}, class: "block", {content}}
    )
}