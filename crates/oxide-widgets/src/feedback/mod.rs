use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxProgress = mod.widgets.View{
        width: Fill height: 6.
        show_bg: true
        draw_bg +: {
            color: uniform(#EAEBEE)
            border_radius: uniform(3.0)
        }
        fill := RoundedView{
            width: 120. height: Fill
            draw_bg +: {
                color: uniform(#10A37F)
                border_radius: uniform(3.0)
            }
        }
    }

    mod.widgets.OxAlert = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 12.
        draw_bg +: {
            color: uniform(#EFF6FF)
            border_radius: uniform(8.0)
        }
        msg := Label{
            draw_text +: {
                color: uniform(#2041B0)
                text_style +: { font_size: 14. }
            }
            text: "Info alert message"
        }
    }

    mod.widgets.OxAlertSuccess = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 12.
        draw_bg +: {
            color: uniform(#ECFDF5)
            border_radius: uniform(8.0)
        }
        msg := Label{
            draw_text +: {
                color: uniform(#065F46)
                text_style +: { font_size: 14. }
            }
            text: "Success!"
        }
    }

    mod.widgets.OxAlertWarning = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 12.
        draw_bg +: {
            color: uniform(#FFFBEB)
            border_radius: uniform(8.0)
        }
        msg := Label{
            draw_text +: {
                color: uniform(#93410F)
                text_style +: { font_size: 14. }
            }
            text: "Warning"
        }
    }

    mod.widgets.OxAlertError = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 12.
        draw_bg +: {
            color: uniform(#FFF2F2)
            border_radius: uniform(8.0)
        }
        msg := Label{
            draw_text +: {
                color: uniform(#991B1B)
                text_style +: { font_size: 14. }
            }
            text: "Error"
        }
    }

    mod.widgets.OxSkeleton = mod.widgets.RoundedView{
        width: Fill height: 16.
        draw_bg +: {
            color: uniform(#EAEBEE)
            border_radius: uniform(4.0)
        }
    }

    mod.widgets.OxSkeletonCircle = mod.widgets.RoundedView{
        width: 40. height: 40.
        draw_bg +: {
            color: uniform(#EAEBEE)
            border_radius: uniform(20.0)
        }
    }

    mod.widgets.OxSkeletonText = mod.widgets.RoundedView{
        width: Fill height: 12.
        draw_bg +: {
            color: uniform(#EAEBEE)
            border_radius: uniform(3.0)
        }
    }
}

pub fn apply_progress_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.border_default));
    let fill = widget.widget(cx, &[id!(fill)]);
    let fill_area = fill.area();
    set_widget_draw_uniform(
        cx,
        fill_area,
        live_id!(color),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}

fn apply_alert_common(cx: &mut Cx, widget: &WidgetRef, theme: &Theme, accent: Vec4) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &[accent.x, accent.y, accent.z, 0.1],
    );
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_alert_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_alert_common(cx, widget, theme, theme.colors.feedback_info);
}

pub fn apply_alert_success_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_alert_common(cx, widget, theme, theme.colors.feedback_success);
}

pub fn apply_alert_warning_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_alert_common(cx, widget, theme, theme.colors.feedback_warning);
}

pub fn apply_alert_error_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_alert_common(cx, widget, theme, theme.colors.feedback_error);
}

pub fn apply_skeleton_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.surface_tertiary),
    );
    widget.redraw(cx);
}

pub fn apply_skeleton_circle_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.surface_tertiary),
    );
    widget.redraw(cx);
}

pub fn apply_skeleton_text_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.surface_tertiary),
    );
    widget.redraw(cx);
}
