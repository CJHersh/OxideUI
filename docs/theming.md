# Theming Guide

OxideUI provides a flexible theming system with 6 pre-built themes (3 light + 3 dark) and support for custom themes. DSL definitions use OpenAI Light as the base theme, and `apply_*_theme` functions patch GPU shader uniforms at runtime when themes change.

## Pre-Built Themes

| Theme | Mode | Accent | Description |
|-------|------|--------|-------------|
| **OpenAI** | Light | `#10A37F` | Clean, professional teal |
| **OpenAI Dark** | Dark | `#19C99D` | Dark variant with brighter teal |
| **Airbnb** | Light | `#FF5A5F` | Warm, inviting coral |
| **Airbnb Dark** | Dark | `#FF7B7F` | Dark variant with softer coral |
| **Notion** | Light | `#346CA3` | Calm, focused blue |
| **Notion Dark** | Dark | `#529CCA` | Dark variant with lighter blue |

## Architecture

```
figma/tokens.json
    |  (oxide generate)
    v
Theme structs (oxide-core)
    |
    v
ThemeEngine (global, atomic switch)
    |
    v
apply_*_theme functions (oxide-widgets)
    |  (set_widget_draw_uniform)
    v
Widget GPU shader uniforms
```

JSON tokens are the source of truth. Codegen produces the `Theme` structs. Each widget module has `apply_*_theme` functions that map semantic tokens onto widget shader uniforms via `set_widget_draw_uniform`.

## Using ThemeEngine

### Initialize

```rust
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::themes::all_themes;

// In handle_startup or before first draw
ThemeEngine::init(all_themes());
```

### Switch Theme

```rust
// By name
ThemeEngine::switch_by_name("Airbnb");

// By index
ThemeEngine::switch(1);

// Cycle
ThemeEngine::next_theme();
ThemeEngine::prev_theme();
```

### Switch with Signal

Use the `_and_signal` variants to automatically post a `ThemeChangedAction` that
other widgets can listen for:

```rust
// Posts ThemeChangedAction so handle_actions can react
ThemeEngine::switch_by_name_and_signal(cx, "Airbnb");
ThemeEngine::next_theme_and_signal(cx);
```

Then in your app:

```rust
use oxide_core::theme::engine::ThemeChangedAction;

fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
    if ThemeChangedAction::is_in(actions) {
        let theme = ThemeEngine::current();
        self.theme_map.apply_all(cx, &theme);
        self.ui.redraw(cx);
    }
}
```

### Access Current Theme

```rust
let theme = ThemeEngine::current();
let accent = theme.colors.interactive_default;
let font = theme.typography.font_family;
```

## Applying Themes to Widgets

### Per-Widget Apply Functions

Each widget module provides `apply_*_theme` functions that map semantic tokens to widget properties. Name your widgets with `:=` syntax in the DSL so they can be referenced:

```rust
// DSL: name widgets
submit_btn := OxButton{ text: "Submit" }
cancel_btn := OxButtonSecondary{ text: "Cancel" }
my_input := OxTextInput{ empty_text: "Type here..." }
```

Then apply the theme:

```rust
use oxide_widgets::buttons;
use oxide_widgets::inputs;

let theme = ThemeEngine::current();
buttons::apply_button_theme(cx, &self.ui.button(cx, ids!(submit_btn)), &theme);
buttons::apply_button_secondary_theme(cx, &self.ui.button(cx, ids!(cancel_btn)), &theme);
inputs::apply_text_input_theme(cx, &self.ui.text_input(cx, ids!(my_input)), &theme);
self.ui.redraw(cx);
```

### Batch Apply with ThemeTarget

For convenience, use `apply_theme` with a list of `ThemeTarget` to patch multiple widgets at once:

```rust
use oxide_widgets::theme_apply::{apply_theme, ThemeTarget};
use oxide_widgets::{buttons, inputs, display, layout};

fn switch_theme(&self, cx: &mut Cx, name: &str) {
    ThemeEngine::switch_by_name(name);
    let theme = ThemeEngine::current();

    apply_theme(cx, &[
        ThemeTarget::new(&self.ui.label(cx, ids!(title)), display::apply_label_title_theme),
        ThemeTarget::new(&self.ui.button(cx, ids!(submit_btn)), buttons::apply_button_theme),
        ThemeTarget::new(&self.ui.button(cx, ids!(cancel_btn)), buttons::apply_button_secondary_theme),
        ThemeTarget::new(&self.ui.text_input(cx, ids!(my_input)), inputs::apply_text_input_theme),
        ThemeTarget::new(&self.ui.view(cx, ids!(card)), layout::apply_card_theme),
    ], &theme);

    self.ui.redraw(cx);
}
```

### Available Apply Functions

#### Buttons (`oxide_widgets::buttons`)

| Function | Widget | Semantic Tokens Used |
|----------|--------|---------------------|
| `apply_button_theme` | OxButton | `interactive_default/hover/pressed`, `text_inverse`, `radius.md` |
| `apply_button_secondary_theme` | OxButtonSecondary | `border_default`, `surface_primary/secondary/tertiary`, `text_primary`, `radius.md` |
| `apply_button_ghost_theme` | OxButtonGhost | `interactive_default` (transparent bg), `radius.md` |
| `apply_button_danger_theme` | OxButtonDanger | `feedback_error`, `text_inverse`, `radius.md` |
| `apply_button_small_theme` | OxButtonSmall | Same as primary + `radius.sm`, `font_size_xs` |
| `apply_button_large_theme` | OxButtonLarge | Same as primary + `radius.lg`, `font_size_md` |
| `apply_icon_button_theme` | OxIconButton | Transparent bg, `radius.md` |
| `apply_toggle_button_theme` | OxToggleButton | `surface_tertiary`, `border_hover/default`, `text_primary`, `radius.md` |

#### Inputs (`oxide_widgets::inputs`)

| Function | Widget | Semantic Tokens Used |
|----------|--------|---------------------|
| `apply_text_input_theme` | OxTextInput | `border_default/focus`, `surface_primary`, `text_primary`, `radius.md`, `font_size_sm` |
| `apply_text_area_theme` | OxTextArea | Same as text input |
| `apply_checkbox_theme` | OxCheckbox | `interactive_default`, `text_primary`, `font_size_sm` |
| `apply_radio_theme` | OxRadio | `interactive_default`, `text_primary`, `font_size_sm` |
| `apply_switch_theme` | OxSwitch | `interactive_default` |
| `apply_slider_theme` | OxSlider | `interactive_default` |

#### Display (`oxide_widgets::display`)

| Function | Widget | Semantic Tokens Used |
|----------|--------|---------------------|
| `apply_label_theme` | OxLabel | `text_primary`, `font_size_md` |
| `apply_label_title_theme` | OxLabelTitle | `text_primary`, `font_size_xxl` |
| `apply_label_subtitle_theme` | OxLabelSubtitle | `text_primary`, `font_size_lg` |
| `apply_label_body_theme` | OxLabelBody | `text_primary`, `font_size_md` |
| `apply_label_caption_theme` | OxLabelCaption | `text_secondary`, `font_size_xs` |
| `apply_label_secondary_theme` | OxLabelSecondary | `text_tertiary`, `font_size_sm` |
| `apply_label_link_theme` | OxLabelLink | `text_link`, `font_size_sm` |
| `apply_avatar_theme` | OxAvatar (all sizes) | `interactive_default`, `text_inverse` |
| `apply_badge_theme` | OxBadge | `surface_tertiary`, `text_secondary`, `radius.xl`, `font_size_xs` |
| `apply_badge_success_theme` | OxBadgeSuccess | `feedback_success` (10% alpha bg), `radius.xl` |
| `apply_badge_warning_theme` | OxBadgeWarning | `feedback_warning` (10% alpha bg), `radius.xl` |
| `apply_badge_error_theme` | OxBadgeError | `feedback_error` (10% alpha bg), `radius.xl` |
| `apply_badge_info_theme` | OxBadgeInfo | `feedback_info` (10% alpha bg), `radius.xl` |
| `apply_icon_theme` | OxIcon | `text_primary` (icon_color uniform) |
| `apply_icon_accent_theme` | OxIcon | `interactive_default` (icon_color uniform) |
| `apply_icon_secondary_theme` | OxIcon | `text_secondary` (icon_color uniform) |

#### Layout (`oxide_widgets::layout`)

| Function | Widget | Semantic Tokens Used |
|----------|--------|---------------------|
| `apply_card_theme` | OxCard | `surface_primary`, `radius.lg` |
| `apply_divider_theme` | OxDivider | `border_default` |

#### Feedback (`oxide_widgets::feedback`)

| Function | Widget | Semantic Tokens Used |
|----------|--------|---------------------|
| `apply_alert_theme` | OxAlert | `feedback_info` (10% alpha bg), `radius.md`, `font_size_sm` |
| `apply_alert_success_theme` | OxAlertSuccess | `feedback_success` (10% alpha bg) |
| `apply_alert_warning_theme` | OxAlertWarning | `feedback_warning` (10% alpha bg) |
| `apply_alert_error_theme` | OxAlertError | `feedback_error` (10% alpha bg) |
| `apply_progress_theme` | OxProgress | `border_default` |
| `apply_skeleton_theme` | OxSkeleton | `surface_tertiary` |

#### Overlay (`oxide_widgets::overlay`)

| Function | Widget | Semantic Tokens Used |
|----------|--------|---------------------|
| `apply_tooltip_theme` | OxTooltip | `surface_inverse`, `text_inverse`, `radius.sm` |
| `apply_popover_theme` | OxPopover | `surface_primary`, `radius.md` |
| `apply_drawer_theme` | OxDrawer | `surface_primary` |
| `apply_menu_theme` | OxMenu | `surface_primary`, `radius.md` |
| `apply_menu_item_theme` | OxMenuItem | `text_primary`, `font_size_sm` |

#### Navigation (`oxide_widgets::navigation`)

| Function | Widget | Semantic Tokens Used |
|----------|--------|---------------------|
| `apply_tab_theme` | OxTab | `text_primary`, `font_size_sm` |

#### Data (`oxide_widgets::data`)

| Function | Widget | Semantic Tokens Used |
|----------|--------|---------------------|
| `apply_list_item_theme` | OxListItem | `text_primary`, `font_size_sm` |
| `apply_table_header_theme` | OxTableHeader | `surface_secondary` |

## Theme Structure

Each theme contains:

- **mode** -- `ThemeMode::Light` or `ThemeMode::Dark`
- **colors** -- Semantic color palette (22 tokens) -- **applied at runtime**
- **radius** -- Border radius scale -- **applied at runtime**
- **opacity** -- Disabled, hover overlay, pressed overlay, backdrop -- **applied at runtime**
- **focus_ring** -- Color, width, offset for keyboard navigation indicators -- **applied at runtime**
- **spacing** -- Scale: none, xs, sm, md, lg, xl, xxl -- *defined, not yet applied at runtime*
- **typography** -- Font family, sizes, weights -- *defined, not yet applied at runtime*
- **shadows** -- Shadow scale (5 levels) -- *defined, not yet applied at runtime*
- **elevation** -- Layered depth scale (5 levels) -- *defined, not yet applied at runtime*
- **motion** -- Animation durations (fast, normal, slow) and easing -- *defined, not yet applied at runtime*

Token categories marked as *"defined, not yet applied"* are available for custom logic
via `ThemeEngine::current()` but are not automatically updated by `apply_*_theme`
functions. Runtime application for these categories is planned for a future release.

## Dark Mode

Every built-in theme has a dark variant. Toggle between light and dark:

```rust
// Toggle dark mode
let current = ThemeEngine::current();
let name = if current.name.contains("Dark") {
    current.name.replace(" Dark", "").to_string()
} else {
    format!("{} Dark", current.name)
};
ThemeEngine::switch_by_name(&name);
```

Dark themes have:
- Darker surface colors with appropriate contrast
- Brighter accent colors for visibility on dark backgrounds
- Stronger shadow opacities (since shadows need more opacity to be visible on dark surfaces)
- Higher hover/pressed overlay opacities for better feedback
- Higher backdrop opacity for modals

## Creating Custom Themes

```rust
use oxide_core::theme::{
    Theme, ThemeMode, SemanticColors, SpacingScale, RadiusScale,
    TypographyScale, ShadowScale, ElevationScale, MotionScale,
    OpacityScale, FocusRingTokens, hex_to_vec4,
};

fn my_theme() -> Theme {
    Theme {
        name: "MyBrand",
        mode: ThemeMode::Light,
        colors: SemanticColors {
            interactive_default: hex_to_vec4("#804DBF"),
            interactive_hover:   hex_to_vec4("#6B3FA3"),
            interactive_pressed: hex_to_vec4("#5A3589"),
            text_link:           hex_to_vec4("#804DBF"),
            border_focus:        hex_to_vec4("#804DBF"),
            feedback_info:       hex_to_vec4("#804DBF"),
            ..Default::default()
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale::default(),
        typography: TypographyScale::default(),
        shadows: ShadowScale::default(),
        elevation: ElevationScale::default(),
        motion: MotionScale::default(),
        opacity: OpacityScale::default(),
        focus_ring: FocusRingTokens {
            color: hex_to_vec4("#804DBF"),
            width: 2.0,
            offset: 2.0,
        },
    }
}

let mut themes = all_themes();
themes.push(my_theme());
ThemeEngine::init(themes);
```

## Figma Integration

Sync design tokens from Figma using the Oxide CLI:

```bash
oxide config set <FIGMA_TOKEN>
oxide sync --file-key <FILE_KEY>
oxide generate
```

See [Figma Workflow](figma-workflow.md) for details.

## Best Practices

1. **Initialize early** -- Call `ThemeEngine::init()` in `handle_startup`
2. **Name your widgets** -- Use `:=` syntax so `apply_theme` can target them
3. **Use semantic tokens** -- Reference `theme.colors.interactive_default` instead of hardcoded hex
4. **Batch apply** -- Use `ThemeTarget` lists to apply themes to all widgets at once
5. **Redraw after switch** -- Call `self.ui.redraw(cx)` after applying the theme
6. **DSL = base defaults** -- DSL definitions use OpenAI theme values; `apply_*_theme` functions patch uniforms at runtime
