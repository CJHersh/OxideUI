use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxIcon = mod.widgets.View{
        width: 24. height: 24.
        align: Center
        show_bg: true
        draw_bg: {
            uniform icon_type: 0.0
            uniform icon_color: #0A0A0AFF

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let cx = self.rect_size.x * 0.5;
                let cy = self.rect_size.y * 0.5;
                let s = min(self.rect_size.x, self.rect_size.y);
                let lw = s * 0.08;
                let m = s * 0.2;

                let t = self.icon_type;

                // 0: check
                if t < 0.5 {
                    sdf.move_to(m, cy);
                    sdf.line_to(cx * 0.85, cy + m * 0.8);
                    sdf.line_to(s - m, cy - m * 0.6);
                    sdf.stroke(self.icon_color, lw);
                }
                // 1: x / close
                else if t < 1.5 {
                    sdf.move_to(m, m);
                    sdf.line_to(s - m, s - m);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(s - m, m);
                    sdf.line_to(m, s - m);
                    sdf.stroke(self.icon_color, lw);
                }
                // 2: chevron-right
                else if t < 2.5 {
                    sdf.move_to(cx - m * 0.4, m);
                    sdf.line_to(cx + m * 0.4, cy);
                    sdf.line_to(cx - m * 0.4, s - m);
                    sdf.stroke(self.icon_color, lw);
                }
                // 3: chevron-left
                else if t < 3.5 {
                    sdf.move_to(cx + m * 0.4, m);
                    sdf.line_to(cx - m * 0.4, cy);
                    sdf.line_to(cx + m * 0.4, s - m);
                    sdf.stroke(self.icon_color, lw);
                }
                // 4: chevron-down
                else if t < 4.5 {
                    sdf.move_to(m, cy - m * 0.4);
                    sdf.line_to(cx, cy + m * 0.4);
                    sdf.line_to(s - m, cy - m * 0.4);
                    sdf.stroke(self.icon_color, lw);
                }
                // 5: chevron-up
                else if t < 5.5 {
                    sdf.move_to(m, cy + m * 0.4);
                    sdf.line_to(cx, cy - m * 0.4);
                    sdf.line_to(s - m, cy + m * 0.4);
                    sdf.stroke(self.icon_color, lw);
                }
                // 6: plus
                else if t < 6.5 {
                    sdf.move_to(cx, m);
                    sdf.line_to(cx, s - m);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(m, cy);
                    sdf.line_to(s - m, cy);
                    sdf.stroke(self.icon_color, lw);
                }
                // 7: minus
                else if t < 7.5 {
                    sdf.move_to(m, cy);
                    sdf.line_to(s - m, cy);
                    sdf.stroke(self.icon_color, lw);
                }
                // 8: search
                else if t < 8.5 {
                    let r = s * 0.25;
                    let ox = cx - s * 0.05;
                    let oy = cy - s * 0.05;
                    sdf.circle(ox, oy, r);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(ox + r * 0.7, oy + r * 0.7);
                    sdf.line_to(s - m, s - m);
                    sdf.stroke(self.icon_color, lw);
                }
                // 9: menu (hamburger)
                else if t < 9.5 {
                    let gap = s * 0.22;
                    sdf.move_to(m, cy - gap);
                    sdf.line_to(s - m, cy - gap);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(m, cy);
                    sdf.line_to(s - m, cy);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(m, cy + gap);
                    sdf.line_to(s - m, cy + gap);
                    sdf.stroke(self.icon_color, lw);
                }
                // 10: alert-circle
                else if t < 10.5 {
                    let r = s * 0.38;
                    sdf.circle(cx, cy, r);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(cx, cy - r * 0.5);
                    sdf.line_to(cx, cy + r * 0.1);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.circle(cx, cy + r * 0.4, lw * 0.8);
                    sdf.fill(self.icon_color);
                }
                // 11: info
                else if t < 11.5 {
                    let r = s * 0.38;
                    sdf.circle(cx, cy, r);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.circle(cx, cy - r * 0.4, lw * 0.8);
                    sdf.fill_keep(self.icon_color);
                    sdf.move_to(cx, cy - r * 0.1);
                    sdf.line_to(cx, cy + r * 0.5);
                    sdf.stroke(self.icon_color, lw);
                }
                // 12: arrow-right
                else if t < 12.5 {
                    sdf.move_to(m, cy);
                    sdf.line_to(s - m, cy);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(s - m - s * 0.15, cy - s * 0.15);
                    sdf.line_to(s - m, cy);
                    sdf.line_to(s - m - s * 0.15, cy + s * 0.15);
                    sdf.stroke(self.icon_color, lw);
                }
                // 13: arrow-left
                else if t < 13.5 {
                    sdf.move_to(s - m, cy);
                    sdf.line_to(m, cy);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(m + s * 0.15, cy - s * 0.15);
                    sdf.line_to(m, cy);
                    sdf.line_to(m + s * 0.15, cy + s * 0.15);
                    sdf.stroke(self.icon_color, lw);
                }
                // 14: eye
                else if t < 14.5 {
                    let ew = s * 0.35;
                    let eh = s * 0.18;
                    sdf.move_to(m, cy);
                    sdf.line_to(cx, cy - eh);
                    sdf.line_to(s - m, cy);
                    sdf.line_to(cx, cy + eh);
                    sdf.close_path();
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.circle(cx, cy, s * 0.08);
                    sdf.fill(self.icon_color);
                }
                // 15: copy
                else if t < 15.5 {
                    let bw = s * 0.35;
                    let off = s * 0.12;
                    sdf.box(m + off, m, bw, bw, lw);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.box(m, m + off, bw, bw, lw);
                    sdf.stroke(self.icon_color, lw);
                }
                // 16: settings (gear)
                else if t < 16.5 {
                    sdf.circle(cx, cy, s * 0.14);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.circle(cx, cy, s * 0.32);
                    sdf.stroke(self.icon_color, lw);
                }
                // 17: star
                else if t < 17.5 {
                    let r = s * 0.35;
                    sdf.move_to(cx, m);
                    sdf.line_to(cx + r * 0.3, cy - r * 0.1);
                    sdf.line_to(s - m, cy - r * 0.1);
                    sdf.line_to(cx + r * 0.45, cy + r * 0.2);
                    sdf.line_to(cx + r * 0.6, s - m);
                    sdf.line_to(cx, cy + r * 0.5);
                    sdf.line_to(cx - r * 0.6, s - m);
                    sdf.line_to(cx - r * 0.45, cy + r * 0.2);
                    sdf.line_to(m, cy - r * 0.1);
                    sdf.line_to(cx - r * 0.3, cy - r * 0.1);
                    sdf.close_path();
                    sdf.stroke(self.icon_color, lw);
                }
                // 18: heart
                else if t < 18.5 {
                    sdf.circle(cx - s * 0.12, cy - s * 0.06, s * 0.16);
                    sdf.fill_keep(self.icon_color);
                    sdf.circle(cx + s * 0.12, cy - s * 0.06, s * 0.16);
                    sdf.fill_keep(self.icon_color);
                    sdf.move_to(m + s * 0.05, cy);
                    sdf.line_to(cx, s - m - s * 0.05);
                    sdf.line_to(s - m - s * 0.05, cy);
                    sdf.close_path();
                    sdf.fill(self.icon_color);
                }
                // 19: external-link
                else if t < 19.5 {
                    sdf.box(m, m + s * 0.15, s * 0.55, s * 0.55, lw);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(cx, m);
                    sdf.line_to(s - m, m);
                    sdf.line_to(s - m, cy);
                    sdf.stroke_keep(self.icon_color, lw);
                    sdf.move_to(s - m, m);
                    sdf.line_to(cx - s * 0.05, cy + s * 0.05);
                    sdf.stroke(self.icon_color, lw);
                }
                // default: empty
                else {
                }

                return sdf.result;
            }
        }
    }

    mod.widgets.OxIconSmall = mod.widgets.OxIcon{
        width: 16. height: 16.
    }

    mod.widgets.OxIconLarge = mod.widgets.OxIcon{
        width: 32. height: 32.
    }
}

/// Icon type constants for use with `icon_type` uniform.
pub mod icons {
    pub const CHECK: f32 = 0.0;
    pub const CLOSE: f32 = 1.0;
    pub const CHEVRON_RIGHT: f32 = 2.0;
    pub const CHEVRON_LEFT: f32 = 3.0;
    pub const CHEVRON_DOWN: f32 = 4.0;
    pub const CHEVRON_UP: f32 = 5.0;
    pub const PLUS: f32 = 6.0;
    pub const MINUS: f32 = 7.0;
    pub const SEARCH: f32 = 8.0;
    pub const MENU: f32 = 9.0;
    pub const ALERT_CIRCLE: f32 = 10.0;
    pub const INFO: f32 = 11.0;
    pub const ARROW_RIGHT: f32 = 12.0;
    pub const ARROW_LEFT: f32 = 13.0;
    pub const EYE: f32 = 14.0;
    pub const COPY: f32 = 15.0;
    pub const SETTINGS: f32 = 16.0;
    pub const STAR: f32 = 17.0;
    pub const HEART: f32 = 18.0;
    pub const EXTERNAL_LINK: f32 = 19.0;
}

pub fn apply_icon_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(icon_color),
        &v4(theme.colors.text_primary),
    );
    widget.redraw(cx);
}

pub fn apply_icon_accent_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(icon_color),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}

pub fn apply_icon_secondary_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(icon_color),
        &v4(theme.colors.text_secondary),
    );
    widget.redraw(cx);
}
