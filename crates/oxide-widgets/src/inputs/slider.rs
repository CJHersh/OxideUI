use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxSlider = mod.widgets.Slider{
        width: Fill height: 40.
        draw_bg +: {
            color: uniform(#E5E5E5)
        }
        draw_slider +: {
            color: uniform(#171717)
        }
    }
}

pub fn apply_slider_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}
