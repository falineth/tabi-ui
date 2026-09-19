mod dropdown_menu;
mod menu_item;
mod menu_list;

use dioxus::signals::Signal;
pub use dropdown_menu::*;
pub use menu_item::*;
pub use menu_list::*;

#[derive(Clone)]
struct MenuContext {
    pub open: Signal<bool>,
}
