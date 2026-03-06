use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxToggleButton = mod.widgets.ButtonFlat{
        text: "Toggle"
        draw_bg +: {
            color: uniform(#EAEBEEFF)
            color_hover: uniform(#D1D3D8FF)
            color_down: uniform(#EAEBEEFF)
            border_radius: uniform(8.0)
        }
        draw_text +: {
            color: uniform(#333333)
            color_hover: uniform(#333333)
        }
    }
}

pub fn apply_toggle_button_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.surface_tertiary),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_hover),
        &v4(theme.colors.border_hover),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_down),
        &v4(theme.colors.border_default),
    );
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}
