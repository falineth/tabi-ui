use dioxus::prelude::*;

use crate::components::menu::MenuContext;
use crate::{PortalId, PortalOut, use_portal};

#[derive(Clone, Copy)]
pub struct ContextMenuContext {
    pub position: Signal<(f64, f64)>,
    pub portal: PortalId,
}

#[component]
pub fn ContextMenuArea(children: Element, #[props(default)] class: String) -> Element {
    let mut open = use_signal(bool::default);

    let mut position = use_signal(|| (0.0, 0.0));

    let portal = use_portal();

    use_context_provider(|| MenuContext { open });

    use_context_provider(|| ContextMenuContext { position, portal });

    rsx! {
        div {
            class: "{class}",
            oncontextmenu: move |event| {
                event.prevent_default();

                let coordinates = event.client_coordinates();
                position.set((coordinates.x, coordinates.y));
                open.set(true);
            },
            {children}
            PortalOut { portal }
        }
    }
}
