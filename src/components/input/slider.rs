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

#[component]
pub fn Slider(
    #[props(default)] class: String,

    #[props(default)] orientation: Option<SliderOrientation>,

    #[props(default)] default_value: f32,
    #[props(default)] value: ReadSignal<Option<f32>>,
    #[props(default)] on_value_change: Callback<f32>,

    #[props(default)] min: f32,
    #[props(default = 100.0)] max: f32,

    #[props(default = 1.0)] small_step: f32,
    #[props(default = 10.0)] large_step: f32,
) -> Element {
    /*

       State

    */

    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    let thumb_id = use_signal(nanoid);

    let mut slider_size: Signal<Option<f64>> = use_signal(Option::default);

    let mut drag_start_position: Signal<Option<f64>> = use_signal(Option::default);

    let mut drag_start_value: Signal<f32> = use_signal(|| *value.read());

    let mut thumb_ref: Signal<Option<Rc<MountedData>>> = use_signal(Option::default);

    /*

       Memos

    */

    let display_position = use_memo(move || {
        let display_value = *value.read();

        let value_range = max - min;

        return 100.0 * (display_value - min) / value_range;
    });

    /*

       Callbacks

    */

    let set_value = use_callback(move |new_value: f32| {
        let new_value = if small_step > 0.0 {
            (new_value / small_step).round() * small_step
        } else {
            new_value
        };

        set_value_internal.call(new_value.clamp(min, max));
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
                set_value.call(*value.read() + large_step);
                event.prevent_default();
            }
            Key::PageDown => {
                set_value.call(*value.read() - large_step);
                event.prevent_default();
            }
            Key::ArrowRight | Key::ArrowUp => {
                set_value.call(*value.read() + small_step);
                event.prevent_default();
            }
            Key::ArrowLeft | Key::ArrowDown => {
                set_value.call(*value.read() - small_step);
                event.prevent_default();
            }
            _ => {}
        }
    });

    let handle_pointerdown = use_callback(move |event: Event<PointerData>| {
        drag_start_value.set(*value.read());

        if orientation == Some(SliderOrientation::Vertical) {
            drag_start_position.set(Some(event.client_coordinates().y));
        } else {
            drag_start_position.set(Some(event.client_coordinates().x));
        }

        let pointer_id = event.data().pointer_id();

        let thumb_id = thumb_id.cloned();

        event.prevent_default();

        spawn(async move {
            if let Some(thumb_ref) = &*thumb_ref.read() {
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
        let Some(slider_size) = *slider_size.read() else {
            return;
        };

        let Some(start) = *drag_start_position.read() else {
            return;
        };

        let slide_amount = if orientation == Some(SliderOrientation::Vertical) {
            start - event.client_coordinates().y
        } else {
            event.client_coordinates().x - start
        };

        let value_range: f64 = (max - min).into();

        let value_change = value_range * slide_amount / slider_size;

        #[allow(clippy::cast_possible_truncation)]
        let new_value = *drag_start_value.read() + (value_change as f32);

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
                            format!("right: 0%; left: {}%", *display_position.read())
                        }
                        SliderOrientation::Vertical => {
                            format!("top: 0%; bottom: {}%", *display_position.read())
                        }
                    },
                }
            }
            div {
                "data-slot": "slider-thumb",
                style: match orientation.unwrap_or_default() {
                    SliderOrientation::Horizontal => {
                        format!(
                            "transform: translateX(-50%); position: absolute; left: calc({}% + 0px);",
                            *display_position.read(),
                        )
                    }
                    SliderOrientation::Vertical => {
                        format!(
                            "transform: translateY(50%); position: absolute; bottom: calc({}% + 0px);",
                            *display_position.read(),
                        )
                    }
                },
                div {
                    id: thumb_id,
                    role: "slider",
                    tabindex: "0",
                    aria_valuemin: min,
                    aria_valuemax: max,
                    aria_orientation: match orientation.unwrap_or_default() {
                        SliderOrientation::Horizontal => "horizontal",
                        SliderOrientation::Vertical => "vertical",
                    },
                    aria_valuenow: *value.read(),
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
