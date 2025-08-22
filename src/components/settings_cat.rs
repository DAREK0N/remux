use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct SettCatProps {
    children: Element,
    title: String,
    // icon: Element,
}


#[component]
pub fn SettingsCategory(props: SettCatProps) -> Element {
    rsx! {
        div{
            class: "select-none h-fit w-full flex flex-col gap-1",
            // Make colapseble?
            h1{
                class: "text-xl",
                {props.title}
            }
            div{
                class: "flex flex-col gap-0.5",
                {props.children}
            }
        }
    }
}