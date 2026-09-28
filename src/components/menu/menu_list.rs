use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct MenuListContext {
    pub active_item: Signal<Option<String>>,
}

#[component]
pub fn MenuList(#[props(default)] class: String, children: Element) -> Element {
    let active_item = use_signal(Option::<String>::default);

    use_context_provider(|| MenuListContext { active_item });

    rsx! {
        div {
            class: "flex flex-col",
            class: "rounded-sm p-1 border select-none outline-none",
            class: "bg-window-backgroundnormal text-window-foregroundnormal border-window-foregroundnormal/20",
            class: "{class}",
            {children}
        }
    }
}
