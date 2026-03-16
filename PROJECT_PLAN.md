# OxideUI Project Plan

> **Note:** This document describes the long-term vision for OxideUI. The current implementation uses Makepad's `script_mod!` DSL for widget definitions rather than the Rust builder APIs shown in some sections below. See the [README](README.md) for the current state of the project.

## 1. Vision & Goals

### Mission Statement

OxideUI is the industry's first **Multi-Skin Rust component library**. Build once, then toggle between pre-built themes with a single line of code. We bridge the gap between high-level design systems and low-level performance—powered by Figma Variables, zero-cost abstractions, and compile-time safety.

### Core Principles

- **Design-First**: Figma Variables drive our token system; designers and developers speak the same language.
- **Zero-Cost Abstractions**: Rust's type system ensures no runtime overhead for theme switching.
- **Atomic Theming**: Themes switch atomically—no partial state, no flicker.
- **Composability**: Small, focused crates that compose into a complete design system.
- **Platform Agnostic**: Built on Makepad for native macOS, Windows, Linux, Android, iOS, and Web.

### Target Users

- **Rust GUI developers** building production applications with Makepad
- **Design system maintainers** who need Figma-to-code parity
- **Startups & enterprises** requiring branded, themeable UIs without vendor lock-in
- **Open source contributors** extending the component ecosystem

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           DESIGN LAYER                                      │
│  Figma Variables  │  Token JSON  │  Design Specs  │  Component Figma Files   │
└─────────────────────────────────────────────────────────────────────────────┘
                                        │
                                        │ oxide sync / oxide generate
                                        ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           OXIDE CLI (oxide-cli)                              │
│  init │ sync │ generate │ watch │ validate │ themes list │ new               │
└─────────────────────────────────────────────────────────────────────────────┘
                                        │
                                        │ generates / validates
                                        ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           OXIDE CORE (oxide-core)                            │
│  ThemeEngine │ Token Structs │ Semantic Tokens │ Theme Switching             │
└─────────────────────────────────────────────────────────────────────────────┘
                                        │
                                        │ depends on
                                        ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           OXIDE WIDGETS (oxide-widgets)                      │
│  OxButton │ OxTextInput │ OxLabel │ OxBadge │ OxAvatar │ OxIcon │ ...        │
└─────────────────────────────────────────────────────────────────────────────┘
                                        │
                                        │ depends on
                                        ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           MAKEPAD (makepad-widgets)                          │
│  View │ Button │ TextInput │ Label │ Image │ Layout │ Events │ Shaders       │
└─────────────────────────────────────────────────────────────────────────────┘
                                        │
                                        ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           APP (your application)                            │
│  AppState │ Screens │ Navigation │ Business Logic                            │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Crate Dependency Diagram

```
                    ┌──────────────────┐
                    │   oxide-cli      │
                    │  (dev tooling)   │
                    └────────┬─────────┘
                             │ reads/writes
                             ▼
┌──────────────────┐   ┌──────────────────┐
│  oxide-widgets   │──▶│   oxide-core     │
│  (UI components) │   │ (theming, tokens)│
└────────┬─────────┘   └────────┬─────────┘
         │                      │
         │                      │
         └──────────┬───────────┘
                    │
                    ▼
         ┌──────────────────────┐
         │   makepad-widgets     │
         │  git = "makepad"     │
         │  branch = "dev"      │
         └──────────────────────┘
```

**Dependency chain:**
- `oxide-cli` → standalone binary, no crate deps
- `oxide-widgets` → `oxide-core` → `makepad-widgets`
- Apps → `oxide-widgets` (and optionally `oxide-core` for custom themes)

---

## 4. Full Project Directory Tree

```
OxideUI/
├── Cargo.toml                    # workspace manifest
├── README.md
├── PROJECT_PLAN.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── CHANGELOG.md
├── LICENSE-MIT
├── LICENSE-APACHE
│
├── crates/
│   ├── oxide-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── theme/
│   │           ├── mod.rs
│   │           ├── tokens.rs         # Theme, SemanticColors, all token scales
│   │           ├── engine.rs         # ThemeEngine, ThemeChangedAction
│   │           ├── tests.rs
│   │           └── themes/
│   │               ├── mod.rs
│   │               ├── shadcn.rs     # shadcn light + dark
│   │               ├── openai.rs
│   │               ├── airbnb.rs
│   │               └── notion.rs
│   │
│   ├── oxide-widgets/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── theme_apply.rs        # ThemeMap, set_widget_draw_uniform
│   │       ├── buttons/
│   │       │   ├── mod.rs
│   │       │   ├── button.rs         # OxButton + 6 variants
│   │       │   ├── icon_button.rs
│   │       │   ├── button_group.rs   # [planned]
│   │       │   └── toggle_button.rs
│   │       ├── inputs/
│   │       │   ├── mod.rs
│   │       │   ├── text_input.rs
│   │       │   ├── text_area.rs
│   │       │   ├── checkbox.rs
│   │       │   ├── radio.rs
│   │       │   ├── switch.rs
│   │       │   └── slider.rs
│   │       ├── display/
│   │       │   ├── mod.rs
│   │       │   ├── label.rs          # OxLabel + 6 variants
│   │       │   ├── badge.rs          # OxBadge + 4 variants
│   │       │   ├── avatar.rs         # OxAvatar + 3 size variants
│   │       │   └── icon.rs
│   │       ├── layout/
│   │       │   └── mod.rs            # OxCard, OxDivider, OxStack [planned], OxGrid [planned]
│   │       ├── feedback/
│   │       │   └── mod.rs            # OxAlert variants, OxProgress, OxSkeleton variants
│   │       ├── navigation/
│   │       │   └── mod.rs            # [planned] OxTabs, OxBreadcrumb, OxPagination, OxStepper
│   │       ├── overlay/
│   │       │   └── mod.rs            # [planned] OxTooltip, OxPopover, OxDrawer, OxMenu
│   │       └── data/
│   │           └── mod.rs            # [planned] OxTable, OxList, OxTimeline
│   │
│   └── oxide-cli/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── codegen/
│           ├── commands/
│           ├── figma/
│           └── parser/
│
├── examples/
│   ├── showcase/                     # Component gallery with section navigation
│   ├── theme-switcher/               # Static component demo
│   └── custom-theme/                 # ThemeEngine API example
│
├── docs/
│   ├── getting-started.md
│   ├── theming.md
│   ├── figma-workflow.md
│   └── components/
│       └── button.md
│
└── figma/
    ├── tokens.json                   # Design tokens for all themes
    └── tokens.schema.json            # JSON Schema for token files
```

---

## 5. Theming System

### Two-Layer Token Hierarchy

**Layer 1: Primitive Tokens** — Raw design values (hex, px, etc.)  
**Layer 2: Semantic Tokens** — Named by purpose (e.g. `background.primary`, `text.primary`)

Semantic tokens reference primitives; themes swap primitives to achieve different looks.

### Rust Struct Definitions (as implemented)

```rust
// oxide-core/src/theme/tokens.rs

pub struct Theme {
    pub name: &'static str,
    pub mode: ThemeMode,
    pub colors: SemanticColors,
    pub spacing: SpacingScale,
    pub radius: RadiusScale,
    pub typography: TypographyScale,
    pub shadows: ShadowScale,
    pub elevation: ElevationScale,
    pub motion: MotionScale,
    pub opacity: OpacityScale,
    pub focus_ring: FocusRingTokens,
}

pub struct SemanticColors {
    // Surfaces
    pub surface_primary: Vec4,
    pub surface_secondary: Vec4,
    pub surface_tertiary: Vec4,
    pub surface_inverse: Vec4,
    // Text
    pub text_primary: Vec4,
    pub text_secondary: Vec4,
    pub text_tertiary: Vec4,
    pub text_disabled: Vec4,
    pub text_inverse: Vec4,
    pub text_link: Vec4,
    // Interactive
    pub interactive_default: Vec4,
    pub interactive_hover: Vec4,
    pub interactive_pressed: Vec4,
    pub interactive_disabled: Vec4,
    // Borders
    pub border_default: Vec4,
    pub border_hover: Vec4,
    pub border_focus: Vec4,
    pub border_error: Vec4,
    // Feedback
    pub feedback_success: Vec4,
    pub feedback_warning: Vec4,
    pub feedback_error: Vec4,
    pub feedback_info: Vec4,
}

pub struct SpacingScale { pub none: f64, pub xs: f64, pub sm: f64, pub md: f64, pub lg: f64, pub xl: f64, pub xxl: f64 }
pub struct RadiusScale  { pub none: f64, pub xs: f64, pub sm: f64, pub md: f64, pub lg: f64, pub xl: f64, pub full: f64 }
```

### ThemeEngine Design (as implemented)

```rust
// oxide-core/src/theme/engine.rs

/// Global theme engine backed by static storage. Switching is atomic.
pub struct ThemeEngine;

impl ThemeEngine {
    pub fn init(themes: Vec<Theme>);
    pub fn current() -> Theme;
    pub fn switch(index: usize);
    pub fn switch_by_name(name: &str) -> bool;
    pub fn next_theme();
    pub fn prev_theme();
    pub fn switch_and_signal(cx: &mut Cx, index: usize);
    // ... _and_signal variants post ThemeChangedAction
}
```

---

## 6. Component Specifications (Tier 1)

> Components are defined as `script_mod!` DSL aliases over Makepad base widgets. They are used directly in DSL markup rather than via Rust builder APIs.

### Buttons

| Component       | Variants                                      | Sizes          | States                          | DSL Usage                          |
|----------------|-----------------------------------------------|----------------|----------------------------------|--------------------------------------|
| **OxButton**   | Primary, Secondary, Ghost, Danger, Outline    | Small, Medium, Large | default, hover, pressed, focused, disabled | `OxButton{ text: "Label" }`         |
| **OxIconButton** | —                                           | 40x40          | default, hover, pressed          | `OxIconButton{}`                     |
| **OxButtonGroup** | [planned]                                  | inherits       | —                                | `OxButtonGroup{ ... }`               |
| **OxToggleButton** | —                                          | —              | default, selected, hover, pressed | `OxToggleButton{ text: "Toggle" }`   |

**States:** `default` | `hover` | `pressed` | `focused` | `disabled`

---

### Inputs

| Component      | Variants | States                          | DSL Usage                          |
|----------------|----------|----------------------------------|--------------------------------------|
| **OxTextInput** | Single-line | default, focused               | `OxTextInput{ empty_text: "Type..." }` |
| **OxTextArea** | Multiline   | default, focused               | `OxTextArea{ empty_text: "..." }`    |
| **OxCheckbox** | —          | default, checked                 | `OxCheckbox{ text: "Accept" }`       |
| **OxRadio**    | —          | default, selected                | `OxRadio{ text: "Option A" }`       |
| **OxSwitch**   | —          | on, off                          | `OxSwitch{}`                         |
| **OxSlider**   | —          | default                          | `OxSlider{}`                         |

---

### Display

| Component   | Variants                                      | Sizes / Shapes       | States        | DSL Usage                          |
|-------------|-----------------------------------------------|----------------------|---------------|--------------------------------------|
| **OxLabel** | Title, Subtitle, Body, Caption, Secondary, Link | —                  | —             | `OxLabelTitle{ text: "Hello" }`      |
| **OxBadge**  | Default, Success, Warning, Error, Info        | —                    | —             | `OxBadgeSuccess{}`                   |
| **OxAvatar** | Default, Small, Large, XLarge                 | 24-64px              | —             | `OxAvatar{}`                         |
| **OxIcon**   | Check, Close, Search, etc.                    | Small, Medium, Large | —             | `OxIcon{}`                           |

---

## 7. Figma-to-Code Workflow

### CLI Commands

| Command              | Description                                                                 |
|----------------------|-----------------------------------------------------------------------------|
| `oxide init`         | Initialize OxideUI in current project; creates `oxide.toml`, `figma/` dir   |
| `oxide sync`         | Pull Figma variables from file/API → `figma/variables.json`                 |
| `oxide generate`     | Generate Rust theme structs from `variables.json` → `oxide-core` or app     |
| `oxide watch`        | Watch `figma/` for changes; auto-run sync + generate                         |
| `oxide validate`     | Validate token JSON against schema; check theme completeness                |
| `oxide themes list`  | List available built-in themes                                              |
| `oxide new <name>`   | Scaffold new app with OxideUI deps and example                              |

### Token JSON Schema

```json
{
  "$schema": "https://oxide-ui.dev/schemas/tokens-v1.json",
  "primitives": {
    "color": {
      "brand-500": "#10A37F",
      "neutral-50": "#FAFAFA",
      "neutral-900": "#171717"
    },
    "spacing": {
      "xs": 4,
      "sm": 8,
      "md": 16,
      "lg": 24,
      "xl": 32
    },
    "radius": {
      "sm": 4,
      "md": 8,
      "lg": 12
    }
  },
  "semantic": {
    "color": {
      "background.primary": "{primitives.color.neutral-50}",
      "accent.primary": "{primitives.color.brand-500}"
    }
  }
}
```

### Figma Variable Structure

- **Collections:** `primitives`, `semantic`
- **Modes:** Optional (e.g. `light`, `dark`) — each mode = one theme
- **Variable types:** Color, Number, String
- **Naming:** `category.token` (e.g. `color.background.primary`)

---

## 8. Development Roadmap

| Phase | Version | Focus                    | Deliverables                                           |
|-------|---------|---------------------------|--------------------------------------------------------|
| **1** | v0.1    | Foundation                | oxide-core, Theme structs, ThemeEngine, 1 built-in theme |
| **2** | v0.2    | Feedback components       | OxButton, OxLabel, OxBadge, OxIcon, states (hover/pressed) |
| **3** | v0.3    | Navigation                | OxTabs, OxBreadcrumb, OxMenu, OxDrawer                  |
| **4** | v0.4    | Data display & inputs     | OxTextInput, OxCheckbox, OxRadio, OxSwitch, OxSlider, OxTable |
| **5** | v0.5    | CLI & Figma workflow      | oxide-cli (init, sync, generate, watch, validate)       |
| **6** | v1.0    | Polish & release          | Docs, examples, 5+ themes, API stability, crates.io   |

---

## 9. API Reference

### Prelude Imports

```rust
use oxide_core::prelude::*;
// Re-exports: ThemeEngine, ThemeChangedAction, Theme, SemanticColors,
//             all token scale types, hex_to_vec4, makepad_widgets::*
```

### Theme Access Patterns

```rust
use oxide_core::prelude::*;

// Initialize themes at startup
ThemeEngine::init(all_themes());

// Get the current theme
let theme = ThemeEngine::current();
let bg = theme.colors.surface_primary;

// Switch and notify widgets
ThemeEngine::switch_by_name_and_signal(cx, "shadcn Dark");
```

### Component DSL Usage

Components are used in `script_mod!` markup:

```rust
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // ...
    OxButton{ text: "Submit" }
    OxButtonSecondary{ text: "Cancel" }
    OxLabelTitle{ text: "Welcome" }
    OxTextInput{ empty_text: "Enter email..." }
    OxCard{
        OxLabelBody{ text: "Card content" }
    }
}
```

---

## 10. Contributing Guidelines

### Code Style

- **Rust:** `rustfmt` + `clippy` (default settings)
- **Naming:** `Ox` prefix for all public widgets; `snake_case` for functions/fields
- **Docs:** All public items must have `///` doc comments

### Commit Messages

- Format: `type(scope): description` (e.g. `feat(button): add Danger variant`)
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- Scope: crate or component name

### PR Process

1. Open issue or comment on existing one
2. Fork, branch from `main`
3. Implement + add/update tests
4. Run `cargo test` and `cargo clippy`
5. Submit PR with description; link issue
6. Address review feedback

### Component Checklist

- [ ] Implements all Tier 1 states (default, hover, pressed, focused, disabled)
- [ ] Uses semantic tokens from theme (no hardcoded colors)
- [ ] Supports size variants where applicable
- [ ] Has `live_design!` / DSL integration
- [ ] Documented with examples
- [ ] Showcase example updated

---

## 11. Open Source Best Practices

### Repo Setup

- `README.md` with badges, quick start, examples
- `LICENSE-MIT` and `LICENSE-APACHE-2.0` (dual license)
- `CONTRIBUTING.md` (link to Contributing Guidelines above)
- `CODE_OF_CONDUCT.md` (Contributor Covenant)
- `.github/` workflows: CI (test, clippy, fmt), release automation

### Documentation

- Inline docs on all public APIs
- `docs/` for guides (getting-started, theming, components, figma-workflow)
- `examples/` as living documentation
- Changelog (`CHANGELOG.md`) per release

### Community

- GitHub Discussions for Q&A and ideas
- Issue templates (bug, feature, docs)
- Welcoming to first-time contributors (good first issue labels)

### Quality

- Unit tests for theme/token logic
- Integration tests for critical components
- Benchmarks for hot paths (optional)
- Security: no `unsafe` unless justified; audit dependencies

### Versioning

- **SemVer:** MAJOR.MINOR.PATCH
- Pre-1.0: MINOR = breaking, PATCH = additive
- Post-1.0: MAJOR = breaking, MINOR = additive, PATCH = patch

### Release Process

1. Update `CHANGELOG.md`
2. Bump version in `Cargo.toml` (all crates)
3. Tag: `git tag v0.1.0`
4. Push: `git push origin v0.1.0`
5. Publish: `cargo publish -p oxide-core` then `oxide-widgets` then `oxide-cli`
6. Create GitHub Release with notes

---

## 12. Appendix

### hex_to_vec4 Utility Function

```rust
/// Convert hex color string (#RRGGBB or #RRGGBBAA) to [f32; 4] (RGBA 0..1).
pub fn hex_to_vec4(hex: &str) -> [f32; 4] {
    let hex = hex.trim_start_matches('#');
    let (r, g, b, a) = match hex.len() {
        6 => (
            u8::from_str_radix(&hex[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&hex[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&hex[4..6], 16).unwrap_or(0),
            255u8,
        ),
        8 => (
            u8::from_str_radix(&hex[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&hex[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&hex[4..6], 16).unwrap_or(0),
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255),
        ),
        _ => (0, 0, 0, 255),
    };
    [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ]
}
```

### Useful Resources

- [Makepad GitHub](https://github.com/makepad/makepad) — `dev` branch
- [Figma Variables](https://help.figma.com/hc/en-us/articles/15339657135383-Guide-to-variables-in-Figma)
- [Design Tokens](https://design-tokens.github.io/community-group/format/) — W3C spec
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
