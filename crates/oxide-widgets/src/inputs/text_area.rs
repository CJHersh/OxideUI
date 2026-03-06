use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxTextArea = mod.widgets.TextInput{
        width: Fill height: 120.
        is_multiline: true
        empty_text: "Type here..."
        draw_bg +: {
            border_radius: uniform(8.0)
            border_size: uniform(1.0)
            border_color: uniform(#EAEBEE)
            border_color_focus: uniform(#10A37F)
            color: uniform(#FFFFFF)
        }
        draw_text +: {
            color: uniform(#202023)
            text_style +: { font_size: 14. }
        }
    }
}

pub fn apply_text_area_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
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
    widget.redraw(cx);
}
