mod checkbox;
mod radio;
mod slider;
mod switch;
mod text_area;
mod text_input;

use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    text_input::script_mod(vm);
    text_area::script_mod(vm);
    checkbox::script_mod(vm);
    radio::script_mod(vm);
    switch::script_mod(vm);
    slider::script_mod(vm);
}

pub use checkbox::apply_checkbox_theme;
pub use radio::apply_radio_theme;
pub use slider::apply_slider_theme;
pub use switch::apply_switch_theme;
pub use text_area::apply_text_area_theme;
pub use text_input::apply_text_input_theme;
