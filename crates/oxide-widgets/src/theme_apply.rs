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
}
