# OxideUI

**The industry's first Multi-Skin Rust component library**

[![CI](https://github.com/oxide-ui/oxide-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/oxide-ui/oxide-ui/actions/workflows/ci.yml)
![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux%20%7C%20Web%20%7C%20Android%20%7C%20iOS-lightgrey.svg)](https://github.com/makepad/makepad)

Build once, then toggle between pre-built themes at runtime. OxideUI is a high-performance, type-safe component kit for the Rust ecosystem—powered by [Makepad](https://github.com/makepad/makepad) (dev branch) and Figma Variables.

---

## Overview

OxideUI bridges the gap between high-level design systems and low-level performance. It allows you to switch between world-class design languages (OpenAI, Airbnb, Notion, and more) or inject your own—all at runtime with GPU-accelerated rendering.

```toml
# Cargo.toml
[dependencies]
oxide-widgets = { git = "https://github.com/oxide-ui/oxide-ui", branch = "main" }
oxide-core = { git = "https://github.com/oxide-ui/oxide-ui", branch = "main" }
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "dev" }
```

---

## Features

| Feature | Description |
|---------|-------------|
| **Runtime Multi-Theme** | Switch between 6 built-in themes (OpenAI, Airbnb, Notion -- each with light and dark variants) or define your own. GPU uniforms are patched directly—no widget rebuild needed. |
| **Dark Mode** | First-class dark mode support. Every built-in theme ships with a carefully crafted dark variant. Toggle between light and dark at runtime. |
| **Figma-First** | Sync Figma Variables directly to Rust structs via the Oxide CLI. Design and code stay in sync. |
| **Cross-Platform** | Native macOS, Windows, Linux, Android, iOS, and Web—powered by Makepad. |
| **GPU-Accelerated** | Built on Makepad's GPU-accelerated rendering pipeline. SDF-based icons and theme changes patch shader uniforms directly. |
| **Type-Safe** | Theme tokens are fully typed Rust structs generated from JSON design tokens. Includes semantic colors, spacing, radius, typography, shadows, elevation, motion, opacity, and focus ring tokens. |

---

## Quick Start

### Option 1: CLI (recommended)

```bash
git clone https://github.com/oxide-ui/oxide-ui.git
cd oxide-ui
cargo install --path crates/oxide-cli
oxide new my-app
cd my-app
cargo run
```

### Option 2: Add to existing project

Add to your `Cargo.toml`:

```toml
[dependencies]
oxide-widgets = { git = "https://github.com/oxide-ui/oxide-ui", branch = "main" }
oxide-core = { git = "https://github.com/oxide-ui/oxide-ui", branch = "main" }
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "dev" }
```

### Basic example

OxideUI uses Makepad's `script_mod!` DSL for widget definitions and `ThemeEngine` for runtime theme switching:

```rust
use makepad_widgets::*;
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::themes::all_themes;
use oxide_widgets::ThemeMap;
use oxide_widgets::{buttons, display, layout};

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    View{
                        flow: Down padding: 32 spacing: 24
                        title := OxLabelTitle{ text: "Hello OxideUI" }
                        card := OxCard{
                            submit := OxButton{ text: "Submit" }
                            cancel := OxButtonSecondary{ text: "Cancel" }
                        }
                    }
                }
            }
        }
    }
    app
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        crate::oxide_widgets::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }

    fn theme_map(&self) -> ThemeMap {
        ThemeMap::builder(&self.ui)
            .add(live_id!(title), display::apply_label_title_theme)
            .add(live_id!(card), layout::apply_card_theme)
            .add(live_id!(submit), buttons::apply_button_theme)
            .add(live_id!(cancel), buttons::apply_button_secondary_theme)
            .build()
    }

    fn switch_theme(&self, cx: &mut Cx, name: &str) {
        ThemeEngine::switch_by_name(name);
        let theme = ThemeEngine::current();
        self.theme_map().apply_all(cx, &theme);
        self.ui.redraw(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        ThemeEngine::init(all_themes());
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

Key points:
- Widgets must be **named** with `:=` syntax (e.g. `submit := OxButton{}`) so they can be targeted for theme patching.
- Call `ThemeEngine::init(all_themes())` in `handle_startup` before switching themes.
- Use `ThemeMap` to build a reusable mapping of widget IDs to theme-apply functions, then call `apply_all()` on each theme switch.

---

## Components

| Category | Components |
|----------|------------|
| **Buttons** | OxButton, OxButtonSecondary, OxButtonGhost, OxButtonDanger, OxButtonSmall, OxButtonLarge, OxIconButton, OxButtonGroup, OxToggleButton |
| **Inputs** | OxTextInput, OxTextArea, OxCheckbox, OxRadio, OxSwitch, OxSlider |
| **Display** | OxLabel (Title, Subtitle, Body, Caption, Secondary, Link), OxBadge (Success, Warning, Error, Info), OxAvatar (Small, Large, XLarge), OxIcon |
| **Layout** | OxCard, OxDivider, OxStack, OxGrid |
| **Feedback** | OxAlert (Success, Warning, Error), OxProgress, OxSkeleton, OxSkeletonCircle, OxSkeletonText |
| **Navigation** | OxTabs, OxTab, OxBreadcrumb, OxBreadcrumbSeparator, OxPagination, OxStepper |
| **Overlay** | OxTooltip, OxPopover, OxDrawer, OxMenu, OxMenuItem |
| **Data** | OxTable, OxTableRow, OxTableHeader, OxList, OxListItem, OxTimeline |

All components with backgrounds support: **default**, **hover**, **pressed** states via GPU shader uniforms.

---

## Themes

| Theme | Light Accent | Dark Accent | Description |
|-------|-------------|-------------|-------------|
| **OpenAI** | `#10A37F` | `#19C99D` | Clean, professional teal (base/default) |
| **Airbnb** | `#FF5A5F` | `#FF7B7F` | Warm, inviting coral |
| **Notion** | `#346CA3` | `#529CCA` | Calm, focused blue |

All 6 themes (3 light + 3 dark) are built-in and switchable at runtime. Each theme includes:

- **22 semantic color tokens** (surfaces, text, interactive, borders, feedback)
- **Spacing scale** (4px grid: xs through xxl)
- **Border radius scale** (per-theme personality)
- **Typography scale** (font sizes, line heights, weights)
- **Shadow & elevation scales** (5 levels each)
- **Motion tokens** (fast/normal/slow durations)
- **Opacity tokens** (disabled, hover overlay, pressed overlay, backdrop)
- **Focus ring tokens** (color, width, offset)

### How theming works

1. **JSON design tokens** (`figma/tokens.json`) are the single source of truth
2. **Codegen** (`oxide generate`) produces typed `Theme` structs in `oxide-core`
3. **`ThemeEngine`** manages the active theme globally (atomic switching)
4. **Per-widget `apply_*_theme` functions** map semantic tokens to shader uniforms via `set_widget_draw_uniform`
5. **`ThemeMap`** builds a reusable registry of widget-to-theme bindings; call `apply_all()` to switch in one line

DSL definitions bake the OpenAI light theme as defaults. When you switch themes, only the shader uniforms that differ are patched — no widget tree rebuild.

### Dark mode

Toggle between light and dark with a single call:

```rust
// Switch to dark variant of current theme
let current = ThemeEngine::current();
let dark_name = format!("{} Dark", current.name);
ThemeEngine::switch_by_name(&dark_name);
```

---

## Documentation

- [Getting Started](docs/getting-started.md)
- [Theming Guide](docs/theming.md)
- [Button Component](docs/components/button.md)
- [Figma Workflow](docs/figma-workflow.md)
- [Project Plan](PROJECT_PLAN.md)

---

## Examples

```bash
# Full component showcase
cargo run -p oxide-showcase

# Theme switcher demo
cargo run -p oxide-theme-switcher

# Custom theme example
cargo run -p oxide-custom-theme
```

---

## Project Structure

```
oxide-ui/
├── crates/
│   ├── oxide-core/         # Theme tokens, ThemeEngine, semantic colors
│   ├── oxide-widgets/      # UI components + per-widget apply_theme functions
│   │   └── src/
│   │       ├── theme_apply.rs  # ThemeMap, apply_theme, set_widget_draw_uniform
│   │       ├── buttons/        # OxButton, OxIconButton, OxToggleButton, etc.
│   │       ├── inputs/         # OxTextInput, OxCheckbox, OxSlider, etc.
│   │       ├── display/        # OxLabel, OxAvatar, OxBadge, OxIcon
│   │       ├── layout/         # OxCard, OxDivider, OxStack, OxGrid
│   │       ├── feedback/       # OxAlert, OxProgress, OxSkeleton
│   │       ├── navigation/     # OxTabs, OxBreadcrumb, OxPagination
│   │       ├── overlay/        # OxTooltip, OxPopover, OxDrawer, OxMenu
│   │       └── data/           # OxTable, OxList, OxTimeline
│   └── oxide-cli/          # Figma sync, codegen, scaffolding
├── examples/
│   ├── showcase/            # Component gallery
│   ├── theme-switcher/      # Theme switching demo
│   └── custom-theme/        # Custom theme example
├── docs/
└── figma/                   # Design tokens (tokens.json, tokens.schema.json)
```

---

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.

---

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md), [Code of Conduct](CODE_OF_CONDUCT.md), and [Project Plan](PROJECT_PLAN.md) before submitting a PR.
