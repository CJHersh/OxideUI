/// Overlay widgets for OxideUI.
///
/// **Status: Planned** -- OxTooltip, OxPopover, OxDrawer, and OxMenu are visual shells
/// with `visible: false` defaults but no show/hide logic, positioning, or animation.
/// OxMenuItem is a functional styled container. Full overlay behavior (trigger-based
/// visibility, backdrop, slide animations) will be added in a future release.
use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxTooltip = mod.widgets.RoundedView{
        width: Fit height: Fit
        visible: false
        padding: Inset{left: 8., right: 8., top: 6., bottom: 6.}
        draw_bg +: {
            color: #171717
            border_radius: 4.0
        }
        label := Label{
            draw_text +: {
                color: #FAFAFA
                text_style +: { font_size: 12. }
            }
            text: "Tooltip"
        }
    }

    mod.widgets.OxPopover = mod.widgets.RoundedView{
        width: 240. height: Fit
        visible: false
        padding: 12.
        flow: Down
        draw_bg +: {
            color: #FFFFFF
            border_radius: 6.0
        }
    }

    mod.widgets.OxDrawer = mod.widgets.SolidView{
        width: 320. height: Fill
        visible: false
        flow: Down padding: 16.
        draw_bg +: {
            color: #FFFFFF
        }
    }

    mod.widgets.OxMenu = mod.widgets.RoundedView{
        width: 200. height: Fit
        flow: Down padding: 4.
        draw_bg +: {
            color: #FFFFFF
            border_radius: 6.0
        }
    }

    mod.widgets.OxMenuItem = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: Inset{left: 12., right: 12., top: 8., bottom: 8.}
draw_bg +: {
            color: #FFFFFF00
            border_radius: 4.0
        }
        label := Label{
            draw_text +: {
                color: #0A0A0A
                text_style +: { font_size: 14. }
            }
            text: "Menu Item"
        }
    }
}

pub fn apply_tooltip_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_inverse));
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.sm as f32]);
    widget.redraw(cx);
}

pub fn apply_popover_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_primary));
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_drawer_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_primary));
    widget.redraw(cx);
}

pub fn apply_menu_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_primary));
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.md as f32]);
    widget.redraw(cx);
}

pub fn apply_menu_item_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.sm as f32]);
    widget.redraw(cx);
}
