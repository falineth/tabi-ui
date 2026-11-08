use dioxus::prelude::*;

use crate::{ThemeDetails, use_theme_colors};

#[component]
pub fn ThemeStyle() -> Element {
    let ThemeDetails { theme_colors, .. } = use_theme_colors();

    rsx! {
        style { "{theme_colors}" }
    }
}
