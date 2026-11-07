use dioxus::html::geometry::Pixels;
use dioxus::html::geometry::euclid::Rect;
use dioxus::prelude::*;

use crate::components::PortalId;
use crate::utils::get_now;

#[derive(Clone)]
pub struct OptionState<T: Clone + PartialEq + 'static> {
    pub index: usize,
    pub value: Option<T>,
    pub text_value: String,
    pub id: String,
}

pub struct SearchText {
    pub text: String,
    pub age: u128,
}

impl Default for SearchText {
    fn default() -> Self {
        Self {
            text: Default::default(),
            age: get_now(),
        }
    }
}

#[derive(Clone)]
pub struct SelectContext<T: Clone + PartialEq + 'static> {
    pub active_value: Signal<Option<OptionState<T>>>,
    pub value: Memo<Option<T>>,
    pub set_value: Callback<Option<T>>,
    pub options: Signal<Vec<OptionState<T>>>,
    pub menu_open: ReadSignal<bool>,
    pub set_menu_open: Callback<bool>,
    pub list_id: Signal<Option<String>>,
    pub combo_rect: Signal<Rect<f64, Pixels>>,
    pub search_string: Signal<SearchText>,
    pub list_height: Signal<Option<f64>>,
}
