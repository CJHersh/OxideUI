use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxTabs = mod.widgets.View{
        width: Fill height: Fit
        flow: Right spacing: 0
    }

    mod.widgets.OxTab = mod.widgets.View{
        width: Fit height: Fit
        flow: Down spacing: 0
        cursor: Hand
        padding: Inset{left: 16., right: 16., top: 10., bottom: 0.}
        label := Label{
            draw_text +: {
                color: uniform(#333333)
                text_style +: { font_size: 14. }
            }
            text: "Tab"
        }
        indicator := RoundedView{
            width: Fill height: 2.
            margin: Inset{top: 8.}
            draw_bg +: {
                color: uniform(#10A37F)
                border_radius: uniform(1.0)
            }
        }
    }

    mod.widgets.OxBreadcrumb = mod.widgets.View{
        width: Fit height: Fit
        flow: Right spacing: 8.
        align: Align{y: 0.5}
    }

    mod.widgets.OxBreadcrumbSeparator = mod.widgets.Label{
        draw_text +: {
            color: uniform(#999999)
            text_style +: { font_size: 14. }
        }
        text: "/"
    }

    mod.widgets.OxPagination = mod.widgets.View{
        width: Fit height: Fit
        flow: Right spacing: 4.
        align: Align{y: 0.5}
    }

    mod.widgets.OxStepper = mod.widgets.View{
        width: Fill height: Fit
        flow: Right spacing: 16.
        align: Align{y: 0.5}
    }
}

pub fn apply_tab_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let indicator = widget.widget(cx, &[id!(indicator)]);
    let area = indicator.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}
