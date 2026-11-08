#[cfg(all(
    not(feature = "web"),
    not(all(target_os = "linux", feature = "desktop"))
))]
mod default;
#[cfg(all(not(feature = "web"), all(target_os = "linux", feature = "desktop")))]
mod linux;
#[cfg(feature = "web")]
mod web;

use dioxus::prelude::*;

#[cfg(all(
    not(feature = "web"),
    not(all(target_os = "linux", feature = "desktop"))
))]
pub use crate::hooks::use_theme_colors::default::*;
#[cfg(all(not(feature = "web"), all(target_os = "linux", feature = "desktop")))]
pub use crate::hooks::use_theme_colors::linux::*;
#[cfg(feature = "web")]
pub use crate::hooks::use_theme_colors::web::*;

pub struct ThemeDetails {
    pub theme_colors: ReadSignal<String, SyncStorage>,
    pub prefer_dark: ReadSignal<bool, SyncStorage>,
}
