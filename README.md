# OxideUI

**A high-quality Rust component kit built on Makepad**

[![CI](https://github.com/CJHersh/OxideUI/actions/workflows/ci.yml/badge.svg)](https://github.com/CJHersh/OxideUI/actions/workflows/ci.yml)
![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux%20%7C%20Web%20%7C%20Android%20%7C%20iOS-lightgrey.svg)](https://github.com/makepad/makepad)

OxideUI is a production-grade component library for Rust, styled with the [shadcn/ui](https://ui.shadcn.com/) design language and powered by [Makepad](https://github.com/makepad/makepad)'s GPU-accelerated rendering. Components render with pixel-perfect defaults out of the box -- no configuration needed.

---

## Features

| Feature | Description |
|---------|-------------|
| **shadcn Design Language** | Components styled to match shadcn/ui -- Inter font, neutral palette, precise spacing and border radius from Figma design tokens. |
| **Cross-Platform** | Native macOS, Windows, Linux, Android, iOS, and Web -- powered by Makepad. |
| **GPU-Accelerated** | Built on Makepad's GPU rendering pipeline. SDF-based icons scale perfectly at any size. Hover, pressed, and focus states animate via shader uniforms. |
| **Type-Safe Token Architecture** | Theme tokens are fully typed Rust structs: semantic colors, spacing, radius, typography, shadows, elevation, motion, opacity, and focus ring scales. Designed for future runtime theme switching. |
| **Figma-First** | Design tokens sourced from Figma Variables. The Oxide CLI syncs tokens from Figma to JSON, with codegen to Rust structs planned. |

---

## Quick Start

### Prerequisites

- **Rust 1.70+** -- install via [rustup](https://rustup.rs/)
- **Platform deps** -- on Linux you may need: `sudo apt install libx11-dev libxcursor-dev libgl-dev libasound2-dev`

### Try it now

```bash
git clone https://github.com/CJHersh/OxideUI.git
cd OxideUI
cargo run -p oxide-showcase
```

### Add to your project

```toml
# Cargo.toml
[dependencies]
oxide-widgets = { git = "https://github.com/CJHersh/OxideUI", branch = "main" }
oxide-core = { git = "https://github.com/CJHersh/OxideUI", branch = "main" }
makepad-widgets = { git = "https://github.com/makepad/makepad", rev = "8b515338a2f50c5e0e2742cdc8b8ee7278aff371" }
```

### Basic example

OxideUI uses Makepad's `script_mod!` DSL for widget definitions. Components render with their built-in shadcn styling -- just use them:

```rust
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    View{
                        flow: Down padding: 32 spacing: 16

                        OxLabelTitle{ text: "Hello OxideUI" }

                        OxCard{
                            OxLabelCaption{ text: "A simple form" }
                            OxTextInput{ empty_text: "Your name..." }
                            View{
                                flow: Right spacing: 8
                                OxButton{ text: "Submit" }
                                OxButtonSecondary{ text: "Cancel" }
                            }
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
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {}
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

---

## Components

| Category | Components |
|----------|------------|
| **Buttons** | OxButton, OxButtonSecondary, OxButtonOutline, OxButtonGhost, OxButtonDanger, OxButtonSmall, OxButtonLarge, OxIconButton, OxButtonGroup, OxToggleButton |
| **Inputs** | OxTextInput, OxTextArea, OxCheckbox, OxRadio, OxSwitch, OxSlider |
| **Display** | OxLabel (Title, Subtitle, Body, Caption, Secondary, Link), OxBadge (Success, Warning, Error, Info), OxAvatar (Small, Large, XLarge), OxIcon |
| **Layout** | OxCard, OxDivider, OxStack, OxGrid |
| **Feedback** | OxAlert (Success, Warning, Error), OxProgress, OxSkeleton, OxSkeletonCircle, OxSkeletonText |
| **Navigation** | OxTabs, OxTab, OxBreadcrumb, OxBreadcrumbSeparator, OxPagination, OxStepper |
| **Overlay** | OxTooltip, OxPopover, OxDrawer, OxMenu, OxMenuItem |
| **Data** | OxTable, OxTableRow, OxTableHeader, OxList, OxListItem, OxTimeline |

All interactive components support **default**, **hover**, **pressed**, **focused**, and **disabled** states via GPU shader uniforms.

---

## Design Tokens

Components are styled using a structured token system based on shadcn/ui:

- **22 semantic color tokens** -- surfaces, text, interactive states, borders, feedback
- **Spacing scale** -- xs (8px) through xxl (32px)
- **Border radius scale** -- xs (2px) through full (pill)
- **Typography scale** -- Inter font, 6 size levels, 3 weight levels
- **Shadow & elevation** -- 5 levels each
- **Motion tokens** -- fast/normal/slow durations
- **Opacity tokens** -- disabled, hover overlay, pressed overlay, backdrop
- **Focus ring** -- color, width, offset

All token values are defined as typed Rust structs in `oxide-core` and sourced from `figma/tokens.json`. DSL defaults in each component bake in the shadcn light values directly.

### Theming architecture (planned)

The codebase includes a full theming infrastructure designed for future runtime theme switching:

- **`ThemeEngine`** -- global atomic theme switching with signal-based notifications
- **`Theme` struct** -- contains all token scales for a complete theme
- **`apply_*_theme` functions** -- per-widget functions that map semantic tokens to shader uniforms
- **`ThemeMap`** -- reusable registry of widget-to-theme bindings for batch application
- **Pre-built themes** -- shadcn (light/dark), OpenAI, Airbnb, Notion (all with dark variants)

Runtime switching currently updates background and border uniforms but cannot update text colors due to a Makepad framework limitation (composite widgets like Button expose only `draw_bg` through `WidgetRef`, not `draw_text`). Full theme switching will be enabled as the Makepad API evolves.

See [docs/theming.md](docs/theming.md) for the complete theming guide.

---

## Examples

```bash
# Component showcase -- all widgets with shadcn styling
cargo run -p oxide-showcase

# Static component demo
cargo run -p oxide-theme-switcher

# ThemeEngine API demonstration (custom purple theme)
cargo run -p oxide-custom-theme
```

---

## Roadmap

- **Component polish** -- pixel-perfect shadcn styling across all components
- **Dark mode** -- dark theme tokens are defined; pending Makepad API support for full text color switching
- **Runtime theme switching** -- ThemeEngine infrastructure is built; waiting on `draw_text` access through `WidgetRef`
- **Brand themes** -- OpenAI, Airbnb, Notion theme definitions exist in `oxide-core`, ready to activate
- **Figma codegen** -- `oxide generate` to produce Theme structs from `figma/tokens.json`

---

## Project Structure

```
OxideUI/
├── crates/
│   ├── oxide-core/         # Theme tokens, ThemeEngine, semantic colors
│   ├── oxide-widgets/      # UI components + per-widget apply_theme functions
│   │   └── src/
│   │       ├── theme_apply.rs  # ThemeMap, set_widget_draw_uniform, v4
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
│   ├── showcase/            # Component gallery with section navigation
│   ├── theme-switcher/      # Static component demo
│   └── custom-theme/        # ThemeEngine API example
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
