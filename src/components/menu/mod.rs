mod context_menu;
mod context_menu_area;
mod dropdown_menu;
mod menu_item;
mod menu_list;
mod sub_menu;

use dioxus::signals::Signal;

pub use crate::components::menu::context_menu::*;
pub use crate::components::menu::context_menu_area::*;
pub use crate::components::menu::dropdown_menu::*;
pub use crate::components::menu::menu_item::*;
pub use crate::components::menu::menu_list::*;
pub use crate::components::menu::sub_menu::*;

#[derive(Clone)]
pub struct MenuContext {
    pub open: Signal<bool>,
}
