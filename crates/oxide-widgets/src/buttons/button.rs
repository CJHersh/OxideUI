use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxButton = mod.widgets.Button{
        text: "Button"
        width: Fit height: Fit
        padding: Inset{top: 8., right: 16., bottom: 8., left: 16.}
        draw_bg +: {
            color: uniform(#171717)
            color_hover: uniform(#404040)
            color_down: uniform(#525252)
            color_focus: uniform(#171717)
            color_disabled: uniform(#D4D4D4)
            border_size: uniform(0.0)
            border_radius: uniform(6.0)
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

                sdf.fill(col);
                return sdf.result;
            }
        }
        draw_text +: {
            color: uniform(#FAFAFA)
            color_hover: uniform(#FAFAFA)
            color_down: uniform(#FAFAFAEE)
            color_focus: uniform(#FAFAFA)
            color_disabled: uniform(#FAFAFA80)
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonSecondary = mod.widgets.Button{
        text: "Secondary"
        width: Fit height: Fit
        padding: Inset{top: 8., right: 16., bottom: 8., left: 16.}
        draw_bg +: {
            color: uniform(#F5F5F5)
            color_hover: uniform(#FAFAFA)
            color_down: uniform(#E5E5E5)
            color_focus: uniform(#F5F5F5)
            color_disabled: uniform(#FAFAFA)
            border_size: uniform(1.0)
            border_radius: uniform(6.0)
            border_color: uniform(#E5E5E5)
            border_color_hover: uniform(#D4D4D4)
            border_color_down: uniform(#D4D4D4)
            border_color_focus: uniform(#D4D4D480)
            border_color_disabled: uniform(#E5E5E5)

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
            color: uniform(#171717)
            color_hover: uniform(#0A0A0A)
            color_down: uniform(#0A0A0A)
            color_focus: uniform(#171717)
            color_disabled: uniform(#A3A3A3)
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonGhost = mod.widgets.Button{
        text: "Ghost"
        width: Fit height: Fit
        padding: Inset{top: 8., right: 16., bottom: 8., left: 16.}
        draw_bg +: {
            color: uniform(#FFFFFF00)
            color_hover: uniform(#0000000D)
            color_down: uniform(#00000014)
            color_focus: uniform(#FFFFFF00)
            color_disabled: uniform(#FFFFFF00)
            border_size: uniform(0.0)
            border_radius: uniform(6.0)
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
            color: uniform(#404040)
            color_hover: uniform(#171717)
            color_down: uniform(#0A0A0A)
            color_focus: uniform(#404040)
            color_disabled: uniform(#40404060)
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonDanger = mod.widgets.Button{
        text: "Danger"
        width: Fit height: Fit
        padding: Inset{top: 8., right: 16., bottom: 8., left: 16.}
        draw_bg +: {
            color: uniform(#DC2626)
            color_hover: uniform(#B91C1C)
            color_down: uniform(#991B1B)
            color_focus: uniform(#DC2626)
            color_disabled: uniform(#FCA5A5)
            border_size: uniform(0.0)
            border_radius: uniform(6.0)
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

                sdf.fill(col);
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

    mod.widgets.OxButtonOutline = mod.widgets.Button{
        text: "Outline"
        width: Fit height: Fit
        padding: Inset{top: 8., right: 16., bottom: 8., left: 16.}
        draw_bg +: {
            color: uniform(#FFFFFF00)
            color_hover: uniform(#0000000D)
            color_down: uniform(#00000014)
            color_focus: uniform(#FFFFFF00)
            color_disabled: uniform(#FFFFFF00)
            border_size: uniform(1.0)
            border_radius: uniform(6.0)
            border_color: uniform(#E5E5E5)
            border_color_hover: uniform(#D4D4D4)
            border_color_down: uniform(#D4D4D4)
            border_color_focus: uniform(#D4D4D480)
            border_color_disabled: uniform(#E5E5E580)

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
            color: uniform(#0A0A0A)
            color_hover: uniform(#171717)
            color_down: uniform(#171717)
            color_focus: uniform(#0A0A0A)
            color_disabled: uniform(#A3A3A3)
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonSmall = mod.widgets.OxButton{
        padding: Inset{top: 6., right: 12., bottom: 6., left: 12.}
        draw_text +: { text_style +: { font_size: 14. } }
        draw_bg +: { border_radius: uniform(6.0) }
    }

    mod.widgets.OxButtonLarge = mod.widgets.OxButton{
        padding: Inset{top: 8., right: 32., bottom: 8., left: 32.}
        draw_text +: { text_style +: { font_size: 14. } }
        draw_bg +: { border_radius: uniform(6.0) }
    }
}

pub fn apply_button_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.interactive_default));
    set_widget_draw_uniform(cx, area, live_id!(color_hover), &v4(theme.colors.interactive_hover));
    set_widget_draw_uniform(cx, area, live_id!(color_down), &v4(theme.colors.interactive_pressed));
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_button_secondary_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_secondary));
    set_widget_draw_uniform(cx, area, live_id!(color_hover), &v4(theme.colors.surface_primary));
    set_widget_draw_uniform(cx, area, live_id!(color_down), &v4(theme.colors.surface_tertiary));
    set_widget_draw_uniform(cx, area, live_id!(border_color), &v4(theme.colors.border_default));
    set_widget_draw_uniform(cx, area, live_id!(border_color_hover), &v4(theme.colors.border_hover));
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_button_ghost_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let hover_a = theme.opacity.hover_overlay as f32;
    let press_a = theme.opacity.pressed_overlay as f32;
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &[0.0, 0.0, 0.0, 0.0]);
    set_widget_draw_uniform(cx, area, live_id!(color_hover), &[0.0, 0.0, 0.0, hover_a]);
    set_widget_draw_uniform(cx, area, live_id!(color_down), &[0.0, 0.0, 0.0, press_a]);
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_button_danger_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let c = theme.colors.feedback_error;
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(c));
    set_widget_draw_uniform(cx, area, live_id!(color_hover), &[c.x * 0.85, c.y * 0.85, c.z * 0.85, c.w]);
    set_widget_draw_uniform(cx, area, live_id!(color_down), &[c.x * 0.7, c.y * 0.7, c.z * 0.7, c.w]);
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_button_outline_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let hover_a = theme.opacity.hover_overlay as f32;
    let press_a = theme.opacity.pressed_overlay as f32;
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &[0.0, 0.0, 0.0, 0.0]);
    set_widget_draw_uniform(cx, area, live_id!(color_hover), &[0.0, 0.0, 0.0, hover_a]);
    set_widget_draw_uniform(cx, area, live_id!(color_down), &[0.0, 0.0, 0.0, press_a]);
    set_widget_draw_uniform(cx, area, live_id!(border_color), &v4(theme.colors.border_default));
    set_widget_draw_uniform(cx, area, live_id!(border_color_hover), &v4(theme.colors.border_hover));
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_button_small_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_button_theme(cx, widget, theme);
}

pub fn apply_button_large_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_button_theme(cx, widget, theme);
}
