use dioxus::prelude::*;

use crate::{DialogContext, DialogOverlay, ProgressIndicator, Spinner, use_portal};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BusyStatus {
    pub title: &'static str,
    pub message: String,
    pub percentage: Option<f64>,
}

#[component]
pub fn BusyOverlay(busy_status: Signal<Option<BusyStatus>, SyncStorage>) -> Element {
    /*

       State

    */

    let open_memo = use_memo(move || busy_status.read().is_some());

    /*

       Callbacks

    */

    let set_open = use_callback(move |_: bool| {});

    /*

       Hooks

    */

    let portal = use_portal();

    let dialog_context = use_context_provider(|| DialogContext {
        portal,
        open: open_memo,
        set_open,
    });

    rsx! {
        DialogOverlay { allow_dismiss: false }
        div {
            "data-slot": "dialog-content",
            "data-state": if *dialog_context.open.read() { "open" } else { "closed" },
            class: "hidden max-w-[calc(100%-2rem)] rounded-sm gap-2 p-4 text-xs/relaxed ring-1 border duration-100 outline-none select-none",
            class: "fixed top-1/2 left-1/2 z-50 -translate-x-1/2 -translate-y-1/2",
            class: "sm:max-w-sm md:max-w-md lg:max-w-lg xl:max-w-xl 2xl:max-w-2xl",
            class: "data-open:grid data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95",
            class: "data-closed:animate-out data-closed:fade-out-0  data-closed:zoom-out-95",
            class: "ring-view-foregroundnormal/5 border-view-foregroundnormal/20 bg-view-backgroundnormal",
            if let Some(progress) = busy_status.read().as_ref() {
                h1 { class: "text-xl", "{progress.title}" }
                div { "{progress.message}" }
                match progress.percentage {
                    Some(percentage) => {
                        #[allow(clippy::cast_possible_truncation)]
                        let progress = percentage as f32;

                        rsx! {
                            div { class: "justify-self-center",
                                ProgressIndicator { progress }
                            }
                        }
                    }
                    None => rsx! {
                        div { class: "justify-self-center",
                            Spinner { class: "scale-200" }
                        }
                    },
                }
            }
        }
    }
}
