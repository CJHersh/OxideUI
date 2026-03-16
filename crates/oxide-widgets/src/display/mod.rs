mod avatar;
mod badge;
mod icon;
mod label;

use makepad_widgets::ScriptVm;

pub fn script_mod(vm: &mut ScriptVm) {
    label::script_mod(vm);
    badge::script_mod(vm);
    avatar::script_mod(vm);
    icon::script_mod(vm);
}

pub use avatar::apply_avatar_theme;
pub use badge::{
    apply_badge_error_theme, apply_badge_info_theme, apply_badge_success_theme, apply_badge_theme,
    apply_badge_warning_theme,
};
pub use icon::{apply_icon_accent_theme, apply_icon_secondary_theme, apply_icon_theme};
pub use label::{
    apply_label_body_theme, apply_label_caption_theme, apply_label_link_theme,
    apply_label_secondary_theme, apply_label_subtitle_theme, apply_label_theme,
    apply_label_title_theme,
};
