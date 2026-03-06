use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::set_widget_draw_uniform;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxIconButton = mod.widgets.ButtonFlat{
        width: 40. height: 40.
        padding: Inset{top: 8., right: 8., bottom: 8., left: 8.}
        align: Center
        draw_bg +: {
            color: uniform(#FFFFFF00)
            color_hover: uniform(#00000010)
            color_down: uniform(#00000020)
            border_radius: uniform(8.0)
        }
    }
}

pub fn apply_icon_button_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}
