use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::menu::MenuContext;
use crate::icons::MdMoreVert;
use crate::{Button, ButtonSize, ButtonVariant, Icon, IconShape, MenuList, PortalOut, use_portal};

#[component]
pub fn DropdownMenu<T: IconShape + Clone + PartialEq + 'static>(
    icon: T,
    #[props(default)] class: String,
    #[props(default)] variant: ButtonVariant,
    children: Element,
) -> Element {
    /*

       State

    */

    let mut open = use_signal(bool::default);

    /*

       Refs

    */

    let mut menu_host: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    /*

       Contexts

    */

    use_context_provider(|| MenuContext { open });

    /*

       Callbacks

    */

    let handle_toggle = use_callback(move |_| {
        open.with_mut(|open| *open = !*open);
    });

    /*

        Effects

    */

    use_effect(move || {
        if !open() {
            return;
        }

        let Some(menu_host) = menu_host() else {
            return;
        };

        spawn(async move {
            _ = menu_host.set_focus(true).await;
        });
    });

    /*

       Elements

    */

    rsx! {
        div {
            class: "relative outline-none",
            tabindex: "0",
            onmounted: move |e| menu_host.set(Some(e.data())),
            onblur: move |_| open.set(false),
            Button {
                class,
                size: ButtonSize::IconLG,
                variant,
                onclick: handle_toggle,
                Icon { icon }
            }
            if *open.read() {
                MenuList { class: "z-50 absolute top-full", {children} }
            }
        }
    }
}
