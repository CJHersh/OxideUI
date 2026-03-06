use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxBadge = mod.widgets.RoundedView{
        width: Fit height: Fit
        padding: Inset{left: 8., right: 8., top: 4., bottom: 4.}
        draw_bg +: {
            color: uniform(#F0F0F0)
            border_radius: uniform(12.0)
        }
        label := Label{
            draw_text +: {
                color: uniform(#555555)
                text_style +: { font_size: 12. }
            }
            text: "Badge"
        }
    }

    mod.widgets.OxBadgeSuccess = mod.widgets.RoundedView{
        width: Fit height: Fit
        padding: Inset{left: 8., right: 8., top: 4., bottom: 4.}
        draw_bg +: {
            color: uniform(#ECFDF5)
            border_radius: uniform(12.0)
        }
        label := Label{
            draw_text +: {
                color: uniform(#065F46)
                text_style +: { font_size: 12. }
            }
            text: "Success"
        }
    }

    mod.widgets.OxBadgeWarning = mod.widgets.RoundedView{
        width: Fit height: Fit
        padding: Inset{left: 8., right: 8., top: 4., bottom: 4.}
        draw_bg +: {
            color: uniform(#FFFBEB)
            border_radius: uniform(12.0)
        }
        label := Label{
            draw_text +: {
                color: uniform(#93410F)
                text_style +: { font_size: 12. }
            }
            text: "Warning"
        }
    }

    mod.widgets.OxBadgeError = mod.widgets.RoundedView{
        width: Fit height: Fit
        padding: Inset{left: 8., right: 8., top: 4., bottom: 4.}
        draw_bg +: {
            color: uniform(#FFF2F2)
            border_radius: uniform(12.0)
        }
        label := Label{
            draw_text +: {
                color: uniform(#991B1B)
                text_style +: { font_size: 12. }
            }
            text: "Error"
        }
    }

    mod.widgets.OxBadgeInfo = mod.widgets.RoundedView{
        width: Fit height: Fit
        padding: Inset{left: 8., right: 8., top: 4., bottom: 4.}
        draw_bg +: {
            color: uniform(#EFF6FF)
            border_radius: uniform(12.0)
        }
        label := Label{
            draw_text +: {
                color: uniform(#2041B0)
                text_style +: { font_size: 12. }
            }
            text: "Info"
        }
    }
}

pub fn apply_badge_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.surface_tertiary),
    );
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.xl as f32]);
    widget.redraw(cx);
}

pub fn apply_badge_success_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let c = theme.colors.feedback_success;
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &[c.x, c.y, c.z, 0.1]);
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.xl as f32]);
    widget.redraw(cx);
}

pub fn apply_badge_warning_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let c = theme.colors.feedback_warning;
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &[c.x, c.y, c.z, 0.1]);
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.xl as f32]);
    widget.redraw(cx);
}

pub fn apply_badge_error_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let c = theme.colors.feedback_error;
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &[c.x, c.y, c.z, 0.1]);
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.xl as f32]);
    widget.redraw(cx);
}

pub fn apply_badge_info_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let c = theme.colors.feedback_info;
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &[c.x, c.y, c.z, 0.1]);
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.xl as f32]);
    widget.redraw(cx);
}
