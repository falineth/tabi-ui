use std::fmt::Display;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::hooks::use_controlled;
use crate::utils::nanoid;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

trait SliderValueCalculations<T> {
    fn get_max(&self) -> T;
    fn get_small_step(&self) -> T;
    fn get_large_step(&self) -> T;
    fn get_fraction(value: T, min: T, max: T) -> f32;
    fn round_to_small_step(value: T, small_step: T) -> T;
    fn adjust_by_range_fraction(start_value: T, min: T, max: T, fraction: f64) -> T;
    fn clamp(value: T, min: T, max: T) -> T;
    fn add_step(value: T, step: T) -> T;
    fn subtract_step(value: T, step: T) -> T;
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderProps<T: Copy + PartialEq + Default + 'static = f32> {
    #[props(default)]
    pub class: String,

    #[props(default)]
    pub orientation: Option<SliderOrientation>,

    #[props(default)]
    pub default_value: T,
    #[props(default)]
    pub value: ReadSignal<Option<T>>,
    #[props(default)]
    pub on_value_change: Callback<T>,

    #[props(default)]
    pub min: T,
    #[props(default)]
    pub max: Option<T>,

    #[props(default)]
    pub small_step: Option<T>,
    #[props(default)]
    pub large_step: Option<T>,
}

macro_rules! impl_integer_value_helper {
    (
        $(
            ($ty:ty, $unsigned:ty, $add:ident, $subtract:ident)
        ),*
        $(,)?
    ) => {
        $(
            impl SliderValueCalculations<$ty> for SliderProps<$ty> {
                fn get_max(&self) -> $ty {
                    self.max.unwrap_or(100)
                }

                fn get_small_step(&self) -> $ty {
                    self.small_step.unwrap_or(1)
                }

                fn get_large_step(&self) -> $ty {
                    self.large_step.unwrap_or(10)
                }

                fn get_fraction(value: $ty, min: $ty, max: $ty) -> f32 {
                    if min == max {
                        return 0.0;
                    }

                    let distance = value.abs_diff(min) as f64;
                    let distance = if value < min { -distance } else { distance };

                    (distance / max.abs_diff(min) as f64) as f32
                }

                fn round_to_small_step(value: $ty, small_step: $ty) -> $ty {
                    if small_step > 0 {
                        (value / small_step) * small_step
                    } else {
                        value
                    }
                }

                fn adjust_by_range_fraction(
                    start_value: $ty,
                    min: $ty,
                    max: $ty,
                    fraction: f64,
                ) -> $ty {
                    let change = max.abs_diff(min) as f64 * fraction;

                    if change < 0.0 {
                        start_value.$subtract((-change) as $unsigned)
                    } else {
                        start_value.$add(change as $unsigned)
                    }
                }

                fn clamp(value: $ty, min: $ty, max: $ty) -> $ty {
                    value.clamp(min, max)
                }

                fn add_step(value: $ty, step: $ty) -> $ty {
                    value.saturating_add(step)
                }

                fn subtract_step(value: $ty, step: $ty) -> $ty {
                    value.saturating_sub(step)
                }
            }
        )*
    };
}

impl_integer_value_helper! {
    (i8, u8, saturating_add_unsigned, saturating_sub_unsigned),
    (i16, u16, saturating_add_unsigned, saturating_sub_unsigned),
    (i32, u32, saturating_add_unsigned, saturating_sub_unsigned),
    (i64, u64, saturating_add_unsigned, saturating_sub_unsigned),
    (i128, u128, saturating_add_unsigned, saturating_sub_unsigned),
    (isize, usize, saturating_add_unsigned, saturating_sub_unsigned),
    (u8, u8, saturating_add, saturating_sub),
    (u16, u16, saturating_add, saturating_sub),
    (u32, u32, saturating_add, saturating_sub),
    (u64, u64, saturating_add, saturating_sub),
    (u128, u128, saturating_add, saturating_sub),
    (usize, usize, saturating_add, saturating_sub),
}

macro_rules! impl_float_value_helper {
    (
        $($ty:ty),*
        $(,)?
    ) => {
        $(
            impl SliderValueCalculations<$ty> for SliderProps<$ty> {
                fn get_max(&self) -> $ty {
                    self.max.unwrap_or(100.0)
                }

                fn get_small_step(&self) -> $ty {
                    self.small_step.unwrap_or(1.0)
                }

                fn get_large_step(&self) -> $ty {
                    self.large_step.unwrap_or(10.0)
                }

                fn get_fraction(value: $ty, min: $ty, max: $ty) -> f32 {
                    let value_range = max - min;

                    ((value - min) / value_range) as f32
                }

                fn round_to_small_step(value: $ty, small_step: $ty) -> $ty {
                    if small_step > 0.0 {
                        (value / small_step).round() * small_step
                    } else {
                        value
                    }
                }

                #[allow(clippy::cast_possible_truncation)]
                fn adjust_by_range_fraction(start_value: $ty, min: $ty, max: $ty, fraction: f64) -> $ty {
                    let value_range: f64 = (max - min).into();

                    let value_change = value_range * fraction;

                    start_value + (value_change as $ty)
                }

                fn clamp(value: $ty, min: $ty, max: $ty) -> $ty {
                    value.clamp(min, max)
                }

                fn add_step(value: $ty, step: $ty) -> $ty {
                    value + step
                }

                fn subtract_step(value: $ty, step: $ty) -> $ty {
                    value - step
                }
            }
        )*
    };
}

impl_float_value_helper!(f32, f64);

#[component]
pub fn Slider<T>(props: SliderProps<T>) -> Element
where
    T: Copy + PartialEq + Default + Display + 'static,
    SliderProps<T>: SliderValueCalculations<T>,
{
    /*

       Props

    */

    let small_step = props.get_small_step();
    let large_step = props.get_large_step();
    let max = props.get_max();

    let SliderProps {
        class,

        orientation,

        default_value,
        value,
        on_value_change,

        min,
        max: _,

        small_step: _,
        large_step: _,
    } = props;

    /*

       State

    */

    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    let thumb_id = use_signal(nanoid);

    let mut slider_size: Signal<Option<f64>> = use_signal(Option::default);

    let mut drag_start_position: Signal<Option<f64>> = use_signal(Option::default);

    let mut drag_start_value: Signal<T> = use_signal(|| *value.read());

    let mut thumb_ref: Signal<Option<Rc<MountedData>>> = use_signal(Option::default);

    /*

       Memos

    */

    let display_position = use_memo(move || {
        let display_value = value();

        100.0 * SliderProps::get_fraction(display_value, min, max)
    });

    /*

       Callbacks

    */

    let set_value = use_callback(move |new_value: T| {
        let new_value = SliderProps::round_to_small_step(new_value, small_step);

        set_value_internal.call(SliderProps::clamp(new_value, min, max));
    });

    let handle_keydown = use_callback(move |event: Event<KeyboardData>| {
        if drag_start_position.read().is_some() {
            if event.key() == Key::Escape {
                drag_start_position.set(None);

                event.prevent_default();
            }

            return;
        };

        match event.key() {
            Key::Home => {
                set_value.call(min);
                event.prevent_default();
            }
            Key::End => {
                set_value.call(max);
                event.prevent_default();
            }
            Key::PageUp => {
                set_value.call(SliderProps::add_step(value(), large_step));
                event.prevent_default();
            }
            Key::PageDown => {
                set_value.call(SliderProps::subtract_step(value(), large_step));
                event.prevent_default();
            }
            Key::ArrowRight | Key::ArrowUp => {
                set_value.call(SliderProps::add_step(value(), small_step));
                event.prevent_default();
            }
            Key::ArrowLeft | Key::ArrowDown => {
                set_value.call(SliderProps::subtract_step(value(), small_step));
                event.prevent_default();
            }
            _ => {}
        }
    });

    let handle_pointerdown = use_callback(move |event: Event<PointerData>| {
        drag_start_value.set(value());

        if orientation == Some(SliderOrientation::Vertical) {
            drag_start_position.set(Some(event.client_coordinates().y));
        } else {
            drag_start_position.set(Some(event.client_coordinates().x));
        }

        let pointer_id = event.data().pointer_id();

        let thumb_id = thumb_id.cloned();

        event.prevent_default();

        spawn(async move {
            if let Some(thumb_ref) = thumb_ref() {
                _ = thumb_ref.set_focus(true).await;
            }

            _ = document::eval(&format!(
                "document.getElementById('{}').setPointerCapture({})",
                thumb_id.as_str(),
                pointer_id
            ))
            .await;
        });
    });

    let handle_pointermove = use_callback(move |event: Event<PointerData>| {
        let Some(slider_size) = slider_size() else {
            return;
        };

        let Some(start) = drag_start_position() else {
            return;
        };

        let slide_amount = if orientation == Some(SliderOrientation::Vertical) {
            start - event.client_coordinates().y
        } else {
            event.client_coordinates().x - start
        };

        let slide_fraction = slide_amount / slider_size;

        let new_value =
            SliderProps::adjust_by_range_fraction(drag_start_value(), min, max, slide_fraction);

        set_value.call(new_value);
    });

    let handle_pointerup = use_callback(move |event: Event<PointerData>| {
        let pointer_id = event.data().pointer_id();

        let thumb_id = thumb_id.cloned();

        drag_start_position.set(None);

        spawn(async move {
            _ = document::eval(&format!(
                r#"
const thumb = document.getElementById('{}');
const pointerId = {};
if (thumb && thumb.hasPointerCapture(pointerId)) {{
    thumb.releasePointerCapture(pointerId);
}}
                "#,
                thumb_id.as_str(),
                pointer_id
            ))
            .await;
        });
    });

    let handle_resize = use_callback(move |event: Event<ResizeData>| {
        let Ok(size) = event.get_content_box_size() else {
            return;
        };

        if orientation == Some(SliderOrientation::Vertical) {
            slider_size.set(Some(size.height));
        } else {
            slider_size.set(Some(size.width));
        }
    });

    let handle_thumb_mounted = use_callback(move |event: Event<MountedData>| {
        thumb_ref.set(Some(event.data()));
    });

    /*

       Elements

    */

    rsx! {
        div {
            dir: "ltr",
            "data-orientation": match orientation.unwrap_or_default() {
                SliderOrientation::Horizontal => "horizontal",
                SliderOrientation::Vertical => "vertical",
            },
            aria_disabled: false,
            "data-slot": "slider",
            class: "relative flex touch-none items-center select-none min-w-40 m-1",
            class: "data-vertical:min-h-40 data-vertical:min-w-auto data-vertical:w-auto data-vertical:flex-col",
            class: "data-disabled:opacity-50",
            class: "{class}",
            onresize: handle_resize,
            div {
                "data-orientation": match orientation.unwrap_or_default() {
                    SliderOrientation::Horizontal => "horizontal",
                    SliderOrientation::Vertical => "vertical",
                },
                "data-slot": "slider-track",
                class: "bg-window-decorationfocus/60",
                class: "relative grow overflow-hidden rounded-full data-horizontal:h-1.25 data-horizontal:w-full data-vertical:h-full data-vertical:w-1.25",
                div {
                    "data-orientation": match orientation.unwrap_or_default() {
                        SliderOrientation::Horizontal => "horizontal",
                        SliderOrientation::Vertical => "vertical",
                    },
                    "data-slot": "slider-range",
                    class: "bg-[color-mix(in_srgb,var(--color-window-foregroundnormal)_20%,var(--color-window-backgroundnormal))]",
                    class: "border-[color-mix(in_srgb,var(--color-window-foregroundnormal)_40%,var(--color-window-backgroundnormal))]",
                    class: "absolute  border-[0.5px]  rounded-full select-none data-horizontal:h-full data-vertical:w-full",
                    style: match orientation.unwrap_or_default() {
                        SliderOrientation::Horizontal => {
                            format!("right: 0%; left: {}%", display_position)
                        }
                        SliderOrientation::Vertical => format!("top: 0%; bottom: {}%", display_position),
                    },
                }
            }
            div {
                "data-slot": "slider-thumb",
                style: match orientation.unwrap_or_default() {
                    SliderOrientation::Horizontal => {
                        format!(
                            "transform: translateX(-50%); position: absolute; left: calc({}% + 0px);",
                            display_position,
                        )
                    }
                    SliderOrientation::Vertical => {
                        format!(
                            "transform: translateY(50%); position: absolute; bottom: calc({}% + 0px);",
                            display_position,
                        )
                    }
                },
                div {
                    id: thumb_id,
                    role: "slider",
                    tabindex: "0",
                    aria_valuemin: "{min}",
                    aria_valuemax: "{max}",
                    aria_orientation: match orientation.unwrap_or_default() {
                        SliderOrientation::Horizontal => "horizontal",
                        SliderOrientation::Vertical => "vertical",
                    },
                    aria_valuenow: "{value}",
                    "data-orientation": match orientation.unwrap_or_default() {
                        SliderOrientation::Horizontal => "horizontal",
                        SliderOrientation::Vertical => "vertical",
                    },
                    "data-slot": "slider-thumb",
                    "data-radix-collection-item": "",
                    class: "ring-window-foregroundnormal/5 border-window-foregroundnormal/30 bg-window-backgroundnormal",
                    class: "hover:ring-1 hover:border-window-decorationhover focus-visible:ring-1 focus-visible:outline-hidden active:ring-1",
                    class: "disabled:pointer-events-none disabled:opacity-50",
                    class: "after:absolute after:-inset-2",
                    class: "relative block size-4.5 shrink-0 rounded-full border transition-[color,box-shadow] select-none",
                    onmounted: handle_thumb_mounted,
                    onkeydown: handle_keydown,
                    onpointerdown: handle_pointerdown,
                    onpointermove: handle_pointermove,
                    onpointerup: handle_pointerup,
                }
            }
        }
    }
}
