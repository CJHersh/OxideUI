use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxLabel = mod.widgets.Label{
        draw_text +: {
            color: uniform(#333333)
            text_style +: { font_size: 16. }
        }
    }

    mod.widgets.OxLabelTitle = mod.widgets.Label{
        draw_text +: {
            color: uniform(#202023)
            text_style +: { font_size: 24. }
        }
    }

    mod.widgets.OxLabelSubtitle = mod.widgets.Label{
        draw_text +: {
            color: uniform(#202023)
            text_style +: { font_size: 18. }
        }
    }

    mod.widgets.OxLabelBody = mod.widgets.Label{
        draw_text +: {
            color: uniform(#333333)
            text_style +: { font_size: 16. }
        }
    }

    mod.widgets.OxLabelCaption = mod.widgets.Label{
        draw_text +: {
            color: uniform(#676A70)
            text_style +: { font_size: 12. }
        }
    }

    mod.widgets.OxLabelSecondary = mod.widgets.Label{
        draw_text +: {
            color: uniform(#8F8FA1)
            text_style +: { font_size: 14. }
        }
    }

    mod.widgets.OxLabelLink = mod.widgets.Label{
        draw_text +: {
            color: uniform(#10A37F)
            text_style +: { font_size: 14. }
        }
    }
}

pub fn apply_label_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.text_primary));
    widget.redraw(cx);
}

pub fn apply_label_title_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.text_primary));
    widget.redraw(cx);
}

pub fn apply_label_subtitle_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.text_primary));
    widget.redraw(cx);
}

pub fn apply_label_body_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.text_primary));
    widget.redraw(cx);
}

pub fn apply_label_caption_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.text_secondary));
    widget.redraw(cx);
}

pub fn apply_label_secondary_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.text_tertiary));
    widget.redraw(cx);
}

pub fn apply_label_link_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.text_link));
    widget.redraw(cx);
}
