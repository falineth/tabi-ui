use dioxus::prelude::*;
use dioxus_free_icons::IconShape;

use crate::components::menu::MenuContext;
use crate::icons::MdMoreVert;
use crate::{Button, ButtonSize, ButtonVariant, Icon, MenuList, PortalOut, use_portal};

#[component]
pub fn DropdownMenu<T: IconShape + Clone + PartialEq + 'static>(
    icon: T,
    #[props(default)] class: String,
    #[props(default)] variant: ButtonVariant,
    children: Element,
) -> Element {
    let mut open = use_signal(bool::default);

    use_context_provider(|| MenuContext { open });

    let handle_toggle = use_callback(move |_| {
        open.with_mut(|open| *open = !*open);
    });

    let handle_menu_shown = use_callback(async move |event: Event<MountedData>| {
        _ = event.data.set_focus(true).await;
    });

    rsx! {
        div { class: "relative",
            Button {
                class,
                size: ButtonSize::IconLG,
                variant,
                onclick: handle_toggle,
                Icon { icon }
            }
            if *open.read() {
                MenuList {
                    class: "z-50 absolute top-full",
                    onmounted: move |event| handle_menu_shown.call(event),
                    onblur: move |_| open.set(false),
                    {children}
                }
            }
        }
    }
}
