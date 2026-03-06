use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxSwitch = mod.widgets.CheckBox{
        width: 52. height: 32.
        text: ""
        draw_check: {
            check_type: Toggle
            size: 14.0
        }
        draw_icon +: {
            color: uniform(#10A37F)
        }
    }
}

pub fn apply_switch_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}
