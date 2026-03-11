mod button;
mod button_group;
mod icon_button;
mod toggle_button;

use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    button::script_mod(vm);
    icon_button::script_mod(vm);
    button_group::script_mod(vm);
    toggle_button::script_mod(vm);
}

pub use button::{
    apply_button_danger_theme, apply_button_ghost_theme, apply_button_large_theme,
    apply_button_outline_theme, apply_button_secondary_theme, apply_button_small_theme,
    apply_button_theme,
};
pub use icon_button::apply_icon_button_theme;
pub use toggle_button::apply_toggle_button_theme;
