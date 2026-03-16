use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxCheckbox = mod.widgets.CheckBox{
        text: "Checkbox"
        draw_icon +: {
            color: #171717
        }
        draw_text +: {
            color: #0A0A0A
            text_style +: { font_size: 14. }
        }
    }
}

pub fn apply_checkbox_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}
