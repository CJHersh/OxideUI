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
            progress: instance(0.5)
            track_color: uniform(#E5E5E5)
            fill_color: uniform(#171717)
            border_radius: uniform(3.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let sz = self.rect_size
                let r = self.border_radius

                sdf.box(0.0, 0.0, sz.x, sz.y, r)
                sdf.fill(self.track_color)

                let fill_w = sz.x * clamp(self.progress, 0.0, 1.0)
                if fill_w > 0.5 {
                    let sdf2 = Sdf2d.viewport(self.pos * self.rect_size)
                    sdf2.box(0.0, 0.0, fill_w, sz.y, r)
                    sdf2.fill(self.fill_color)

                    let px = self.pos.x * sz.x
                    let in_fill = step(px, fill_w)
                    return mix(sdf.result, sdf2.result, in_fill * sdf2.result.w)
                }

                return sdf.result
            }
        }
    }

    mod.widgets.OxAlert = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 12.
        draw_bg +: {
            color: #EFF6FF
            border_radius: 6.0
        }
        msg := Label{
            draw_text +: {
                color: #1E40AF
                text_style +: { font_size: 14. }
            }
            text: "Info alert message"
        }
    }

    mod.widgets.OxAlertSuccess = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 12.
        draw_bg +: {
            color: #F0FDF4
            border_radius: 6.0
        }
        msg := Label{
            draw_text +: {
                color: #166534
                text_style +: { font_size: 14. }
            }
            text: "Success!"
        }
    }

    mod.widgets.OxAlertWarning = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 12.
        draw_bg +: {
            color: #FFFBEB
            border_radius: 6.0
        }
        msg := Label{
            draw_text +: {
                color: #924011
                text_style +: { font_size: 14. }
            }
            text: "Warning"
        }
    }

    mod.widgets.OxAlertError = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 12.
        draw_bg +: {
            color: #FEF2F2
            border_radius: 6.0
        }
        msg := Label{
            draw_text +: {
                color: #991B1B
                text_style +: { font_size: 14. }
            }
            text: "Error"
        }
    }

    mod.widgets.OxSkeleton = mod.widgets.RoundedView{
        width: Fill height: 16.
        draw_bg +: {
            color: #F5F5F5
            border_radius: 4.0
        }
    }

    mod.widgets.OxSkeletonCircle = mod.widgets.RoundedView{
        width: 40. height: 40.
        draw_bg +: {
            color: #F5F5F5
            border_radius: 20.0
        }
    }

    mod.widgets.OxSkeletonText = mod.widgets.RoundedView{
        width: Fill height: 12.
        draw_bg +: {
            color: #F5F5F5
            border_radius: 3.0
        }
    }
}

pub fn apply_progress_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(track_color),
        &v4(theme.colors.border_default),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(fill_color),
        &v4(theme.colors.interactive_default),
    );
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.sm as f32]);
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
