//! Multi-skin UI component library for OxideUI powered by Makepad.

pub mod buttons;
pub mod data;
pub mod display;
pub mod feedback;
pub mod inputs;
pub mod layout;
pub mod navigation;
pub mod overlay;
pub mod theme_apply;

pub use makepad_widgets;
pub use oxide_core;

pub mod prelude {
    pub use oxide_core::prelude::*;
}

use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    oxide_core::script_mod(vm);
    buttons::script_mod(vm);
    inputs::script_mod(vm);
    display::script_mod(vm);
    feedback::script_mod(vm);
    overlay::script_mod(vm);
    navigation::script_mod(vm);
    layout::script_mod(vm);
    data::script_mod(vm);
}

pub use theme_apply::{apply_theme, ThemeMap, ThemeTarget};
