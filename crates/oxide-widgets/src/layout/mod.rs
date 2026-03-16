/// Layout widgets for OxideUI.
///
/// OxCard and OxDivider are fully functional. OxStack and OxGrid are layout placeholders
/// (**Status: Planned**) -- OxStack is a simple vertical flow and OxGrid is a horizontal
/// flow. Future versions will add responsive column counts, gap control, and wrap behavior.
use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxCard = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 24.
        flow: Down spacing: 16.
        draw_bg +: {
            color: #FFFFFF
            border_radius: 8.0
            border_size: 1.0
            border_color: #E5E5E5
        }
    }

    mod.widgets.OxDivider = mod.widgets.SolidView{
        width: Fill height: 1.
        draw_bg +: {
            color: #E5E5E5
        }
    }

    mod.widgets.OxStack = mod.widgets.View{
        width: Fill height: Fit
        flow: Down spacing: 16.
    }

    mod.widgets.OxGrid = mod.widgets.View{
        width: Fill height: Fit
        flow: Right spacing: 16.
    }
}

pub fn apply_card_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_primary));
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(border_color),
        &v4(theme.colors.border_default),
    );
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.lg as f32]);
    widget.redraw(cx);
}

pub fn apply_divider_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.border_default));
    widget.redraw(cx);
}
