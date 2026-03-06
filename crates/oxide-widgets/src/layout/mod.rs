use makepad_widgets::*;
use oxide_core::theme::Theme;

use crate::theme_apply::{set_widget_draw_uniform, v4};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxCard = mod.widgets.RoundedView{
        width: Fill height: Fit
        padding: 24.
        flow: Down spacing: 16.
        draw_bg +: {
            color: uniform(#FFFFFF)
            border_radius: uniform(12.0)
            border_size: uniform(1.0)
            border_color: uniform(#E4E4E7)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = self.border_radius;
                let bs = self.border_size;
                let sz = self.rect_size;

                sdf.box(bs, bs, sz.x - bs * 2.0, sz.y - bs * 2.0, max(0.0, r - bs));
                sdf.fill(self.color);

                sdf.box(bs * 0.5, bs * 0.5, sz.x - bs, sz.y - bs, r);
                sdf.stroke(self.border_color, bs);

                return sdf.result;
            }
        }
    }

    mod.widgets.OxDivider = mod.widgets.SolidView{
        width: Fill height: 1.
        draw_bg +: {
            color: uniform(#E4E4E7)
        }
    }

    mod.widgets.OxStack = mod.widgets.View{
        width: Fill height: Fit
        flow: Down spacing: 16.
    }

    mod.widgets.OxGrid = mod.widgets.View{
        width: Fill height: Fit
        flow: Right spacing: 16.
    }
}

pub fn apply_card_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.surface_primary));
    set_widget_draw_uniform(
        cx,
        area,
        live_id!(border_color),
        &v4(theme.colors.border_default),
    );
    set_widget_draw_uniform(cx, area, live_id!(border_radius), &[theme.radius.lg as f32]);
    widget.redraw(cx);
}

pub fn apply_divider_theme(cx: &mut Cx, widget: &WidgetRef, theme: &Theme) {
    let area = widget.area();
    set_widget_draw_uniform(cx, area, live_id!(color), &v4(theme.colors.border_default));
    widget.redraw(cx);
}
