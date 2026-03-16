use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxAvatar = mod.widgets.RoundedView{
        width: 40. height: 40.
        align: Center
        draw_bg +: {
            color: #171717
            border_radius: 20.0
        }
        initials := Label{
            draw_text +: {
                color: #FAFAFA
                text_style +: { font_size: 14. }
            }
            text: "AB"
        }
    }

    mod.widgets.OxAvatarSmall = mod.widgets.RoundedView{
        width: 32. height: 32.
        align: Center
        draw_bg +: {
            color: #171717
            border_radius: 16.0
        }
        initials := Label{
            draw_text +: {
                color: #FAFAFA
                text_style +: { font_size: 11. }
            }
            text: "AB"
        }
    }

    mod.widgets.OxAvatarLarge = mod.widgets.RoundedView{
        width: 56. height: 56.
        align: Center
        draw_bg +: {
            color: #171717
            border_radius: 28.0
        }
        initials := Label{
            draw_text +: {
                color: #FAFAFA
                text_style +: { font_size: 18. }
            }
            text: "AB"
        }
    }

    mod.widgets.OxAvatarXLarge = mod.widgets.RoundedView{
        width: 80. height: 80.
        align: Center
        draw_bg +: {
            color: #171717
            border_radius: 40.0
        }
        initials := Label{
            draw_text +: {
                color: #FAFAFA
                text_style +: { font_size: 24. }
            }
            text: "AB"
        }
    }
}

pub fn apply_avatar_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(color),
        &v4(theme.colors.interactive_default),
    );
    widget.redraw(cx);
}
