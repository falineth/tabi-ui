use dioxus::prelude::*;
use tabi_ui::ThemeContext;
use tabi_ui::components::*;
use tabi_ui::icons::{MdEdit, MdMoreVert, MdSettings};

const TAILWIND_CSS: Asset = asset!("../assets/tailwind.css");

#[derive(Default, PartialEq)]
enum PageTypes {
    #[default]
    Button,
    TextInput,
    Toggle,
    Slider,
    DropdownMenu,
}

#[component]
fn Pages() -> Element {
    let mut current_page = use_signal(PageTypes::default);

    rsx! {
        div { class: "w-40 shrink-0 h-full flex flex-col p-1 gap-2 border-r bg-view-backgroundnormal border-view-foregroundnormal/20",
            Toggle {
                value: *current_page.read() == PageTypes::Button,
                on_value_change: move |_| current_page.set(PageTypes::Button),
                "Button"
            }
            Toggle {
                value: *current_page.read() == PageTypes::TextInput,
                on_value_change: move |_| current_page.set(PageTypes::TextInput),
                "Text Input"
            }
            Toggle {
                value: *current_page.read() == PageTypes::Toggle,
                on_value_change: move |_| current_page.set(PageTypes::Toggle),
                "Toggle"
            }
            Toggle {
                value: *current_page.read() == PageTypes::Slider,
                on_value_change: move |_| current_page.set(PageTypes::Slider),
                "Slider"
            }
            Toggle {
                value: *current_page.read() == PageTypes::DropdownMenu,
                on_value_change: move |_| current_page.set(PageTypes::DropdownMenu),
                "Dropdown Menu"
            }
        }
        div { class: "grow flex flex-col h-full w-full",
            {
                match *current_page.read() {
                    PageTypes::Button => rsx! {
                        ButtonPage {}
                    },
                    PageTypes::TextInput => rsx! {
                        TextInputPage {}
                    },
                    PageTypes::Toggle => rsx! {
                        TogglePage {}
                    },
                    PageTypes::Slider => rsx! {
                        SliderPage {}
                    },
                    PageTypes::DropdownMenu => rsx! {
                        DropdownMenuPage {}
                    },
                }
            }
        }

    }
}

#[component]
fn ButtonPage() -> Element {
    rsx! {
        div { class: "flex flex-col p-1 gap-1",
            h1 { "Variants" }
            div { class: "flex flex-wrap p-2 gap-1",
                Button { variant: ButtonVariant::Default, "Default" }
                Button { variant: ButtonVariant::GhostWindow, "GhostWindow" }
                Button { variant: ButtonVariant::Outline, "Outline" }
                Button { variant: ButtonVariant::Secondary, "Secondary" }
                Button { variant: ButtonVariant::Destructive, "Destructive" }
                Button { variant: ButtonVariant::Link, "Link" }
                Button { variant: ButtonVariant::Custom, "Custom" }
            }
            Card { class: "flex-row! flex-wrap p-2! gap-1!",
                Button { variant: ButtonVariant::Default, "Default" }
                Button { variant: ButtonVariant::GhostView, "GhostView" }
            }
            h1 { "Sizes" }
            div { class: "flex flex-wrap gap-1 items-end",
                Button { size: ButtonSize::XS,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                    "XS"
                }
                Button { size: ButtonSize::SM,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                    "SM"
                }
                Button { size: ButtonSize::Default,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                    "Default"
                }
                Button { size: ButtonSize::LG,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                    "LG"
                }
            }
            div { class: "flex flex-wrap gap-1 items-end",
                "IconXS"
                Button { size: ButtonSize::IconXS,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                }
                "IconSM"
                Button { size: ButtonSize::IconSM,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                }
                "Icon"
                Button { size: ButtonSize::Icon,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                }
                "IconLG"
                Button { size: ButtonSize::IconLG,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                }
            }
        }
    }
}

#[component]
fn TextInputPage() -> Element {
    rsx! {
        div { class: "flex flex-col p-1 gap-1",
            div { class: "p-2",
                TextInput { class: "p-4", placeholder: "placeholder" }
            }
            Card { class: "flex-row! flex-wrap p-2! gap-1!",
                TextInput { placeholder: "placeholder" }
            }
        }
    }
}

#[component]
fn TogglePage() -> Element {
    let mut toggled = use_signal(|| Some(false));

    let handle_toggle = use_callback(move |value: bool| {
        toggled.set(Some(value));
    });

    rsx! {
        div { class: "flex flex-col p-1 gap-1",
            h1 { "Variants" }
            div { class: "flex flex-wrap p-2 gap-1",
                Toggle {
                    variant: ToggleVariant::Default,
                    value: toggled,
                    on_value_change: handle_toggle,
                    "Default"
                }
                Toggle {
                    variant: ToggleVariant::Outline,
                    value: toggled,
                    on_value_change: handle_toggle,
                    "Outline"
                }
            }
            Card { class: "flex-row! flex-wrap! px-2! gap-1!",
                Toggle {
                    variant: ToggleVariant::Default,
                    value: toggled,
                    on_value_change: handle_toggle,
                    "Default"
                }
                Toggle {
                    variant: ToggleVariant::Outline,
                    value: toggled,
                    on_value_change: handle_toggle,
                    "Outline"
                }
            }
            h1 { "Sizes" }
            div { class: "flex items-end gap-1",
                Toggle {
                    size: ToggleSize::SM,
                    value: toggled,
                    on_value_change: handle_toggle,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                    "SM"
                }
                Toggle {
                    size: ToggleSize::Default,
                    value: toggled,
                    on_value_change: handle_toggle,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                    "Default"
                }
                Toggle {
                    size: ToggleSize::LG,
                    value: toggled,
                    on_value_change: handle_toggle,
                    svg { view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M0 0h24v24H0z" }
                    }
                    "LG"
                }
            }
        }
    }
}

#[component]
fn SliderPage() -> Element {
    let mut value = use_signal(|| Some(50f32));

    rsx! {
        div { class: "flex flex-col items-start p-4 gap-4",
            "value: {value.read().unwrap_or_default()}"
            Slider {
                class: "",
                value,
                on_value_change: move |new_value| value.set(Some(new_value)),
            }
            Slider {
                class: "",
                value,
                on_value_change: move |new_value| value.set(Some(new_value)),
                orientation: SliderOrientation::Vertical,
            }
        }
    }
}

#[component]
fn DropdownMenuPage() -> Element {
    rsx! {
        div { class: "flex flex-col items-start p-4 gap-4",
            h1 { "Dropdown Menu" }
            DropdownMenu { icon: MdMoreVert,
                MenuItem { icon: MdSettings, label: "Settings" }
                MenuItem { icon: MdEdit, label: "Edit" }
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        TabiDefaultContext { class: "h-lvh w-lvw flex", Pages {} }
    }
}

#[cfg(feature = "desktop")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    use dioxus::desktop::{Config, WindowBuilder};

    let theme_context = ThemeContext::init().await;

    let mut config = Config::default()
        .with_background_color(theme_context.bg_color)
        .with_window(WindowBuilder::new().with_title("Component Gallery"));

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
