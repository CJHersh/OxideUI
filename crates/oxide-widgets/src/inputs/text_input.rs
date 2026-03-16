/// OxTextInput renders a single-line text input with shadcn styling.
///
/// Default values below match the shadcn light theme from
/// `oxide_core::theme::themes::shadcn`. The `apply_text_input_theme` function
/// overrides these at runtime for theme switching.
use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxTextInput = mod.widgets.TextInput{
        width: Fill height: Fit
        empty_text: "Type here..."
        draw_bg +: {
            border_radius: 6.0
            border_size: 1.0
            border_color: #E5E5E5
            border_color_focus: #D4D4D4
            border_color_error: uniform(#EF4444)
            color: #FFFFFF
        }
        draw_text +: {
            color: #0A0A0A
            text_style +: { font_size: 14. }
        }
    }
}

pub fn apply_text_input_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_primary));
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(border_color),
        &v4(theme.colors.border_default),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(border_color_focus),
        &v4(theme.colors.border_focus),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(border_color_error),
        &v4(theme.colors.border_error),
    );
    widget.redraw(cx);
}
