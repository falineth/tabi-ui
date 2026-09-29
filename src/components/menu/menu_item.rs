use dioxus::prelude::*;

use crate::components::menu::{MenuContext, MenuListContext};
use crate::hooks::use_unique_id;
use crate::{Icon, IconShape};

#[component]
pub fn MenuItem<T: IconShape + Clone + PartialEq + 'static>(
    icon: T,
    #[props(default)] label: String,
    #[props(default)] onclick: EventHandler<Event<MouseData>>,
) -> Element {
    let item_id = use_unique_id();

    let mut menu_context: MenuContext = use_context();

    let mut list_context = use_context::<MenuListContext>();

    let highlighted = use_memo(move || {
        list_context.active_item.read().as_deref() == Some(item_id.read().as_str())
    });

    let handle_click = use_callback(move |event| {
        menu_context.open.set(false);
        onclick.call(event);
    });

    rsx! {
        div {
            id: item_id,
            class: "flex px-3 py-1 gap-2 cursor-default select-none",
            class: "border border-transparent rounded-sm",
            class: "data-highlighted:bg-window-decorationfocus/20 data-highlighted:border-window-decorationfocus",
            "data-highlighted": if highlighted() { true },
            onpointerenter: move |_| list_context.active_item.set(Some(item_id())),
            onclick: handle_click,
            Icon { icon }
            div { class: "whitespace-nowrap", "{label}" }
        }
    }
}
