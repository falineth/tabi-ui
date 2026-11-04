use dioxus::prelude::*;

use crate::hooks::use_controlled;

#[component]
pub fn TextArea(
    #[props(default)] class: String,

    #[props(default)] value: ReadSignal<Option<String>>,
    #[props(default)] on_value_change: Callback<String>,
    #[props(default)] on_accept: Option<Callback<String>>,

    #[props(extends = GlobalAttributes, extends = textarea)] rest: Vec<Attribute>,

    children: Element,
) -> Element {
    let mut current_value = use_signal(move || value.read().cloned().unwrap_or_default());

    let handle_on_input = use_callback(move |event: Event<FormData>| {
        current_value.set(event.data.value());
        on_value_change(event.data.value());
    });

    use_effect(move || {
        current_value.set(value.cloned().unwrap_or_default());
    });

    rsx! {
        textarea {
            "data-slot": "textarea",
            class: "border-view-foregroundnormal/20 bg-view-backgroundnormal text-view-foregroundnormal hover:border-view-decorationhover focus-visible:border-view-decorationfocus placeholder:text-view-foregroundinactive",
            class: "aria-invalid:ring-view-foregroundnegative/20 aria-invalid:border-view-foregroundnegative",
            class: "resize-none rounded-sm border px-2 py-2 text-sm transition-colors aria-invalid:ring-2 md:text-xs/relaxed flex field-sizing-content min-h-16 w-full outline-none disabled:cursor-not-allowed disabled:opacity-50",
            class: "{class}",
            value: if value.read().is_some() { value },
            oninput: handle_on_input,
            onkeydown: move |e| {
                if let Some(on_accept) = on_accept && e.key() == Key::Enter
                    && e.modifiers().is_empty()
                {
                    e.prevent_default();

                    if let Some(value) = value() {
                        on_accept.call(value);
                        return;
                    }

                    on_accept.call(current_value());
                }
            },
            ..rest,
            {children}
        }
    }
}
