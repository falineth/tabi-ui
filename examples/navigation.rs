use dioxus::prelude::*;
use tabi_ui::components::*;
use tabi_ui::icons::{MdHome, MdInfoOutline, MdSettings};

const TAILWIND_CSS: Asset = asset!("../assets/tailwind.css");

#[component]
pub fn NavbarView() -> Element {
    let nav = navigator();

    let handle_goto_home = use_callback(move |_| {
        nav.replace(Route::HomeView {});
    });

    let handle_goto_settings = use_callback(move |_| {
        nav.replace(Route::SettingsView {});
    });

    let handle_goto_about = use_callback(move |_| {
        nav.replace(Route::AboutView {});
    });

    rsx! {
        GlobalDraw {
            actions: rsx! {
                DrawAction { icon: MdHome, label: "Home", onclick: handle_goto_home }
                DrawAction { icon: MdSettings, label: "Settings", onclick: handle_goto_settings }
                DrawAction { icon: MdInfoOutline, label: "About", onclick: handle_goto_about }
            },
            content: rsx! {
                div { class: "p-2", Slider {} }
            },
            Outlet::<Route> {}
        }
    }
}

#[component]
pub fn HomeView() -> Element {
    rsx! {
        div { class: "w-full",
            BreadcrumbBar {
                Breadcrumb {
                    BreadcrumbList {
                        BreadcrumbItem {
                            BreadcrumbPage { "Home" }
                        }
                    }
                }
            }
            div { class: "p-4", "Home View" }
        }
    }
}

#[component]
pub fn SettingsView() -> Element {
    rsx! {
        div { class: "w-full",
            div {
                class: "flex px-4 items-center h-12 w-full border-b",
                class: "border-window-foregroundnormal/20",
                Breadcrumb {
                    BreadcrumbList {
                        BreadcrumbItem {
                            BreadcrumbLink { to: Route::HomeView {}, "Home" }
                        }
                        BreadcrumbSeparator {}
                        BreadcrumbItem {
                            BreadcrumbPage { "Settings" }
                        }
                    }
                }
            }
            div { class: "p-4", "Settings View" }
        }
    }
}

#[component]
pub fn AboutView() -> Element {
    rsx! {
        div { class: "w-full",
            div {
                class: "flex px-4 items-center h-12 w-full border-b",
                class: "border-window-foregroundnormal/20",
                Breadcrumb {
                    BreadcrumbList {
                        BreadcrumbItem {
                            BreadcrumbLink { to: Route::HomeView {}, "Home" }
                        }
                        BreadcrumbSeparator {}
                        BreadcrumbItem {
                            BreadcrumbPage { "About" }
                        }
                    }
                }
            }
            div { class: "p-4", "About View" }
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(NavbarView)]
    #[route("/")]
    HomeView {},

    #[route("/settings")]
    SettingsView {},

    #[route("/about")]
    AboutView {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        TabiDefaultContext { Router::<Route> {} }
    }
}

#[cfg(feature = "desktop")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    use dioxus::desktop::{Config, WindowBuilder};
    use tabi_ui::ThemeContext;

    let theme_context = ThemeContext::init().await;

    let mut config = Config::default()
        .with_background_color(theme_context.bg_color)
        .with_window(WindowBuilder::new().with_title("Navigation Example"));

    if cfg!(not(debug_assertions)) {
        config = config.with_menu(None);
    }

    LaunchBuilder::new()
        .with_cfg(config)
        .with_context(theme_context)
        .launch(App);
}

#[cfg(feature = "web")]
fn main() {
    dioxus::launch(App);
}
