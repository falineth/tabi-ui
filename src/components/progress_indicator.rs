use std::f32::consts::PI;

use dioxus::prelude::*;

#[component]
pub fn ProgressIndicator(
    #[props(default = 20)] size: u32,
    #[props(default = 0.0)] progress: f32,
) -> Element {
    let filled_radius = 2.0 * PI * 22.0 * progress;

    rsx! {
        svg { width: size, height: size, view_box: "0 0 100 100",
            circle {
                cx: 50,
                cy: 50,
                r: 45,
                fill: "#0000",
                stroke: "currentColor",
                stroke_width: 3,
            }
            circle {
                cx: 50,
                cy: 50,
                r: 22,
                fill: "#0000",
                stroke: "currentColor",
                stroke_width: 44,
                stroke_dasharray: "{filled_radius} 300",
                transform: "rotate(-90 50 50)",
            }
        }
    }
}
