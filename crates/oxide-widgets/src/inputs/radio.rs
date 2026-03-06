use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxRadio = mod.widgets.RadioButton{
        text: "Radio"
        draw_radio: {
            radio_type: Round
            size: 8.0
        }
        draw_bg +: {
            color: uniform(#10A37F)
        }
        draw_text +: {
            color: uniform(#202023)
            text_style +: { font_size: 14. }
        }
    }
}

pub fn apply_radio_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}
