use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxButton = mod.widgets.Button{
        text: "Button"
        width: Fit height: Fit
        padding: Inset{top: 10., right: 20., bottom: 10., left: 20.}
        draw_bg +: {
            color: uniform(#10A37F)
            color_hover: uniform(#0E9471)
            color_down: uniform(#0A6B50)
            color_focus: uniform(#10A37F)
            color_disabled: uniform(#B0D8CB)
            border_size: uniform(0.0)
            border_radius: uniform(8.0)
            border_color: uniform(#00000000)
            border_color_hover: uniform(#00000000)
            border_color_down: uniform(#00000000)
            border_color_focus: uniform(#00000000)
            border_color_disabled: uniform(#00000000)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = self.border_radius;

                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, r);

                let c1 = mix(self.color, self.color_focus, self.focus);
                let c2 = mix(c1, self.color_hover, self.hover);
                let c3 = mix(c2, self.color_down, self.down);
                let col = mix(c3, self.color_disabled, self.disabled);

                let g = mix(1.06, 0.94, self.pos.y);
                let shaded = vec4(col.rgb * g, col.a);

                sdf.fill(shaded);
                return sdf.result;
            }
        }
        draw_text +: {
            color: uniform(#FFFFFF)
            color_hover: uniform(#FFFFFF)
            color_down: uniform(#FFFFFFEE)
            color_focus: uniform(#FFFFFF)
            color_disabled: uniform(#FFFFFF80)
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonSecondary = mod.widgets.Button{
        text: "Secondary"
        width: Fit height: Fit
        padding: Inset{top: 10., right: 20., bottom: 10., left: 20.}
        draw_bg +: {
            color: uniform(#FFFFFF)
            color_hover: uniform(#F4F4F5)
            color_down: uniform(#E4E4E7)
            color_focus: uniform(#FFFFFF)
            color_disabled: uniform(#FAFAFA)
            border_size: uniform(1.0)
            border_radius: uniform(8.0)
            border_color: uniform(#D4D4D8)
            border_color_hover: uniform(#A1A1AA)
            border_color_down: uniform(#A1A1AA)
            border_color_focus: uniform(#10A37F80)
            border_color_disabled: uniform(#E4E4E7)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = self.border_radius;
                let bs = self.border_size;

                sdf.box(bs, bs, self.rect_size.x - bs * 2.0, self.rect_size.y - bs * 2.0, max(0.0, r - bs));

                let c1 = mix(self.color, self.color_focus, self.focus);
                let c2 = mix(c1, self.color_hover, self.hover);
                let c3 = mix(c2, self.color_down, self.down);
                let col = mix(c3, self.color_disabled, self.disabled);

                sdf.fill(col);

                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, r);

                let bc1 = mix(self.border_color, self.border_color_focus, self.focus);
                let bc2 = mix(bc1, self.border_color_hover, self.hover);
                let bc3 = mix(bc2, self.border_color_down, self.down);
                let bcol = mix(bc3, self.border_color_disabled, self.disabled);

                sdf.stroke(bcol, bs);

                return sdf.result;
            }
        }
        draw_text +: {
            color: uniform(#27272A)
            color_hover: uniform(#18181B)
            color_down: uniform(#18181B)
            color_focus: uniform(#27272A)
            color_disabled: uniform(#A1A1AA)
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonGhost = mod.widgets.Button{
        text: "Ghost"
        width: Fit height: Fit
        padding: Inset{top: 10., right: 20., bottom: 10., left: 20.}
        draw_bg +: {
            color: uniform(#FFFFFF00)
            color_hover: uniform(#10A37F14)
            color_down: uniform(#10A37F24)
            color_focus: uniform(#FFFFFF00)
            color_disabled: uniform(#FFFFFF00)
            border_size: uniform(0.0)
            border_radius: uniform(8.0)
            border_color: uniform(#00000000)
            border_color_hover: uniform(#00000000)
            border_color_down: uniform(#00000000)
            border_color_focus: uniform(#00000000)
            border_color_disabled: uniform(#00000000)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);

                let c1 = mix(self.color, self.color_focus, self.focus);
                let c2 = mix(c1, self.color_hover, self.hover);
                let c3 = mix(c2, self.color_down, self.down);
                let col = mix(c3, self.color_disabled, self.disabled);

                sdf.fill(col);
                return sdf.result;
            }
        }
        draw_text +: {
            color: uniform(#10A37F)
            color_hover: uniform(#0D8A6A)
            color_down: uniform(#0B7359)
            color_focus: uniform(#10A37F)
            color_disabled: uniform(#10A37F60)
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonDanger = mod.widgets.Button{
        text: "Danger"
        width: Fit height: Fit
        padding: Inset{top: 10., right: 20., bottom: 10., left: 20.}
        draw_bg +: {
            color: uniform(#DC2626)
            color_hover: uniform(#B91C1C)
            color_down: uniform(#991B1B)
            color_focus: uniform(#DC2626)
            color_disabled: uniform(#FCA5A5)
            border_size: uniform(0.0)
            border_radius: uniform(8.0)
            border_color: uniform(#00000000)
            border_color_hover: uniform(#00000000)
            border_color_down: uniform(#00000000)
            border_color_focus: uniform(#00000000)
            border_color_disabled: uniform(#00000000)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = self.border_radius;

                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, r);

                let c1 = mix(self.color, self.color_focus, self.focus);
                let c2 = mix(c1, self.color_hover, self.hover);
                let c3 = mix(c2, self.color_down, self.down);
                let col = mix(c3, self.color_disabled, self.disabled);

                let g = mix(1.06, 0.94, self.pos.y);
                let shaded = vec4(col.rgb * g, col.a);

                sdf.fill(shaded);
                return sdf.result;
            }
        }
        draw_text +: {
            color: uniform(#FFFFFF)
            color_hover: uniform(#FFFFFF)
            color_down: uniform(#FFFFFFEE)
            color_focus: uniform(#FFFFFF)
            color_disabled: uniform(#FFFFFF80)
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonSmall = mod.widgets.OxButton{
        padding: Inset{top: 6., right: 14., bottom: 6., left: 14.}
        draw_text +: { text_style +: { font_size: 12. } }
        draw_bg +: { border_radius: uniform(6.0) }
    }

    mod.widgets.OxButtonLarge = mod.widgets.OxButton{
        padding: Inset{top: 14., right: 28., bottom: 14., left: 28.}
        draw_text +: { text_style +: { font_size: 16. } }
        draw_bg +: { border_radius: uniform(10.0) }
    }
}

pub fn apply_button_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.interactive_default),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_hover),
        &v4(theme.colors.interactive_hover),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_down),
        &v4(theme.colors.interactive_pressed),
    );
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_button_secondary_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_primary));
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_hover),
        &v4(theme.colors.surface_secondary),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_down),
        &v4(theme.colors.surface_tertiary),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(border_color),
        &v4(theme.colors.border_default),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(border_color_hover),
        &v4(theme.colors.border_hover),
    );
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_button_ghost_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let c = theme.colors.interactive_default;
    let hover_a = theme.opacity.hover_overlay as f32;
    let press_a = theme.opacity.pressed_overlay as f32;
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &[0.0, 0.0, 0.0, 0.0]);
    set_widget_draw_uniform(cx, area, live_id!(color_hover), &[c.x, c.y, c.z, hover_a]);
    set_widget_draw_uniform(cx, area, live_id!(color_down), &[c.x, c.y, c.z, press_a]);
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_button_danger_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let c = theme.colors.feedback_error;
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(c));
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_hover),
        &[c.x * 0.9, c.y * 0.9, c.z * 0.9, c.w],
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_down),
        &[c.x * 0.8, c.y * 0.8, c.z * 0.8, c.w],
    );
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_button_small_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_button_theme(cx, widget, theme);
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.sm as f32]);
}

pub fn apply_button_large_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_button_theme(cx, widget, theme);
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.lg as f32]);
}
