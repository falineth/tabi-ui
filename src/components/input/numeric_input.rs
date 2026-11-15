use std::fmt::Display;

use dioxus::prelude::*;

use crate::hooks::use_controlled;

#[derive(Clone, PartialEq)]
pub enum NumericInputValue<T>
where
    T: Default,
{
    Valid(T, String),
    Invalid(String),
}

impl<T: Default> NumericInputValue<T> {
    pub fn is_valid(&self) -> bool {
        match self {
            NumericInputValue::Valid(_, _) => true,
            NumericInputValue::Invalid(_) => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

impl<T: Default + Display> Default for NumericInputValue<T> {
    fn default() -> Self {
        let default_value = T::default();

        let default_value_text = format!("{default_value}");

        NumericInputValue::Valid(default_value, default_value_text)
    }
}

impl<T: Display + Default> From<T> for NumericInputValue<T> {
    fn from(value: T) -> Self {
        let value_text = format!("{value}");

        NumericInputValue::Valid(value, value_text)
    }
}

macro_rules! impl_integer_numeric_input_value {
    ($($ty:ty),* $(,)?) => {
        $(
            impl From<String> for NumericInputValue<$ty> {
                fn from(text: String) -> Self {
                    match text.parse::<$ty>() {
                        Ok(value) => NumericInputValue::Valid(value, text),
                        Err(_) => NumericInputValue::Invalid(text),
                    }
                }
            }
        )*
    };
}

impl_integer_numeric_input_value! {
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
}

#[component]
pub fn NumericInput<T>(
    #[props(default)] class: String,

    #[props(default)] default_value: NumericInputValue<T>,

    #[props(default)] value: ReadSignal<Option<NumericInputValue<T>>>,

    #[props(default)] on_value_change: Callback<NumericInputValue<T>>,

    #[props(default)] on_blur: Callback<Event<FocusData>>,

    #[props(default)] on_mounted: Callback<Event<MountedData>>,

    #[props(default)] on_accept: Callback<T>,

    #[props(extends = GlobalAttributes, extends = input)] rest: Vec<Attribute>,
    children: Element,
) -> Element
where
    T: Copy + Default + PartialEq + Display + 'static,
    NumericInputValue<T>: From<String>,
{
    /*

       States

    */

    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    /*

       Memo

    */

    let input_text = use_memo(move || match value() {
        NumericInputValue::Valid(_, text) => text,
        NumericInputValue::Invalid(text) => text,
    });

    /*

       Callbacks

    */

    let handle_input = use_callback(move |e: Event<FormData>| {
        set_value_internal(e.data.value().into());
    });

    let handle_accept = use_callback(move |e: Event<KeyboardData>| {
        if e.key() == Key::Enter
            && let NumericInputValue::Valid(value, _) = value()
        {
            on_accept.call(value);
        }
    });

    let handle_on_blur = use_callback(move |e: Event<FocusData>| {
        if let NumericInputValue::Valid(value, _) = value() {
            set_value_internal(value.into());
        }

        on_blur.call(e)
    });

    /*

       Elements

    */

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
            value: "{input_text}",
            aria_invalid: value.read().is_invalid(),
            oninput: handle_input,
            onblur: handle_on_blur,
            onmounted: move |e| on_mounted.call(e),
            onkeydown: handle_accept,
            ..rest,
            {children}
        }
    }
}
