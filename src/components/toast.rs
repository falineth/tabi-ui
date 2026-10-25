use dioxus::prelude::*;

use crate::sleep;

const DEFAULT_TOAST_DURATION_MILLISECONDS: i32 = 5000;
const TOAST_EXIT_DURATION_MILLISECONDS: i32 = 200;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Error,
}

impl std::fmt::Display for ToastVariant {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant = match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Error => "error",
        };

        formatter.write_str(variant)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ToastMessage {
    id: u64,
    message: String,
    variant: ToastVariant,
}

#[derive(Clone, Copy)]
pub struct ToastContext {
    messages: Signal<Vec<ToastMessage>>,
    next_id: Signal<u64>,
}

impl ToastContext {
    pub fn show(&mut self, message: impl Into<String>, variant: ToastVariant) {
        let id = self.next_id.with_mut(|next_id| {
            let id = *next_id;
            *next_id = next_id.wrapping_add(1);
            id
        });

        self.messages.write().push(ToastMessage {
            id,
            message: message.into(),
            variant,
        });
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.show(message, ToastVariant::Info);
    }

    pub fn success(&mut self, message: impl Into<String>) {
        self.show(message, ToastVariant::Success);
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.show(message, ToastVariant::Error);
    }
}

pub fn use_toast() -> ToastContext {
    use_context::<ToastContext>()
}

#[component]
pub fn ToastProvider(children: Element) -> Element {
    let messages = use_signal(Vec::<ToastMessage>::new);
    let next_id = use_signal(|| 0u64);

    use_context_provider(|| ToastContext { messages, next_id });

    rsx! {
        {children}
        ToastViewport { messages }
    }
}

#[component]
fn ToastViewport(messages: Signal<Vec<ToastMessage>>) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-end gap-2 pointer-events-none ",
            class: "fixed left-1/2 bottom-4 z-50 -translate-x-1/2",
            for message in messages.read().iter() {
                Toast {
                    key: "{message.id}",
                    message: message.clone(),
                    on_dismiss: move |id| messages.write().retain(|message| message.id != id),
                }
            }
        }
    }
}

#[component]
fn Toast(message: ToastMessage, on_dismiss: Callback<u64>) -> Element {
    let message_id = message.id;
    let mut closing = use_signal(|| false);

    use_future(move || async move {
        sleep(DEFAULT_TOAST_DURATION_MILLISECONDS).await;
        closing.set(true);
    });

    use_effect(move || {
        if closing() {
            spawn(async move {
                sleep(TOAST_EXIT_DURATION_MILLISECONDS).await;
                on_dismiss.call(message_id);
            });
        }
    });

    let role = match message.variant {
        ToastVariant::Error => "alert",
        ToastVariant::Info | ToastVariant::Success => "status",
    };

    rsx! {
        div {
            class: "grid w-full select-none",
            class: "data-[closing=false]:motion-safe:animate-toast-open",
            class: "data-[closing=true]:opacity-0 data-[closing=true]:motion-safe:animate-toast-close",
            "data-closing": "{closing}",
            style: "--toast-exit-duration: {TOAST_EXIT_DURATION_MILLISECONDS}ms",
            div { class: "min-h-0 overflow-hidden max-w-[50lvw]",
                div {
                    class: "pointer-events-auto flex w-full gap-3 rounded-md border p-3 text-sm shadow-md",
                    class: "border-window-foregroundnormal/20 bg-view-backgroundnormal text-view-foregroundnormal",
                    class: "data-[variant=info]:border-view-foregroundactive data-[variant=info]:text-view-foregroundactive",
                    class: "data-[variant=success]:border-view-foregroundpositive data-[variant=success]:text-view-foregroundpositive",
                    class: "data-[variant=error]:border-view-foregroundnegative data-[variant=error]:text-view-foregroundnegative",
                    "data-variant": "{message.variant}",
                    role,
                    "aria-atomic": "true",
                    p { class: "min-w-0 grow whitespace-pre-wrap", "{message.message}" }
                    button {
                        class: "shrink-0 cursor-pointer rounded-sm px-1 text-lg leading-none hover:text-view-foregroundnormal focus-visible:outline focus-visible:outline-view-decorationfocus",
                        "aria-label": "Dismiss notification",
                        disabled: closing(),
                        onclick: move |_| closing.set(true),
                        "x"
                    }
                }
            }
        }
    }
}
