/// OxSwitch renders as a toggle switch using Makepad's built-in Toggle widget.
///
/// Default values below match the shadcn light theme from
/// `oxide_core::theme::themes::shadcn`. The `apply_switch_theme` function
/// overrides these at runtime for theme switching.
use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxSwitch = mod.widgets.ToggleFlat{
        width: Fit height: Fit
        text: ""
        padding: Inset{top: 0., right: 0., bottom: 0., left: 0.}

        draw_bg +: {
            size: uniform(22.0)
            border_size: uniform(1.0)
            border_radius: uniform(100.0)

            color: #E5E5E5
            color_hover: #D4D4D4
            color_down: #D4D4D4
            color_active: #171717
            color_focus: #E5E5E5
            color_disabled: #F5F5F5

            border_color: #D4D4D4
            border_color_hover: #A3A3A3
            border_color_down: #A3A3A3
            border_color_active: #171717
            border_color_focus: #D4D4D480
            border_color_disabled: #E5E5E5

            mark_color: #FFFFFF
            mark_color_hover: #FFFFFF
            mark_color_active: #FFFFFF
            mark_color_active_hover: #FFFFFF
            mark_color_disabled: #A3A3A3
        }

        animator +: {
            active: {
                default: @off
                off: AnimatorState{
                    ease: OutQuad
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {active: 0.0}
                    }
                }
                on: AnimatorState{
                    ease: OutQuad
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {active: 1.0}
                    }
                }
            }
        }
    }
}

pub fn apply_switch_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.border_default));
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color_active),
        &v4(theme.colors.interactive_default),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(border_color),
        &v4(theme.colors.border_hover),
    );
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(border_color_active),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}
