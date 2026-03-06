use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{OnceLock, RwLock};

use makepad_widgets::Cx;

use super::Theme;

static THEMES: OnceLock<RwLock<Vec<Theme>>> = OnceLock::new();
static CURRENT_THEME: AtomicUsize = AtomicUsize::new(0);

/// Action emitted when the active theme changes.
///
/// Use the `_and_signal` variants of `ThemeEngine` methods to automatically
/// post this action, then check for it in `handle_actions`:
///
/// ```rust,ignore
/// fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
///     if ThemeChangedAction::is_in(actions) {
///         let theme = ThemeEngine::current();
///         self.theme_map.apply_all(cx, &theme);
///         self.ui.redraw(cx);
///     }
/// }
/// ```
#[derive(Clone, Debug)]
pub struct ThemeChangedAction;

/// Global theme engine for switching between themes at runtime.
///
/// Call [`ThemeEngine::init`] to set the initial theme list. Subsequent calls
/// replace the existing themes, allowing hot-reload workflows.
pub struct ThemeEngine;

impl ThemeEngine {
    /// Initialize (or replace) the theme list.
    ///
    /// The first call creates the internal storage. Later calls swap in the
    /// new list and reset the active index to 0.
    pub fn init(themes: Vec<Theme>) {
        match THEMES.get() {
            Some(lock) => {
                let mut guard = lock.write().unwrap();
                *guard = themes;
                CURRENT_THEME.store(0, Ordering::Relaxed);
            }
            None => {
                let _ = THEMES.set(RwLock::new(themes));
            }
        }
    }

    /// Get a clone of the current theme. Panics if not initialized.
    pub fn current() -> Theme {
        Self::try_current().expect("ThemeEngine not initialized")
    }

    /// Get a clone of the current theme, or `None` if not initialized.
    pub fn try_current() -> Option<Theme> {
        let lock = THEMES.get()?;
        let themes = lock.read().unwrap();
        let idx = CURRENT_THEME.load(Ordering::Relaxed);
        themes.get(idx).or(themes.first()).cloned()
    }

    /// Switch to theme by index (clamped to valid range).
    pub fn switch(index: usize) {
        if let Some(lock) = THEMES.get() {
            let themes = lock.read().unwrap();
            let idx = index.min(themes.len().saturating_sub(1));
            CURRENT_THEME.store(idx, Ordering::Relaxed);
        }
    }

    /// Switch to theme by name. Returns `true` if found.
    pub fn switch_by_name(name: &str) -> bool {
        if let Some(lock) = THEMES.get() {
            let themes = lock.read().unwrap();
            if let Some((idx, _)) = themes.iter().enumerate().find(|(_, t)| t.name == name) {
                CURRENT_THEME.store(idx, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Get the current theme index.
    pub fn current_index() -> usize {
        CURRENT_THEME.load(Ordering::Relaxed)
    }

    /// Get names of all available themes.
    pub fn available_themes() -> Vec<&'static str> {
        THEMES
            .get()
            .map(|lock| {
                let themes = lock.read().unwrap();
                themes.iter().map(|t| t.name).collect()
            })
            .unwrap_or_default()
    }

    /// Get the number of registered themes.
    pub fn theme_count() -> usize {
        THEMES
            .get()
            .map(|lock| lock.read().unwrap().len())
            .unwrap_or(0)
    }

    /// Switch to the next theme (wraps around).
    pub fn next_theme() {
        if let Some(lock) = THEMES.get() {
            let themes = lock.read().unwrap();
            let count = themes.len();
            if count > 0 {
                let idx = (CURRENT_THEME.load(Ordering::Relaxed) + 1) % count;
                CURRENT_THEME.store(idx, Ordering::Relaxed);
            }
        }
    }

    /// Switch to the previous theme (wraps around).
    pub fn prev_theme() {
        if let Some(lock) = THEMES.get() {
            let themes = lock.read().unwrap();
            let count = themes.len();
            if count > 0 {
                let idx = CURRENT_THEME
                    .load(Ordering::Relaxed)
                    .wrapping_add(count - 1)
                    % count;
                CURRENT_THEME.store(idx, Ordering::Relaxed);
            }
        }
    }

    /// Switch by index and post a [`ThemeChangedAction`].
    pub fn switch_and_signal(cx: &mut Cx, index: usize) {
        Self::switch(index);
        cx.action(ThemeChangedAction);
    }

    /// Switch by name and post a [`ThemeChangedAction`]. Returns `true` if found.
    pub fn switch_by_name_and_signal(cx: &mut Cx, name: &str) -> bool {
        let found = Self::switch_by_name(name);
        if found {
            cx.action(ThemeChangedAction);
        }
        found
    }

    /// Cycle to the next theme and post a [`ThemeChangedAction`].
    pub fn next_theme_and_signal(cx: &mut Cx) {
        Self::next_theme();
        cx.action(ThemeChangedAction);
    }

    /// Cycle to the previous theme and post a [`ThemeChangedAction`].
    pub fn prev_theme_and_signal(cx: &mut Cx) {
        Self::prev_theme();
        cx.action(ThemeChangedAction);
    }
}

impl ThemeChangedAction {
    /// Check whether any action in the slice is a `ThemeChangedAction`.
    pub fn is_in(actions: &[Box<dyn makepad_widgets::ActionTrait>]) -> bool {
        actions
            .iter()
            .any(|a| a.downcast_ref::<ThemeChangedAction>().is_some())
    }
}
