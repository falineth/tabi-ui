use dioxus::prelude::*;

use crate::components::menu::MenuListContext;
use crate::components::{Icon, IconShape, MenuList};
use crate::hooks::use_unique_id;
use crate::icons::MdChevronRight;

#[component]
pub fn SubMenu<T: IconShape + Clone + PartialEq + 'static>(
    icon: T,
    title: String,
    children: Element,
) -> Element {
    let item_id = use_unique_id();

    let mut list_context = use_context::<MenuListContext>();

    let highlighted = use_memo(move || {
        list_context.active_item.read().as_deref() == Some(item_id.read().as_str())
    });

    rsx! {
        div {
            id: item_id,
            class: "relative",
            onpointerenter: move |_| list_context.active_item.set(Some(item_id())),
            div {
                class: "flex items-center justify-between px-3 py-1 gap-2 cursor-default",
                class: "border border-transparent rounded-sm",
                class: "data-highlighted:bg-window-decorationfocus/20 data-highlighted:border-window-decorationfocus",
                "data-highlighted": if highlighted() { true },
                Icon { icon }
                span { class: "whitespace-nowrap", "{title}" }
                Icon { icon: MdChevronRight }
            }

            if highlighted() {
                MenuList { class: "absolute z-10 left-[calc(100%+8px)] -top-1 min-w-52 shadow-md",
                    {children}
                }
            }
        }
    }
}
