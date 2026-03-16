/// OxIcon renders SVG icons using Makepad's DrawSvg system.
///
/// Default values below match the shadcn light theme from
/// `oxide_core::theme::themes::shadcn`. The `apply_*_theme` functions
/// override these at runtime for theme switching.
use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxIcon = mod.widgets.Icon{
        width: Fit height: Fit
        icon_walk: Walk{ width: 24. height: Fit }
        draw_bg +: { color: #00000000 }
        draw_icon +: {
            svg: crate_resource("self:resources/icons/check.svg")
            color: #0A0A0A
        }
    }

    mod.widgets.OxIconSmall = mod.widgets.OxIcon{
        icon_walk: Walk{ width: 16. height: Fit }
    }

    mod.widgets.OxIconLarge = mod.widgets.OxIcon{
        icon_walk: Walk{ width: 32. height: Fit }
    }

    mod.widgets.OxIconCheck = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/check.svg") }
    }

    mod.widgets.OxIconClose = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/close.svg") }
    }

    mod.widgets.OxIconChevronRight = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/chevron-right.svg") }
    }

    mod.widgets.OxIconChevronLeft = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/chevron-left.svg") }
    }

    mod.widgets.OxIconChevronDown = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/chevron-down.svg") }
    }

    mod.widgets.OxIconChevronUp = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/chevron-up.svg") }
    }

    mod.widgets.OxIconPlus = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/plus.svg") }
    }

    mod.widgets.OxIconMinus = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/minus.svg") }
    }

    mod.widgets.OxIconSearch = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/search.svg") }
    }

    mod.widgets.OxIconMenu = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/menu.svg") }
    }

    mod.widgets.OxIconAlertCircle = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/alert-circle.svg") }
    }

    mod.widgets.OxIconInfo = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/info.svg") }
    }

    mod.widgets.OxIconArrowRight = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/arrow-right.svg") }
    }

    mod.widgets.OxIconArrowLeft = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/arrow-left.svg") }
    }

    mod.widgets.OxIconEye = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/eye.svg") }
    }

    mod.widgets.OxIconCopy = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/copy.svg") }
    }

    mod.widgets.OxIconSettings = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/settings.svg") }
    }

    mod.widgets.OxIconStar = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/star.svg") }
    }

    mod.widgets.OxIconHeart = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/heart.svg") }
    }

    mod.widgets.OxIconExternalLink = mod.widgets.OxIcon{
        draw_icon +: { svg: crate_resource("self:resources/icons/external-link.svg") }
    }
}

pub fn apply_icon_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.text_primary));
    widget.redraw(cx);
}

pub fn apply_icon_accent_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}

pub fn apply_icon_secondary_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.text_secondary));
    widget.redraw(cx);
}
