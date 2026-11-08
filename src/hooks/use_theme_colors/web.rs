use dioxus::prelude::*;

use crate::ThemeDetails;

const DEFAULT_THEME: &str = include_str!("../../../assets/default_theme.css");
const DEFAULT_DARK_THEME: &str = include_str!("../../../assets/default_dark_theme.css");
const DEFAULT_WINDOW_BACKGROUND: (u8, u8, u8, u8) = (0xEF, 0xF0, 0xF1, 0xFF);
const DEFAULT_DARK_WINDOW_BACKGROUND: (u8, u8, u8, u8) = (0x20, 0x23, 0x26, 0xFF);

pub fn use_theme_colors() -> ThemeDetails {
    let theme_colors = use_signal_sync(get_theme_colors);
    let prefer_dark = use_signal_sync(get_is_dark_mode);

    ThemeDetails {
        theme_colors: theme_colors.into(),
        prefer_dark: prefer_dark.into(),
    }
}

fn get_theme_colors() -> String {
    if get_is_dark_mode() {
        DEFAULT_DARK_THEME.to_string()
    } else {
        DEFAULT_THEME.to_string()
    }
}

pub fn get_window_background_color() -> (u8, u8, u8, u8) {
    if get_is_dark_mode() {
        DEFAULT_DARK_WINDOW_BACKGROUND
    } else {
        DEFAULT_WINDOW_BACKGROUND
    }
}

pub fn get_is_dark_mode() -> bool {
    web_sys::window()
        .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok()?)
        .map(|query| query.matches())
        .unwrap()
}

#[derive(Clone)]
pub struct ThemeContext {
    pub theme_css: String,
    pub bg_color: (u8, u8, u8, u8),
}

impl ThemeContext {
    pub fn init() -> Self {
        if get_is_dark_mode() {
            ThemeContext {
                theme_css: DEFAULT_DARK_THEME.to_string(),
                bg_color: DEFAULT_DARK_WINDOW_BACKGROUND,
            }
        } else {
            ThemeContext {
                theme_css: DEFAULT_THEME.to_string(),
                bg_color: DEFAULT_WINDOW_BACKGROUND,
            }
        }
    }
}
