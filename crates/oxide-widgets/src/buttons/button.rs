/// OxButton variants styled with the shadcn/ui design language.
///
/// Default hex values below match the shadcn light theme from
/// `oxide_core::theme::themes::shadcn`. The `apply_*_theme` functions
/// override these at runtime for theme switching.
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
            color: #171717
            color_hover: #404040
            color_down: #525252
            color_focus: #171717
            color_disabled: #D4D4D4
            border_size: 0.0
            border_radius: 6.0
            border_color: #00000000
            border_color_hover: #00000000
            border_color_down: #00000000
            border_color_focus: #00000000
            border_color_disabled: #00000000
        }
        draw_text +: {
            color: #FAFAFA
            color_hover: #FAFAFA
            color_down: #FAFAFAEE
            color_focus: #FAFAFA
            color_disabled: #FAFAFA80
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonSecondary = mod.widgets.Button{
        text: "Secondary"
        width: Fit height: Fit
        padding: Inset{top: 8., right: 16., bottom: 8., left: 16.}
        draw_bg +: {
            color: #F5F5F5
            color_hover: #FAFAFA
            color_down: #E5E5E5
            color_focus: #F5F5F5
            color_disabled: #FAFAFA
            border_size: 1.0
            border_radius: 6.0
            border_color: #E5E5E5
            border_color_hover: #D4D4D4
            border_color_down: #D4D4D4
            border_color_focus: #D4D4D480
            border_color_disabled: #E5E5E5
        }
        draw_text +: {
            color: #171717
            color_hover: #0A0A0A
            color_down: #0A0A0A
            color_focus: #171717
            color_disabled: #A3A3A3
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonGhost = mod.widgets.Button{
        text: "Ghost"
        width: Fit height: Fit
        padding: Inset{top: 8., right: 16., bottom: 8., left: 16.}
        draw_bg +: {
            color: #FFFFFF00
            color_hover: #0000000D
            color_down: #00000014
            color_focus: #FFFFFF00
            color_disabled: #FFFFFF00
            border_size: 0.0
            border_radius: 6.0
            border_color: #00000000
            border_color_hover: #00000000
            border_color_down: #00000000
            border_color_focus: #00000000
            border_color_disabled: #00000000
        }
        draw_text +: {
            color: #404040
            color_hover: #171717
            color_down: #0A0A0A
            color_focus: #404040
            color_disabled: #40404060
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonDanger = mod.widgets.Button{
        text: "Danger"
        width: Fit height: Fit
        padding: Inset{top: 8., right: 16., bottom: 8., left: 16.}
        draw_bg +: {
            color: #DC2626
            color_hover: #B91C1C
            color_down: #991B1B
            color_focus: #DC2626
            color_disabled: #FCA5A5
            border_size: 0.0
            border_radius: 6.0
            border_color: #00000000
            border_color_hover: #00000000
            border_color_down: #00000000
            border_color_focus: #00000000
            border_color_disabled: #00000000
        }
        draw_text +: {
            color: #FFFFFF
            color_hover: #FFFFFF
            color_down: #FFFFFFEE
            color_focus: #FFFFFF
            color_disabled: #FFFFFF80
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonOutline = mod.widgets.Button{
        text: "Outline"
        width: Fit height: Fit
        padding: Inset{top: 8., right: 16., bottom: 8., left: 16.}
        draw_bg +: {
            color: #FFFFFF00
            color_hover: #0000000D
            color_down: #00000014
            color_focus: #FFFFFF00
            color_disabled: #FFFFFF00
            border_size: 1.0
            border_radius: 6.0
            border_color: #E5E5E5
            border_color_hover: #D4D4D4
            border_color_down: #D4D4D4
            border_color_focus: #D4D4D480
            border_color_disabled: #E5E5E580
        }
        draw_text +: {
            color: #0A0A0A
            color_hover: #171717
            color_down: #171717
            color_focus: #0A0A0A
            color_disabled: #A3A3A3
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxButtonSmall = mod.widgets.OxButton{
        padding: Inset{top: 6., right: 12., bottom: 6., left: 12.}
        draw_text +: { text_style +: { font_size: 14. } }
        draw_bg +: { border_radius: 6.0 }
    }

    mod.widgets.OxButtonLarge = mod.widgets.OxButton{
        padding: Inset{top: 8., right: 32., bottom: 8., left: 32.}
        draw_text +: { text_style +: { font_size: 14. } }
        draw_bg +: { border_radius: 6.0 }
    }

    mod.widgets.OxNavButtonActive = mod.widgets.Button{
        width: Fill height: Fit
        padding: Inset{top: 8., right: 12., bottom: 8., left: 12.}
        draw_bg +: {
            color: #EAEAEA
            color_hover: #E0E0E0
            color_down: #D6D6D6
            color_focus: #EAEAEA
            border_radius: 6.0
        }
        draw_text +: {
            color: #0A0A0A
            color_hover: #0A0A0A
            color_down: #0A0A0A
            color_focus: #0A0A0A
            text_style +: { font_size: 13. }
        }
    }

    mod.widgets.OxNavButtonInactive = mod.widgets.Button{
        width: Fill height: Fit
        padding: Inset{top: 8., right: 12., bottom: 8., left: 12.}
        draw_bg +: {
            color: #FFFFFF00
            color_hover: #F0F0F0
            color_down: #EBEBEB
            color_focus: #FFFFFF00
            border_radius: 6.0
        }
        draw_text +: {
            color: #737373
            color_hover: #525252
            color_down: #404040
            color_focus: #737373
            text_style +: { font_size: 13. }
        }
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
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.surface_secondary),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_hover),
        &v4(theme.colors.surface_primary),
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
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_hover),
        &[c.x * 0.85, c.y * 0.85, c.z * 0.85, c.w],
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_down),
        &[c.x * 0.7, c.y * 0.7, c.z * 0.7, c.w],
    );
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

pub fn apply_button_small_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_button_theme(cx, widget, theme);
}

pub fn apply_button_large_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    apply_button_theme(cx, widget, theme);
}
