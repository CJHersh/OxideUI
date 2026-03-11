# Theming Guide

## Current State

Components render from their **DSL defaults** using shadcn/ui light values. There is no runtime theme switching active in the showcase or example apps. The DSL is the single source of truth for component appearance.

The theming infrastructure (ThemeEngine, token structs, apply functions) exists in the codebase and is architecturally complete, but is not called at runtime due to a Makepad framework limitation described below.

## Design Tokens

All components use the shadcn/ui neutral palette:

| Category | Tokens |
|----------|--------|
| **Surfaces** | background `#FFFFFF`, secondary `#F5F5F5`, inverse `#171717` |
| **Text** | foreground `#0A0A0A`, secondary `#737373`, tertiary `#404040`, disabled `#A3A3A3` |
| **Interactive** | primary `#171717`, hover `#404040`, pressed `#525252` |
| **Borders** | default `#E5E5E5`, hover `#D4D4D4`, focus `#D4D4D4` |
| **Feedback** | success `#16A34A`, warning `#F59E0B`, error `#DC2626`, info `#3B82F6` |
| **Radius** | xs=2, sm=4, md=6 (buttons/inputs), lg=8 (cards), xl=12 |
| **Font** | Inter (Regular 400, Medium 500, SemiBold 600) |

These values are hardcoded in each component's `script_mod!` DSL block and also defined as typed Rust structs in `oxide-core/src/theme/tokens.rs`.

## Architecture (Planned)

The theming system is designed for runtime theme switching once Makepad's API supports it:

```
figma/tokens.json
    |  (oxide generate -- planned)
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

### What works today

- **ThemeEngine** -- fully functional: init, switch by name/index, cycle, signal actions
- **Theme structs** -- complete with all token scales (colors, spacing, radius, typography, shadows, elevation, motion, opacity, focus ring)
- **Pre-built themes** -- shadcn light/dark returned by `all_themes()`; OpenAI, Airbnb, Notion themes available via individual constructors
- **`apply_*_theme` functions** -- update `draw_bg` shader uniforms (background color, border color, border radius)
- **`set_widget_draw_uniform`** -- low-level function that patches GPU shader uniforms on a widget's draw call

### What does not work yet

- **Text color switching** -- Makepad's `WidgetRef::area()` returns the `draw_bg` area on composite widgets like Button. The `draw_text` area is inaccessible through the public API. This means `apply_*_theme` functions can update backgrounds and borders but not text colors.
- **Full dark mode** -- requires text color switching to look correct. Dark theme tokens are defined (`shadcn_dark_theme()`) but cannot be fully applied at runtime.

### Path forward

1. **Makepad API evolution** -- once `WidgetRef` exposes `draw_text` access or an `apply_over` equivalent, the existing apply functions can be extended to set text uniforms.
2. **Shader-based approach** -- embed a `mode` uniform in each widget's shader that flips colors in the `pixel()` function, bypassing external uniform updates entirely.

## ThemeEngine API Reference

The ThemeEngine is fully implemented and available for use in custom applications:

### Initialize

```rust
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::themes::all_themes;

ThemeEngine::init(all_themes());
```

### Switch Theme

```rust
ThemeEngine::switch_by_name("shadcn Dark");
ThemeEngine::switch(1);
ThemeEngine::next_theme();
ThemeEngine::prev_theme();
```

### Switch with Signal

```rust
ThemeEngine::switch_by_name_and_signal(cx, "shadcn Dark");
ThemeEngine::next_theme_and_signal(cx);
```

### Access Current Theme

```rust
let theme = ThemeEngine::current();
let primary = theme.colors.interactive_default;
let font = theme.typography.font_family;
```

## Available Themes

### Active (returned by `all_themes()`)

| Theme | Mode | Primary |
|-------|------|---------|
| **shadcn** | Light | `#171717` |
| **shadcn Dark** | Dark | `#F5F5F5` |

### Available (not in `all_themes()`, use individual constructors)

```rust
use oxide_core::theme::themes::{openai_theme, airbnb_theme, notion_theme};
```

| Theme | Mode | Accent |
|-------|------|--------|
| OpenAI | Light/Dark | `#10A37F` |
| Airbnb | Light/Dark | `#FF5A5F` |
| Notion | Light/Dark | `#346CA3` |

## Per-Widget Apply Functions

Each widget module provides `apply_*_theme` functions that map semantic tokens to `draw_bg` shader uniforms. These are the library's public API for theme application and will become fully functional once text color switching is resolved.

> **Note:** These functions currently update background colors, border colors, and border radius. Text colors remain at DSL defaults.

### Buttons (`oxide_widgets::buttons`)

| Function | Widget |
|----------|--------|
| `apply_button_theme` | OxButton |
| `apply_button_secondary_theme` | OxButtonSecondary |
| `apply_button_ghost_theme` | OxButtonGhost |
| `apply_button_danger_theme` | OxButtonDanger |
| `apply_button_outline_theme` | OxButtonOutline |
| `apply_button_small_theme` | OxButtonSmall |
| `apply_button_large_theme` | OxButtonLarge |
| `apply_icon_button_theme` | OxIconButton |
| `apply_toggle_button_theme` | OxToggleButton |

### Inputs (`oxide_widgets::inputs`)

| Function | Widget |
|----------|--------|
| `apply_text_input_theme` | OxTextInput |
| `apply_text_area_theme` | OxTextArea |
| `apply_checkbox_theme` | OxCheckbox |
| `apply_radio_theme` | OxRadio |
| `apply_switch_theme` | OxSwitch |
| `apply_slider_theme` | OxSlider |

### Display (`oxide_widgets::display`)

| Function | Widget |
|----------|--------|
| `apply_label_theme` | OxLabel |
| `apply_label_title_theme` | OxLabelTitle |
| `apply_label_subtitle_theme` | OxLabelSubtitle |
| `apply_label_body_theme` | OxLabelBody |
| `apply_label_caption_theme` | OxLabelCaption |
| `apply_label_secondary_theme` | OxLabelSecondary |
| `apply_label_link_theme` | OxLabelLink |
| `apply_avatar_theme` | OxAvatar (all sizes) |
| `apply_badge_theme` | OxBadge |
| `apply_badge_success_theme` | OxBadgeSuccess |
| `apply_badge_warning_theme` | OxBadgeWarning |
| `apply_badge_error_theme` | OxBadgeError |
| `apply_badge_info_theme` | OxBadgeInfo |
| `apply_icon_theme` | OxIcon |
| `apply_icon_accent_theme` | OxIcon (accent color) |
| `apply_icon_secondary_theme` | OxIcon (secondary color) |

### Layout (`oxide_widgets::layout`)

| Function | Widget |
|----------|--------|
| `apply_card_theme` | OxCard |
| `apply_divider_theme` | OxDivider |

### Feedback (`oxide_widgets::feedback`)

| Function | Widget |
|----------|--------|
| `apply_alert_theme` | OxAlert |
| `apply_alert_success_theme` | OxAlertSuccess |
| `apply_alert_warning_theme` | OxAlertWarning |
| `apply_alert_error_theme` | OxAlertError |
| `apply_progress_theme` | OxProgress |
| `apply_skeleton_theme` | OxSkeleton |

### Overlay (`oxide_widgets::overlay`)

| Function | Widget |
|----------|--------|
| `apply_tooltip_theme` | OxTooltip |
| `apply_popover_theme` | OxPopover |
| `apply_drawer_theme` | OxDrawer |
| `apply_menu_theme` | OxMenu |
| `apply_menu_item_theme` | OxMenuItem |

### Navigation (`oxide_widgets::navigation`)

| Function | Widget |
|----------|--------|
| `apply_tab_theme` | OxTab |

### Data (`oxide_widgets::data`)

| Function | Widget |
|----------|--------|
| `apply_list_item_theme` | OxListItem |
| `apply_table_header_theme` | OxTableHeader |

## Theme Structure

Each `Theme` struct contains:

| Scale | Fields | Status |
|-------|--------|--------|
| **colors** | 22 semantic color tokens | Defined in DSL + Theme struct |
| **radius** | xs, sm, md, lg, xl, full | Defined in DSL + Theme struct |
| **spacing** | none, xs, sm, md, lg, xl, xxl | Defined in Theme struct |
| **typography** | font family, 6 sizes, 3 weights | Defined in DSL + Theme struct |
| **shadows** | 5 levels (none through xl) | Defined in Theme struct |
| **elevation** | 5 levels (level0 through level4) | Defined in Theme struct |
| **motion** | fast, normal, slow durations + ease | Defined in Theme struct |
| **opacity** | disabled, hover overlay, pressed overlay, backdrop | Defined in Theme struct |
| **focus_ring** | color, width, offset | Defined in Theme struct |

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

See `examples/custom-theme/` for a working example.

## Figma Integration

Design tokens live in `figma/tokens.json`. The Oxide CLI can sync from Figma:

```bash
oxide config set <FIGMA_TOKEN>
oxide sync --file-key <FILE_KEY>
oxide generate   # codegen planned
```

## Best Practices

1. **Use Ox-prefixed components** -- they carry correct shadcn styling out of the box
2. **Stick to DSL defaults** -- avoid overriding colors with hardcoded hex unless intentional
3. **Name widgets with `:=`** -- enables future theme targeting when runtime switching is available
4. **Reference token values** -- when you need a color in custom code, use `ThemeEngine::current()` to access the semantic token structs rather than hardcoding hex values
5. **DSL = shadcn light** -- all widget DSL defaults use shadcn light palette values
