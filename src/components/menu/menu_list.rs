use dioxus::prelude::*;

#[component]
pub fn MenuList(
    #[props(default)] class: String,
    children: Element,
    #[props(default)] onmounted: Callback<Event<MountedData>>,
    #[props(default)] onblur: Callback<Event<FocusData>>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col",
            class: "rounded-sm p-1 border select-none outline-none",
            class: "bg-window-backgroundnormal text-window-foregroundnormal border-window-foregroundnormal/20",
            class: "{class}",
            tabindex: "0",
            onmounted,
            onblur,
            {children}
        }
    }
}
