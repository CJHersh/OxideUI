use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxTable = mod.widgets.View{
        width: Fill height: Fit
        flow: Down spacing: 0
    }

    mod.widgets.OxTableRow = mod.widgets.View{
        width: Fill height: Fit
        flow: Right
        padding: Inset{left: 12., right: 12., top: 10., bottom: 10.}
    }

    mod.widgets.OxTableHeader = mod.widgets.View{
        width: Fill height: Fit
        flow: Right
        padding: Inset{left: 12., right: 12., top: 10., bottom: 10.}
        show_bg: true
        draw_bg +: {
            color: uniform(#F5F5F5)
        }
    }

    mod.widgets.OxList = mod.widgets.View{
        width: Fill height: Fit
        flow: Down spacing: 0
    }

    mod.widgets.OxListItem = mod.widgets.View{
        width: Fill height: Fit
        padding: Inset{left: 16., right: 16., top: 12., bottom: 12.}
        show_bg: true
        cursor: Hand
        draw_bg +: {
            color: uniform(#FFFFFF00)
        }
        label := Label{
            draw_text +: {
                color: uniform(#0A0A0A)
                text_style +: { font_size: 14. }
            }
            text: "List item"
        }
    }

    mod.widgets.OxTimeline = mod.widgets.View{
        width: Fill height: Fit
        flow: Down spacing: 16.
    }
}

pub fn apply_list_item_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_primary));
    widget.redraw(cx);
}

pub fn apply_table_header_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.surface_secondary),
    );
    widget.redraw(cx);
}
