// TODO: ensure focus cannot leave dialog

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_free_icons::icons::md_navigation_icons::MdClose;

use crate::components::{
    Button, ButtonSize, ButtonVariant, Icon, PortalId, PortalIn, PortalOut, use_portal,
};
use crate::hooks::{use_controlled, use_window_size};
use crate::icons::MdCircle;

#[derive(Clone)]
pub struct DialogContext {
    pub portal: PortalId,
    pub open: Memo<bool>,
    pub set_open: Callback<bool>,
}

pub fn use_dialog_context() -> DialogContext {
    use_context::<DialogContext>()
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    #[props(default)]
    open: ReadSignal<Option<bool>>,
    #[props(default)]
    default_open: bool,
    #[props(default)]
    on_open_change: Callback<bool>,
    children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let DialogProps {
        open,
        default_open,
        on_open_change,
        children,
    } = props;

    let (open, set_open) = use_controlled::<bool>(open, default_open, on_open_change);

    let portal = use_portal();

    use_context_provider(|| DialogContext {
        portal,
        open,
        set_open,
    });

    rsx! {
        div { {children} }

        PortalOut { portal }
    }
}

#[component]
pub fn DialogOverlay(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    #[props(default)] allow_dismiss: bool,
    children: Element,
) -> Element {
    let dialog_context = use_dialog_context();

    let handle_dismiss_dialog = move |_| {
        if allow_dismiss {
            dialog_context.set_open.call(false);
        }
    };

    rsx! {
        div {
            "data-slot": "dialog-overlay",
            "data-state": if dialog_context.open.read().eq(&true) { "open" } else { "closed" },
            class: "hidden data-open:block data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/30 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50",
            class: "{class}",
            onclick: handle_dismiss_dialog,
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogContent(
    #[props(default)] class: String,
    #[props(default)] allow_dismiss: bool,
    #[props(default = true)] full_width: bool,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    let dialog_context = use_dialog_context();

    rsx! {
        PortalIn { portal: dialog_context.portal,
            DialogOverlay { allow_dismiss }
            div {
                "data-slot": "dialog-content",
                "data-state": if dialog_context.open.read().eq(&true) { "open" } else { "closed" },
                class: "hidden data-open:grid data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 max-w-[calc(100%-2rem)] gap-4 rounded-sm p-4 text-xs/relaxed ring-1 border duration-100 sm:max-w-sm md:max-w-md lg:max-w-lg xl:max-w-xl 2xl:max-w-2xl fixed top-1/2 left-1/2 z-50 -translate-x-1/2 -translate-y-1/2 outline-none",
                class: "ring-view-foregroundnormal/5 border-view-foregroundnormal/20 bg-view-backgroundnormal",
                class: "{class}",
                class: if full_width { "w-full" },
                ..rest,
                {children}
                DialogClose {
                    "data-slot": "dialog-close",
                    class: "absolute top-2 right-2",
                    Button {
                        variant: ButtonVariant::GhostView,
                        size: ButtonSize::IconSM,
                        Icon { icon: MdClose }
                        span { class: "sr-only", "Close" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DialogHeader(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-header",
            class: "gap-1 flex flex-col",
            class: "text-window-foregroundnormal",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogTitle(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-title",
            class: "text-sm font-medium",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogDescription(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-description",
            class: "text-view-foregroundnormal *:[a]:hover:text-view-foregroundlink text-xs/relaxed *:[a]:underline *:[a]:underline-offset-3",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogFooter(
    #[props(default)] show_close_button: bool,
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-footer",
            class: "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
            class: "{class}",
            ..rest,
            {children}
            if show_close_button {
                Button { variant: ButtonVariant::Outline, "Close" }
            }
        }
    }
}

#[component]
pub fn DialogClose(
    #[props(default)] show_close_button: bool,
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    let dialog_context = use_dialog_context();

    let handle_open_dialog = move |_| {
        dialog_context.set_open.call(false);
    };

    rsx! {
        div {
            "data-slot": "dialog-close",
            class: "{class}",
            onclick: handle_open_dialog,
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogTrigger(
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] show_close_button: ReadSignal<bool>,
    #[props(default)] class: ReadSignal<String>,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    let dialog_context = use_dialog_context();

    let handle_open_dialog = move |_| {
        if !*disabled.read() {
            dialog_context.set_open.call(true);
        }
    };

    rsx! {
        div { class: "{class}", onclick: handle_open_dialog, ..rest, {children} }
    }
}
