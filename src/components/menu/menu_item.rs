use dioxus::prelude::*;
use dioxus_free_icons::IconShape;

use crate::Icon;
use crate::components::menu::MenuContext;

#[component]
pub fn MenuItem<T: IconShape + Clone + PartialEq + 'static>(
    icon: T,
    #[props(default)] label: String,
    #[props(default)] onclick: EventHandler<Event<MouseData>>,
) -> Element {
    let mut menu_context: MenuContext = use_context();

    let handle_click = use_callback(move |event| {
        menu_context.open.set(false);
        onclick.call(event);
    });

    rsx! {
        div {
            class: "flex px-3 py-1 gap-2 cursor-default select-none",
            class: "border border-transparent rounded-sm",
            class: "hover:bg-window-decorationfocus/20 hover:border-window-decorationfocus",
            onclick: handle_click,
            Icon { icon }
            div { "{label}" }
        }
    }
}
