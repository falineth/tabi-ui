use dioxus::core::AttributeValue;
use dioxus::prelude::*;

use crate::hooks::use_controlled;

#[component]
pub fn TextArea(
    #[props(default)] class: String,

    #[props(default)] initial_value: ReadSignal<Option<String>>,
    #[props(default)] value: ReadSignal<Option<String>>,

    #[props(default)] on_blur: Callback<Event<FocusData>>,
    #[props(default)] on_mounted: Callback<Event<MountedData>>,
    #[props(default)] on_value_change: Callback<String>,
    #[props(default)] on_accept: Option<Callback<String>>,

    #[props(extends = GlobalAttributes, extends = textarea)] rest: Vec<Attribute>,
) -> Element {
    let mut current_value = use_signal(move || {
        value
            .read()
            .cloned()
            .unwrap_or_else(move || initial_value().unwrap_or_default())
    });

    let handle_on_input = use_callback(move |event: Event<FormData>| {
        current_value.set(event.data.value());
        on_value_change(event.data.value());
    });

    let handle_keydown = use_callback(move |e: Event<KeyboardData>| {
        if let Some(on_accept) = on_accept
            && e.key() == Key::Enter
            && e.modifiers().is_empty()
        {
            e.prevent_default();

            if let Some(value) = value() {
                on_accept.call(value);
                return;
            }

            on_accept.call(current_value());
        }
    });

    use_effect(move || {
        if let Some(value) = value.read().as_ref() {
            current_value.set(value.clone());
        }
    });

    let optional = match value.read().as_ref() {
        Some(value) => vec![Attribute {
            name: "value",
            value: AttributeValue::Text(value.clone()),
            namespace: None,
            volatile: true,
        }],
        None => Vec::default(),
    };

    rsx! {
        textarea {
            "data-slot": "textarea",
            class: "border-view-foregroundnormal/20 bg-view-backgroundnormal text-view-foregroundnormal hover:border-view-decorationhover focus-visible:border-view-decorationfocus placeholder:text-view-foregroundinactive",
            class: "aria-invalid:ring-view-foregroundnegative/20 aria-invalid:border-view-foregroundnegative",
            class: "resize-none rounded-sm border px-2 py-2 text-sm transition-colors aria-invalid:ring-2 md:text-xs/relaxed flex field-sizing-content min-h-16 w-full outline-none disabled:cursor-not-allowed disabled:opacity-50",
            class: "{class}",
            initial_value,
            oninput: handle_on_input,
            onkeydown: handle_keydown,
            ..optional,
            ..rest,
        }
    }
}
