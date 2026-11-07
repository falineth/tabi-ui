use std::rc::Rc;
use std::time::Duration;

use dioxus::html::geometry::Pixels;
use dioxus::html::geometry::euclid::Rect;
use dioxus::prelude::*;

use crate::components::SelectContext;
use crate::hooks::use_window_size;
use crate::{get_now, sleep};

#[allow(unused)]
#[derive(Debug)]
enum SelectAction {
    Close,
    CloseSelect,
    First,
    Last,
    Next,
    Open,
    Previous,
    Select,
    Type(String),
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes, extends = button)]
    rest: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn SelectTrigger<T: Clone + PartialEq + 'static>(props: SelectTriggerProps) -> Element {
    /*

        Props

    */

    let SelectTriggerProps {
        class,
        rest,
        children,
    } = props;

    let window_size = use_window_size();

    /*

        Refs

    */

    let mut combo_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    /*

        Context

    */

    let mut ctx = use_context::<SelectContext<T>>();

    /*

       Callbacks

    */

    let handle_toggle_open = use_callback(move |_: ()| {
        let new_open_state = !*ctx.menu_open.read();

        if ctx.active_value.read().is_none()
            && let Some(first) = ctx.options.read().iter().min_by_key(|option| option.index)
        {
            ctx.active_value.set(Some(first.clone()));
        }

        if new_open_state && combo_element.read().is_some() && window_size.read().height > -1.0 {
            spawn(async move {
                if let Some(combo_element) = combo_element.read().as_ref()
                    && let Ok(rect) = combo_element.get_client_rect().await
                {
                    let current_rect = *ctx.combo_rect.read();

                    if current_rect != rect {
                        ctx.combo_rect.set(rect);
                    }
                }

                ctx.set_menu_open.call(new_open_state);
            });

            return;
        }

        ctx.set_menu_open.call(new_open_state);
    });

    let do_action = use_callback(move |params: (SelectAction, Event<KeyboardData>)| {
        let (action, event) = params;

        let active_value = ctx.active_value.read().cloned();

        if matches!(action, SelectAction::Last | SelectAction::First) {
            ctx.set_menu_open.call(true);
        }

        match action {
            SelectAction::First => {
                event.prevent_default();

                let Some(active_value) = active_value else {
                    return;
                };

                if let Some(first) = ctx.options.read().iter().min_by_key(|option| option.index)
                    && first.index != active_value.index
                {
                    ctx.active_value.set(Some(first.clone()));
                }
            }
            SelectAction::Previous => {
                event.prevent_default();

                let Some(active_value) = active_value else {
                    return;
                };

                if let Some(prev) = ctx
                    .options
                    .read()
                    .iter()
                    .filter(|option| option.index < active_value.index)
                    .max_by_key(|option| option.index)
                {
                    ctx.active_value.set(Some(prev.clone()));
                }
            }
            SelectAction::Next => {
                event.prevent_default();

                let Some(active_value) = active_value else {
                    return;
                };

                if let Some(next) = ctx
                    .options
                    .read()
                    .iter()
                    .filter(|option| option.index > active_value.index)
                    .min_by_key(|option| option.index)
                {
                    ctx.active_value.set(Some(next.clone()));
                }
            }
            SelectAction::Last => {
                event.prevent_default();

                let Some(active_value) = active_value else {
                    return;
                };

                if let Some(last) = ctx.options.read().iter().max_by_key(|option| option.index)
                    && last.index != active_value.index
                {
                    ctx.active_value.set(Some(last.clone()));
                }
            }
            SelectAction::CloseSelect => {
                event.prevent_default();

                if let Some(active_value) = active_value {
                    ctx.set_value.call(active_value.value.clone());
                }

                ctx.set_menu_open.call(false);
            }
            SelectAction::Close => {
                event.prevent_default();
                ctx.set_menu_open.call(false);
            }
            SelectAction::Type(text) => {
                ctx.set_menu_open.call(true);

                let mut search_string = ctx.search_string.write();

                if get_now() - search_string.age > 500 {
                    search_string.text.clear();
                }

                let lowercase_text = text.to_lowercase();

                search_string.text.push_str(&lowercase_text);
                search_string.age = get_now();

                let matching_option = ctx
                    .options
                    .read()
                    .iter()
                    .filter(|option| {
                        option
                            .text_value
                            .to_lowercase()
                            .starts_with(search_string.text.as_str())
                    })
                    .min_by_key(|option| option.index)
                    .cloned();

                if let Some(matching_option) = matching_option {
                    let matching_option = Some(matching_option.clone());

                    ctx.active_value.set(matching_option);
                } else {
                    search_string.text = lowercase_text;
                }
            }
            SelectAction::Open => {
                event.prevent_default();
                handle_toggle_open(());
            }
            _ => {}
        }
    });

    let handle_blur = use_callback(move |_| {
        spawn(async move {
            // HACK: is there a way to avoid an arbitrary wait
            // to prevent blur from preventing clicking on select options.
            sleep(200).await;
            ctx.set_menu_open.call(false);
        });
    });

    rsx! {
        button {
            role: "combobox",
            class: "bg-button-backgroundnormal text-button-foregroundnormal border-button-foregroundnormal/20 data-placeholder:text-button-foregroundinactive",
            class: "hover:border-button-decorationhover/80",
            class: "focus-visible:ring-button-decorationfocus/5 focus-visible:border-button-decorationfocus/80",
            class: "aria-expanded:border-button-decorationfocus/80 aria-expanded:bg-[color-mix(in_srgb,var(--color-button-decorationfocus)_25%,var(--color-button-backgroundnormal))]",
            class: "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50",
            class: "gap-1.5 rounded-sm border px-2 py-1.5 text-xs/relaxed transition-colors focus-visible:ring-2 aria-invalid:ring-2 data-[size=default]:h-7 data-[size=sm]:h-6 *:data-[slot=select-value]:flex *:data-[slot=select-value]:gap-1.5 [&_svg:not([class*='size-'])]:size-3.5 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:items-center [&_svg]:pointer-events-none [&_svg]:shrink-0",
            class: "{class}",
            aria_controls: ctx.list_id,
            aria_expanded: ctx.menu_open,
            aria_haspopup: "listbox",
            aria_activedescendant: if let Some(active_value) = ctx.active_value.read().as_ref() { active_value.id.clone() },
            "data-placeholder": if ctx.value.read().is_none() { "" },
            "data-size": "default",
            "data-state": if *ctx.menu_open.read() { "open" } else { "closed" },
            onmounted: move |e| {
                combo_element.set(Some(e.data()));
            },
            onblur: handle_blur,
            onkeydown: move |e| {
                let action = map_key_to_action(e.key(), e.modifiers(), *ctx.menu_open.read());

                if let Some(action) = action {
                    do_action.call((action, e));
                }
            },
            onclick: move |_| {
                handle_toggle_open.call(());
            },
            ..rest,
            {children}
        }
    }
}

fn map_key_to_action(key: Key, modifiers: Modifiers, menu_open: bool) -> Option<SelectAction> {
    let is_space = match &key {
        Key::Character(characters) => matches!(characters.as_str(), " "),
        _ => false,
    };

    if !menu_open && (is_space || matches!(key, Key::ArrowDown | Key::ArrowUp | Key::Enter)) {
        return Some(SelectAction::Open);
    }

    match key {
        Key::Home => return Some(SelectAction::First),
        Key::End => return Some(SelectAction::Last),
        _ => {}
    }

    if modifiers.is_empty()
        && !is_space
        && let Key::Character(text) = key
    {
        return Some(SelectAction::Type(text));
    }

    if menu_open {
        if matches!(key, Key::ArrowUp) && modifiers.alt() {
            return Some(SelectAction::CloseSelect);
        }

        if matches!(key, Key::ArrowDown) && !modifiers.alt() {
            return Some(SelectAction::Next);
        }

        if is_space {
            return Some(SelectAction::CloseSelect);
        }

        return match key {
            Key::ArrowUp => Some(SelectAction::Previous),
            Key::Escape => Some(SelectAction::Close),
            Key::Enter => Some(SelectAction::CloseSelect),
            _ => None,
        };
    }

    return None;
}
