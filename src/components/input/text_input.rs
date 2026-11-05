use dioxus::prelude::*;

use crate::hooks::use_controlled;

#[component]
pub fn TextInput(
    #[props(default)] class: String,

    #[props(default)] value: ReadSignal<Option<String>>,

    #[props(default)] on_value_change: Callback<String>,

    #[props(default)] on_blur: Callback<Event<FocusData>>,

    #[props(default)] on_mounted: Callback<Event<MountedData>>,

    #[props(default)] on_accept: Callback<String>,

    #[props(extends = GlobalAttributes, extends = input)] rest: Vec<Attribute>,
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
        input {
            "data-slot": "input",
            class: "bg-view-backgroundnormal text-view-foregroundnormal placeholder:text-view-foregroundinactive border-view-foregroundnormal/20",
            class: "hover:border-complementary-decorationhover focus-visible:border-complementary-decorationfocus",
            class: "aria-invalid:ring-view-foregroundnegative/20 aria-invalid:border-view-foregroundnegative",
            class: "file:h-6 file:text-xs/relaxed file:font-medium file:text-foreground file:inline-flex file:border-0 file:bg-transparent",
            class: "disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
            class: "h-7 rounded-sm border px-2 py-0.5 text-sm transition-colors aria-invalid:ring-2 md:text-xs/relaxed w-full min-w-0 outline-none",
            class: "{class}",
            value: if value.read().is_some() { value },
            oninput: handle_on_input,
            onblur: move |e| on_blur.call(e),
            onmounted: move |e| on_mounted.call(e),
            onkeydown: move |e| {
                if e.key() == Key::Enter {
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
