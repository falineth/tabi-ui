use dioxus::prelude::*;
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
    ContextMenu,
    Dialog,
    Toast,
    BusyOverlay,
}

#[component]
fn Pages() -> Element {
    let mut current_page = use_signal(PageTypes::default);

    rsx! {
        div { class: "w-40 shrink-0 h-full flex flex-col p-1 gap-2 border-r bg-view-backgroundnormal border-view-foregroundnormal/20",
            Toggle {
                value: *current_page.read() == PageTypes::BusyOverlay,
                on_value_change: move |_| current_page.set(PageTypes::BusyOverlay),
                "Busy Overlay"
            }
            Toggle {
                value: *current_page.read() == PageTypes::Button,
                on_value_change: move |_| current_page.set(PageTypes::Button),
                "Button"
            }
            Toggle {
                value: *current_page.read() == PageTypes::ContextMenu,
                on_value_change: move |_| current_page.set(PageTypes::ContextMenu),
                "Context Menu"
            }
            Toggle {
                value: *current_page.read() == PageTypes::Dialog,
                on_value_change: move |_| current_page.set(PageTypes::Dialog),
                "Dialog"
            }
            Toggle {
                value: *current_page.read() == PageTypes::DropdownMenu,
                on_value_change: move |_| current_page.set(PageTypes::DropdownMenu),
                "Dropdown Menu"
            }
            Toggle {
                value: *current_page.read() == PageTypes::Slider,
                on_value_change: move |_| current_page.set(PageTypes::Slider),
                "Slider"
            }
            Toggle {
                value: *current_page.read() == PageTypes::TextInput,
                on_value_change: move |_| current_page.set(PageTypes::TextInput),
                "Text Input"
            }
            Toggle {
                value: *current_page.read() == PageTypes::Toast,
                on_value_change: move |_| current_page.set(PageTypes::Toast),
                "Toast"
            }
            Toggle {
                value: *current_page.read() == PageTypes::Toggle,
                on_value_change: move |_| current_page.set(PageTypes::Toggle),
                "Toggle"
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
                    PageTypes::ContextMenu => rsx! {
                        ContextMenuPage {}
                    },
                    PageTypes::Dialog => rsx! {
                        DialogPage {}
                    },
                    PageTypes::Toast => rsx! {
                        ToastPage {}
                    },
                    PageTypes::BusyOverlay => rsx! {
                        BusyOverlayPage {}
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
                MenuItem {
                    icon: MdSettings,
                    label: "Settings",
                    onclick: |_| debug!("Settings"),
                }
                MenuItem {
                    icon: MdEdit,
                    label: "Edit",
                    onclick: |_| debug!("Edit"),
                }
                SubMenu { title: "More options", icon: MdMoreVert,
                    MenuItem {
                        icon: MdSettings,
                        label: "Preferences",
                        onclick: |_| debug!("Preferences"),
                    }
                    MenuItem {
                        icon: MdEdit,
                        label: "Advanced settings",
                        onclick: |_| debug!("Advanced settings"),
                    }
                }
            }
        }
    }
}

#[component]
fn ContextMenuPage() -> Element {
    rsx! {
        div { class: "flex flex-col items-start p-4 gap-4 w-full h-full",
            h1 { "Context Menu" }
            ContextMenuArea { class: "grow w-full flex items-center justify-center rounded-sm border border-view-foregroundnormal/20 p-8 select-none",
                p { "Right-click anywhere in this area." }
                ContextMenu {
                    MenuItem {
                        icon: MdSettings,
                        label: "Settings",
                        onclick: |_| debug!("Settings"),
                    }
                    MenuItem {
                        icon: MdEdit,
                        label: "Edit",
                        onclick: |_| debug!("Edit"),
                    }
                    SubMenu { title: "More options", icon: MdMoreVert,
                        MenuItem {
                            icon: MdSettings,
                            label: "Preferences",
                            onclick: |_| debug!("Preferences"),
                        }
                        MenuItem {
                            icon: MdEdit,
                            label: "Advanced settings",
                            onclick: |_| debug!("Advanced settings"),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogPage() -> Element {
    let mut open = use_signal(|| Some(false));
    let mut confirmed = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col items-start p-4 gap-4",
            h1 { "Dialog" }
            h2 { "Dismissible dialog" }
            p { "Open a dialog, then close it with a button or by clicking the backdrop." }
            Dialog {
                DialogTrigger {
                    Button { "Open dialog" }
                }
                DialogContent { allow_dismiss: true,
                    DialogHeader {
                        DialogTitle { "Welcome to the gallery" }
                        DialogDescription { "Dialogs display content above the current page." }
                    }
                    p { "Use the header, description, body, and footer to organize your content." }
                    DialogFooter {
                        DialogClose {
                            Button { variant: ButtonVariant::Outline, "Close" }
                        }
                    }
                }
            }
            h2 { "Controlled confirmation" }
            p { "This compact dialog uses a signal to manage its open state." }
            Dialog { open, on_open_change: move |value| open.set(Some(value)),
                Button {
                    onclick: move |_| {
                        confirmed.set(false);
                        open.set(Some(true));
                    },
                    "Open confirmation"
                }
                DialogContent { full_width: false,
                    DialogHeader {
                        DialogTitle { "Confirm action" }
                        DialogDescription { "The backdrop does not dismiss this dialog." }
                    }
                    p { "Confirm to update the result shown on this page." }
                    DialogFooter {
                        DialogClose {
                            Button { variant: ButtonVariant::Outline, "Cancel" }
                        }
                        Button {
                            onclick: move |_| {
                                confirmed.set(true);
                                open.set(Some(false));
                            },
                            "Confirm"
                        }
                    }
                }
            }
            p {
                if confirmed() {
                    "Action confirmed."
                } else {
                    "No action confirmed."
                }
            }
        }
    }
}

#[component]
fn ToastPage() -> Element {
    let mut message = use_signal(|| Some("This is a toast message.".to_string()));
    let mut toast = use_toast();

    rsx! {
        div { class: "flex flex-col items-start p-4 gap-4 w-full",
            h1 { "Toast" }
            label { r#for: "toast-message", "Message" }
            TextArea {
                id: "toast-message",
                placeholder: "Enter a toast message",
                value: message,
                on_value_change: move |value| message.set(Some(value)),
            }
            div { class: "flex flex-wrap gap-1",
                Button { onclick: move |_| toast.info(message.read().clone().unwrap_or_default()),
                    "Info"
                }
                Button { onclick: move |_| toast.success(message.read().clone().unwrap_or_default()),
                    "Success"
                }
                Button { onclick: move |_| toast.error(message.read().clone().unwrap_or_default()),
                    "Error"
                }
            }
        }
    }
}

#[component]
fn BusyOverlayPage() -> Element {
    let mut busy_status = use_signal_sync(|| None::<BusyStatus>);

    rsx! {
        div { class: "flex flex-col items-start p-4 gap-4",
            h1 { "Busy Overlay" }
            p {
                "Show a blocking overlay with a spinner or progress indicator. Each demo closes automatically."
            }
            div { class: "flex flex-wrap gap-2",
                Button {
                    onclick: move |_| async move {
                        busy_status
                            .set(
                                Some(BusyStatus {
                                    title: "Loading",
                                    message: "Please wait while the demo loads…".into(),
                                    percentage: None,
                                }),
                            );
                        tabi_ui::sleep(3000).await;
                        busy_status.set(None);
                    },
                    "Show spinner"
                }
                Button {
                    onclick: move |_| async move {
                        for step in 0..=10 {
                            busy_status
                                .set(
                                    Some(BusyStatus {
                                        title: "Processing",
                                        message: format!("Progress: {}%", step * 10),
                                        percentage: Some(f64::from(step) / 10.0),
                                    }),
                                );
                            tabi_ui::sleep(500).await;
                        }
                        busy_status.set(None);
                    },
                    "Show progress"
                }
            }
        }
        BusyOverlay { busy_status }
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
    use tabi_ui::ThemeContext;

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
