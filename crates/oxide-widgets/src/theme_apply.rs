use makepad_widgets::*;
use oxide_core::theme::Theme;

/// Function signature for per-widget theme application.
pub type ApplyFn = fn(&mut Cx, &WidgetRef, &Theme);

/// Holds named widget refs and their associated apply functions for batch theming.
pub struct ThemeTarget<'a> {
    pub widget: &'a WidgetRef,
    pub apply_fn: ApplyFn,
}

impl<'a> ThemeTarget<'a> {
    pub fn new(widget: &'a WidgetRef, apply_fn: ApplyFn) -> Self {
        Self { widget, apply_fn }
    }
}

/// Apply a theme to a list of widget targets.
pub fn apply_theme(cx: &mut Cx, targets: &[ThemeTarget], theme: &Theme) {
    for target in targets {
        (target.apply_fn)(cx, target.widget, theme);
    }
}

/// A stored entry in a [`ThemeMap`] that pairs a widget path with its apply function.
struct ThemeEntry {
    id: LiveId,
    apply_fn: ApplyFn,
}

/// A reusable map of widget-to-theme-function bindings.
///
/// Build once at startup, then call [`ThemeMap::apply_all`] each time the
/// theme changes. This avoids repeating dozens of `ThemeTarget::new(...)` lines
/// on every switch.
///
/// ```rust,ignore
/// let map = ThemeMap::builder(&self.ui)
///     .add(live_id!(submit), buttons::apply_button_theme)
///     .add(live_id!(cancel), buttons::apply_button_secondary_theme)
///     .add(live_id!(title), display::apply_label_title_theme)
///     .add(live_id!(card), layout::apply_card_theme)
///     .build();
///
/// // On theme switch:
/// ThemeEngine::switch_by_name(name);
/// let theme = ThemeEngine::current();
/// map.apply_all(cx, &theme);
/// self.ui.redraw(cx);
/// ```
pub struct ThemeMap {
    ui: WidgetRef,
    entries: Vec<ThemeEntry>,
}

impl ThemeMap {
    /// Start building a new theme map rooted at the given UI widget.
    pub fn builder(ui: &WidgetRef) -> ThemeMapBuilder {
        ThemeMapBuilder {
            ui: ui.clone(),
            entries: Vec::new(),
        }
    }

    /// Apply the active theme to all registered widgets.
    ///
    /// Each widget is resolved from the UI tree by its `LiveId` and the
    /// corresponding apply function is called. Widgets that cannot be found
    /// are silently skipped.
    pub fn apply_all(&self, cx: &mut Cx, theme: &Theme) {
        for entry in &self.entries {
            let widget = self.ui.widget(cx, &[entry.id]);
            (entry.apply_fn)(cx, &widget, theme);
        }
    }

    /// Returns the number of registered widget-theme bindings.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if no widget-theme bindings have been registered.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns `true` if a binding for the given `LiveId` exists.
    pub fn contains(&self, id: LiveId) -> bool {
        self.entries.iter().any(|e| e.id == id)
    }
}

/// Builder for constructing a [`ThemeMap`].
pub struct ThemeMapBuilder {
    ui: WidgetRef,
    entries: Vec<ThemeEntry>,
}

impl ThemeMapBuilder {
    /// Register a named widget and its theme-apply function.
    pub fn add(mut self, id: LiveId, apply_fn: ApplyFn) -> Self {
        self.entries.push(ThemeEntry { id, apply_fn });
        self
    }

    /// Finalize the map.
    pub fn build(self) -> ThemeMap {
        ThemeMap {
            ui: self.ui,
            entries: self.entries,
        }
    }
}

/// Convert a `Vec4` color to an `[f32; 4]` array for uniform setting.
#[inline]
pub fn v4(c: Vec4) -> [f32; 4] {
    [c.x, c.y, c.z, c.w]
}

/// Set a shader uniform on a widget's draw call via Cx internals.
///
/// Only affects properties declared with `uniform()` in the shader DSL.
/// Call `widget.redraw(cx)` afterward to make the change visible.
pub fn set_widget_draw_uniform(cx: &mut Cx, area: Area, name: LiveId, value: &[f32]) {
    let info = {
        if let Some(inst) = area.valid_instance(cx) {
            let draw_list = &cx.draw_lists[inst.draw_list_id];
            let draw_item = &draw_list.draw_items[inst.draw_item_id];
            if let Some(draw_call) = draw_item.kind.draw_call() {
                let sh = &cx.draw_shaders[draw_call.draw_shader_id.index];
                sh.mapping
                    .dyn_uniforms
                    .inputs
                    .iter()
                    .find(|i| i.id == name)
                    .map(|input| {
                        (
                            inst.draw_list_id,
                            inst.draw_item_id,
                            input.offset,
                            input.slots,
                        )
                    })
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some((dl_id, di_id, offset, slots)) = info {
        let pass_id = cx.draw_lists[dl_id].draw_pass_id;
        let draw_list = &mut cx.draw_lists[dl_id];
        let draw_item = &mut draw_list.draw_items[di_id];
        if let Some(draw_call) = draw_item.kind.draw_call_mut() {
            let count = value.len().min(slots);
            draw_call.dyn_uniforms[offset..offset + count].copy_from_slice(&value[..count]);
            draw_call.uniforms_dirty = true;
        }
        if let Some(pid) = pass_id {
            cx.passes[pid].paint_dirty = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v4_converts_vec4_correctly() {
        let c = vec4(0.1, 0.2, 0.3, 0.4);
        let arr = v4(c);
        assert!((arr[0] - 0.1).abs() < 1e-5);
        assert!((arr[1] - 0.2).abs() < 1e-5);
        assert!((arr[2] - 0.3).abs() < 1e-5);
        assert!((arr[3] - 0.4).abs() < 1e-5);
    }

    #[test]
    fn theme_map_builder_chain() {
        fn dummy(_cx: &mut Cx, _w: &WidgetRef, _t: &Theme) {}
        let ui = WidgetRef::default();
        let map = ThemeMap::builder(&ui)
            .add(live_id!(a), dummy)
            .add(live_id!(b), dummy)
            .add(live_id!(c), dummy)
            .build();
        assert_eq!(map.len(), 3);
        assert!(!map.is_empty());
        assert!(map.contains(live_id!(a)));
        assert!(map.contains(live_id!(b)));
        assert!(map.contains(live_id!(c)));
        assert!(!map.contains(live_id!(d)));
    }

    #[test]
    fn theme_map_empty_by_default() {
        let ui = WidgetRef::default();
        let map = ThemeMap::builder(&ui).build();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn theme_map_can_register_all_apply_functions() {
        use crate::buttons;
        use crate::data;
        use crate::display;
        use crate::feedback;
        use crate::inputs;
        use crate::layout;
        use crate::navigation;
        use crate::overlay;

        let ui = WidgetRef::default();
        let map = ThemeMap::builder(&ui)
            .add(live_id!(btn), buttons::apply_button_theme)
            .add(live_id!(btn_sec), buttons::apply_button_secondary_theme)
            .add(live_id!(btn_ghost), buttons::apply_button_ghost_theme)
            .add(live_id!(btn_danger), buttons::apply_button_danger_theme)
            .add(live_id!(btn_outline), buttons::apply_button_outline_theme)
            .add(live_id!(btn_sm), buttons::apply_button_small_theme)
            .add(live_id!(btn_lg), buttons::apply_button_large_theme)
            .add(live_id!(icon_btn), buttons::apply_icon_button_theme)
            .add(live_id!(toggle_btn), buttons::apply_toggle_button_theme)
            .add(live_id!(label), display::apply_label_theme)
            .add(live_id!(label_title), display::apply_label_title_theme)
            .add(live_id!(label_sub), display::apply_label_subtitle_theme)
            .add(live_id!(label_body), display::apply_label_body_theme)
            .add(live_id!(label_cap), display::apply_label_caption_theme)
            .add(live_id!(label_sec), display::apply_label_secondary_theme)
            .add(live_id!(label_link), display::apply_label_link_theme)
            .add(live_id!(badge), display::apply_badge_theme)
            .add(live_id!(badge_s), display::apply_badge_success_theme)
            .add(live_id!(badge_w), display::apply_badge_warning_theme)
            .add(live_id!(badge_e), display::apply_badge_error_theme)
            .add(live_id!(badge_i), display::apply_badge_info_theme)
            .add(live_id!(avatar), display::apply_avatar_theme)
            .add(live_id!(icon), display::apply_icon_theme)
            .add(live_id!(icon_a), display::apply_icon_accent_theme)
            .add(live_id!(icon_s), display::apply_icon_secondary_theme)
            .add(live_id!(text_in), inputs::apply_text_input_theme)
            .add(live_id!(text_area), inputs::apply_text_area_theme)
            .add(live_id!(checkbox), inputs::apply_checkbox_theme)
            .add(live_id!(radio), inputs::apply_radio_theme)
            .add(live_id!(switch), inputs::apply_switch_theme)
            .add(live_id!(slider), inputs::apply_slider_theme)
            .add(live_id!(card), layout::apply_card_theme)
            .add(live_id!(divider), layout::apply_divider_theme)
            .add(live_id!(progress), feedback::apply_progress_theme)
            .add(live_id!(alert), feedback::apply_alert_theme)
            .add(live_id!(alert_s), feedback::apply_alert_success_theme)
            .add(live_id!(alert_w), feedback::apply_alert_warning_theme)
            .add(live_id!(alert_e), feedback::apply_alert_error_theme)
            .add(live_id!(skel), feedback::apply_skeleton_theme)
            .add(live_id!(skel_c), feedback::apply_skeleton_circle_theme)
            .add(live_id!(skel_t), feedback::apply_skeleton_text_theme)
            .add(live_id!(tab), navigation::apply_tab_theme)
            .add(live_id!(tooltip), overlay::apply_tooltip_theme)
            .add(live_id!(popover), overlay::apply_popover_theme)
            .add(live_id!(drawer), overlay::apply_drawer_theme)
            .add(live_id!(menu), overlay::apply_menu_theme)
            .add(live_id!(menu_item), overlay::apply_menu_item_theme)
            .add(live_id!(list_item), data::apply_list_item_theme)
            .add(live_id!(table_hdr), data::apply_table_header_theme)
            .build();

        assert_eq!(map.len(), 49);
    }

    #[test]
    fn all_apply_functions_are_valid_fn_pointers() {
        let fns: Vec<ApplyFn> = vec![
            crate::buttons::apply_button_theme,
            crate::buttons::apply_button_secondary_theme,
            crate::buttons::apply_button_ghost_theme,
            crate::buttons::apply_button_danger_theme,
            crate::buttons::apply_button_outline_theme,
            crate::buttons::apply_button_small_theme,
            crate::buttons::apply_button_large_theme,
            crate::buttons::apply_icon_button_theme,
            crate::buttons::apply_toggle_button_theme,
            crate::display::apply_label_theme,
            crate::display::apply_label_title_theme,
            crate::display::apply_label_subtitle_theme,
            crate::display::apply_label_body_theme,
            crate::display::apply_label_caption_theme,
            crate::display::apply_label_secondary_theme,
            crate::display::apply_label_link_theme,
            crate::display::apply_badge_theme,
            crate::display::apply_badge_success_theme,
            crate::display::apply_badge_warning_theme,
            crate::display::apply_badge_error_theme,
            crate::display::apply_badge_info_theme,
            crate::display::apply_avatar_theme,
            crate::display::apply_icon_theme,
            crate::display::apply_icon_accent_theme,
            crate::display::apply_icon_secondary_theme,
            crate::inputs::apply_text_input_theme,
            crate::inputs::apply_text_area_theme,
            crate::inputs::apply_checkbox_theme,
            crate::inputs::apply_radio_theme,
            crate::inputs::apply_switch_theme,
            crate::inputs::apply_slider_theme,
            crate::layout::apply_card_theme,
            crate::layout::apply_divider_theme,
            crate::feedback::apply_progress_theme,
            crate::feedback::apply_alert_theme,
            crate::feedback::apply_alert_success_theme,
            crate::feedback::apply_alert_warning_theme,
            crate::feedback::apply_alert_error_theme,
            crate::feedback::apply_skeleton_theme,
            crate::feedback::apply_skeleton_circle_theme,
            crate::feedback::apply_skeleton_text_theme,
            crate::navigation::apply_tab_theme,
            crate::overlay::apply_tooltip_theme,
            crate::overlay::apply_popover_theme,
            crate::overlay::apply_drawer_theme,
            crate::overlay::apply_menu_theme,
            crate::overlay::apply_menu_item_theme,
            crate::data::apply_list_item_theme,
            crate::data::apply_table_header_theme,
        ];
        assert_eq!(fns.len(), 49, "Expected 49 apply_*_theme functions");
    }
}
