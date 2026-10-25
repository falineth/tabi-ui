use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use dioxus::prelude::*;

use crate::{ThemeContext, ThemeStyle, ToastProvider, WindowSize};

/// # TabiDefaultContext
///
/// The [`TabiDefaultContext`] component is a container that simplifies bringing in
/// contexts and utilities used by Tabi components.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use tabi_ui::*;
///
/// #[derive(Debug, Clone, Routable, PartialEq)]
/// enum Route {
///     #[layout(NavbarView)]
///     #[route("/")]
///     HomeView {},
/// }
///
/// #[component]
/// pub fn HomeView() -> Element {
///     rsx! {}
/// }
///
/// #[component]
/// pub fn NavbarView() -> Element {
///     rsx! {
///         TabiDefaultContext { class: "h-lvh w-lvw overflow-hidden", Outlet::<Route> {} }
///     }
/// }
/// ```
///
/// ## Function
///
/// Delay showing the app content to allow styles to load
///
/// Provides the contexts
///  - WindowSize
///  - ToastContext
///
#[component]
pub fn TabiDefaultContext(#[props(default)] class: String, children: Element) -> Element {
    /*

       State

    */

    let mut loaded = use_signal(bool::default);

    let theme_context = try_use_context::<ThemeContext>();

    /*

        Contexts

    */

    let mut window_size = use_context_provider(|| Signal::new(WindowSize::default()));

    /*

       Memos

    */

    let bg_color = use_memo(move || {
        if let Some(theme_context) = theme_context.as_ref() {
            let (r, g, b, _a) = theme_context.bg_color;
            return format!("background-color: #{r:02x}{g:02x}{b:02x};");
        }

        return String::default();
    });

    let mut outer_frame_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    /*

       Callbacks

    */

    let handle_outer_frame_mounted = use_callback(move |e: Event<MountedData>| {
        outer_frame_element.set(Some(e.data()));
    });

    let handle_resize_event = use_callback(move |_: Event<ResizeData>| {
        let outer_frame_element = outer_frame_element.cloned();

        let Some(outer_frame_element) = outer_frame_element else {
            return;
        };

        spawn(async move {
            let Ok(rect) = outer_frame_element.get_client_rect().await else {
                return;
            };

            window_size.set(WindowSize {
                width: rect.width(),
                height: rect.height(),
            });
        });
    });

    /*

       Futures

    */

    use_future(move || async move {
        #[cfg(not(feature = "web"))]
        {
            _ = tokio::spawn(async move {
                sleep(Duration::from_millis(10));
            })
            .await;
        }

        loaded.set(true);
    });

    rsx! {
        ThemeStyle {}

        if !*loaded.read() {
            div {
                class: "bg-white",
                style: "height: 100lvh; width: 100lvw; {bg_color}",
            }
        }

        div {
            class,
            style: if !*loaded.read() { "display: none;" },
            onmounted: handle_outer_frame_mounted,
            onresize: handle_resize_event,
            ToastProvider { {children} }
        }
    }
}
