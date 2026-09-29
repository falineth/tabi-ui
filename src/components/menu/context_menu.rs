use dioxus::prelude::*;

use crate::components::menu::MenuContext;
use crate::{ContextMenuContext, MenuList, PortalId, PortalIn, PortalOut, use_portal};

#[component]
pub fn ContextMenu(children: Element) -> Element {
    let context_menu_context = use_context::<ContextMenuContext>();

    let mut menu_context = use_context::<MenuContext>();

    let handle_menu_mounted = use_callback(async move |event: Event<MountedData>| {
        _ = event.data.set_focus(true).await;
    });

    let (left, top) = *context_menu_context.position.read();

    rsx! {
        PortalIn { portal: context_menu_context.portal,
            if *menu_context.open.read() {
                div {
                    class: "fixed z-50 outline-none",
                    tabindex: "0",
                    style: "left: {left}px; top: {top}px;",
                    onmounted: move |event| handle_menu_mounted.call(event),
                    onblur: move |_| menu_context.open.set(false),
                    onclick: move |_| menu_context.open.set(false),
                    oncontextmenu: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                    },
                    MenuList { class: "min-w-48 shadow-md", {children} }
                }
            }
        }
    }
}
