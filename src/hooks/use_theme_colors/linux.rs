use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use dbus::arg::{RefArg, Variant};
use dbus::message::MatchRule;
use dbus::nonblock::{MsgMatch, Proxy, SyncConnection};
use dioxus::prelude::*;

use crate::ThemeDetails;

const DEFAULT_THEME: &str = include_str!("../../../assets/default_theme.css");
const DEFAULT_WINDOW_BACKGROUND: (u8, u8, u8, u8) = (0xEF, 0xF0, 0xF1, 0xFF);

type VariantRefArg = Variant<Box<dyn RefArg + 'static>>;

type ConfigSection = HashMap<String, HashMap<String, VariantRefArg>>;

#[derive(Clone)]
pub struct ThemeContext {
    pub dbus_conn: Option<Arc<SyncConnection>>,
    pub theme_css: String,
    pub bg_color: (u8, u8, u8, u8),
    pub prefer_dark: bool,
}

impl ThemeContext {
    pub async fn init() -> Self {
        let Ok((resource, conn)) = dbus_tokio::connection::new_session_sync() else {
            return ThemeContext {
                dbus_conn: None,
                theme_css: DEFAULT_THEME.to_string(),
                bg_color: DEFAULT_WINDOW_BACKGROUND,
                prefer_dark: false,
            };
        };

        tokio::spawn(async {
            let err = resource.await;
            println!("DBus connection lost: {:?}", err);
        });

        let Some((theme_css, bg_color, prefer_dark)) = get_theme_css(conn.clone()).await else {
            return ThemeContext {
                dbus_conn: Some(conn),
                theme_css: DEFAULT_THEME.to_string(),
                bg_color: DEFAULT_WINDOW_BACKGROUND,
                prefer_dark: false,
            };
        };

        return ThemeContext {
            dbus_conn: Some(conn),
            theme_css,
            bg_color,
            prefer_dark,
        };
    }
}

pub fn use_theme_colors() -> ThemeDetails {
    /*

        Contexts

    */

    let theme_context = try_use_context::<ThemeContext>();

    /*

        State

    */

    let theme_colors: Signal<String, SyncStorage> = use_signal_sync(|| {
        if let Some(theme_context) = theme_context.as_ref() {
            return theme_context.theme_css.clone();
        }

        DEFAULT_THEME.to_string()
    });

    let prefer_dark: Signal<bool, SyncStorage> = use_signal_sync(|| {
        if let Some(theme_context) = theme_context.as_ref() {
            return theme_context.prefer_dark;
        }

        return false;
    });

    let mut dbus_conn: Signal<Option<Arc<SyncConnection>>, SyncStorage> =
        use_signal_sync(Option::default);

    let mut dbus_incoming_signal: Signal<Option<MsgMatch>, SyncStorage> =
        use_signal_sync(Option::default);

    /*

        Futures

    */

    use_future(move || {
        let theme_context = theme_context.clone();

        async move {
            let Some(conn) = get_dbus_connection(theme_context.as_ref()) else {
                return;
            };

            let signal_match_rule =
                MatchRule::new_signal("org.freedesktop.portal.Settings", "SettingChanged");

            let Ok(message_match) = conn
                .add_match(signal_match_rule)
                .await
                .inspect_err(|err| println!("Error 1 {err:?}"))
            else {
                return;
            };

            dbus_conn.set(Some(conn.clone()));

            theme_refresh(theme_colors, prefer_dark, dbus_conn.cloned());

            let incoming_signal = message_match.cb(
                move |_, (source, setting, _value): (String, String, VariantRefArg)| {
                    if matches!(
                        source.as_str(),
                        "org.kde.kdeglobals.General" | "org.freedesktop.appearance"
                    ) && matches!(
                        setting.as_str(),
                        "ColorScheme" | "color-scheme" | "font" | "fixed"
                    ) {
                        println!("Theme colors changed");
                        theme_refresh(theme_colors, prefer_dark, dbus_conn.cloned());
                    }

                    true
                },
            );

            dbus_incoming_signal.set(Some(incoming_signal));
        }
    });

    return ThemeDetails {
        theme_colors: theme_colors.into(),
        prefer_dark: prefer_dark.into(),
    };
}

fn get_dbus_connection(theme_context: Option<&ThemeContext>) -> Option<Arc<SyncConnection>> {
    if let Some(theme_context) = theme_context
        && let Some(dbus_conn) = theme_context.dbus_conn.as_ref()
    {
        return Some(dbus_conn.clone());
    }

    let (resource, conn) = dbus_tokio::connection::new_session_sync().ok()?;

    tokio::spawn(async {
        let err = resource.await;
        println!("DBus connection lost: {:?}", err);
    });

    return Some(conn);
}

fn theme_refresh(
    mut theme_colors: Signal<String, SyncStorage>,
    mut prefer_dark: Signal<bool, SyncStorage>,
    dbus_conn: Option<Arc<SyncConnection>>,
) {
    let Some(conn) = dbus_conn else {
        return;
    };

    tokio::spawn(async move {
        if let Some((new_theme_css, _bg_color, prefer_dark_flag)) = get_theme_css(conn).await {
            prefer_dark.set(prefer_dark_flag);
            theme_colors.set(new_theme_css);
        }
    });
}

async fn get_theme_css(conn: Arc<SyncConnection>) -> Option<(String, (u8, u8, u8, u8), bool)> {
    let mut css_lines = Vec::new();
    let mut prefer_dark = false;

    let settings = read_portal_settings(
        &conn,
        &[
            "org.freedesktop.appearance",
            "org.kde.kdeglobals.General",
            "org.kde.kdeglobals.Colors:Button",
            "org.kde.kdeglobals.Colors:Complementary",
            "org.kde.kdeglobals.Colors:Selection",
            "org.kde.kdeglobals.Colors:Tooltip",
            "org.kde.kdeglobals.Colors:View",
            "org.kde.kdeglobals.Colors:Window",
        ],
    )
    .await?;

    if let Some(general_settings) = settings.get("org.kde.kdeglobals.General") {
        if let Some(font_declaration) = get_font_css(general_settings, "font", "font-sans") {
            css_lines.push(font_declaration);
            css_lines.push("--default-font-family:var(--font-sans);".to_string());
        }

        if let Some(font_declaration) = get_font_css(general_settings, "fixed", "font-mono") {
            css_lines.push(font_declaration);
            css_lines.push("--default-mono-font-family:var(--font-mono);".to_string());
        }
    }

    if let Some(appearance_settings) = settings.get("org.freedesktop.appearance") {
        prefer_dark = appearance_settings
            .get("color-scheme")
            .map(|value| value == "1")
            .unwrap_or_default();
    }

    if let Some(button_colors) =
        get_color_css(&settings, "org.kde.kdeglobals.Colors:Button", "button")
    {
        css_lines.extend(button_colors);
    }

    if let Some(button_colors) = get_color_css(
        &settings,
        "org.kde.kdeglobals.Colors:Complementary",
        "complementary",
    ) {
        css_lines.extend(button_colors);
    }

    if let Some(button_colors) = get_color_css(
        &settings,
        "org.kde.kdeglobals.Colors:Selection",
        "selection",
    ) {
        css_lines.extend(button_colors);
    }

    if let Some(button_colors) =
        get_color_css(&settings, "org.kde.kdeglobals.Colors:Tooltip", "tooltip")
    {
        css_lines.extend(button_colors);
    }

    if let Some(button_colors) = get_color_css(&settings, "org.kde.kdeglobals.Colors:View", "view")
    {
        css_lines.extend(button_colors);
    }

    if let Some(button_colors) =
        get_color_css(&settings, "org.kde.kdeglobals.Colors:Window", "window")
    {
        css_lines.extend(button_colors);
    }

    let bg_color = settings
        .get("org.kde.kdeglobals.Colors:Window")
        .and_then(|window_colors| window_colors.get("backgroundnormal"))
        .and_then(|bg_color| get_color_value(bg_color))
        .unwrap_or(DEFAULT_WINDOW_BACKGROUND);

    css_lines.push("--color-background:var(--color-window-backgroundnormal);--color-foreground:var(--color-window-foregroundnormal);--background:var(--color-window-backgroundnormal);--foreground:var(--color-window-foregroundnormal);body{background-color:var(--color-window-backgroundnormal);}}".to_string());

    let theme_css = format!(":root {{{}}}", css_lines.join(""));

    return Some((theme_css, bg_color, prefer_dark));
}

fn get_font_css(
    general_settings: &HashMap<String, String>,
    key: &str,
    theme_value: &str,
) -> Option<String> {
    let font_value = general_settings.get(key)?;

    let font_name = font_value
        .split_once(",")
        .map(|(font_name, _remainder)| font_name.to_string())?;

    format!("--{theme_value}:{font_name};").into()
}

fn get_color_css(
    settings: &HashMap<String, HashMap<String, String>>,
    section_name: &str,
    theme_section: &str,
) -> Option<Vec<String>> {
    let color_section = settings.get(section_name)?;

    color_section
        .iter()
        .filter_map(|(key, value)| format_color_value(theme_section, key, value))
        .collect::<Vec<_>>()
        .into()
}

fn format_color_value(theme_section: &str, key: &str, value: &str) -> Option<String> {
    let (r, g_b) = value.split_once(",")?;

    let r = r.parse::<u8>().ok()?;

    let (g, b) = g_b.split_once(",")?;

    let g = g.parse::<u8>().ok()?;

    let b = b.parse::<u8>().ok()?;

    return Some(format!(
        "--color-{theme_section}-{key}:#{r:02x}{g:02x}{b:02x};"
    ));
}

fn get_color_value(value: &str) -> Option<(u8, u8, u8, u8)> {
    let (r, g_b) = value.split_once(",")?;

    let r = r.parse::<u8>().ok()?;

    let (g, b) = g_b.split_once(",")?;

    let g = g.parse::<u8>().ok()?;

    let b = b.parse::<u8>().ok()?;

    return Some((r, g, b, 0xFF));
}

async fn read_portal_settings(
    conn: &Arc<SyncConnection>,
    namespaces: &[&str],
) -> Option<HashMap<String, HashMap<String, String>>> {
    let proxy = Proxy::new(
        "org.freedesktop.portal.Desktop",
        "/org/freedesktop/portal/desktop",
        Duration::from_secs(10),
        conn.clone(),
    );

    let result: Result<(ConfigSection,), dbus::Error> = proxy
        .method_call("org.freedesktop.portal.Settings", "ReadAll", (namespaces,))
        .await;

    if let Err(err) = result.as_ref() {
        debug!("read_portal_settings result: {err:?}");
    }

    return result
        .ok()?
        .0
        .into_iter()
        .map(|(namespace, values)| {
            (
                namespace,
                values
                    .into_iter()
                    .map(|(key, value)| (key.to_ascii_lowercase(), get_variant_string(value)))
                    .collect::<HashMap<String, String>>(),
            )
        })
        .collect::<HashMap<String, HashMap<String, String>>>()
        .into();
}

fn get_variant_string(value: VariantRefArg) -> String {
    return value
        .as_str()
        .map(|value| value.to_string())
        .or_else(|| value.as_i64().map(|value| value.to_string()))
        .or_else(|| value.as_u64().map(|value| value.to_string()))
        .or_else(|| value.as_f64().map(|value| value.to_string()))
        .unwrap_or_default();
}
