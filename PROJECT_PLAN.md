# OxideUI Project Plan

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
oxide-ui/
├── Cargo.toml                    # workspace manifest
├── README.md
├── PROJECT_PLAN.md
├── LICENSE-MIT
├── LICENSE-APACHE
│
├── crates/
│   ├── oxide-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── theme.rs
│   │       ├── tokens/
│   │       │   ├── mod.rs
│   │       │   ├── colors.rs
│   │       │   ├── spacing.rs
│   │       │   ├── radius.rs
│   │       │   ├── typography.rs
│   │       │   └── shadows.rs
│   │       └── engine.rs
│   │
│   ├── oxide-widgets/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── prelude.rs
│   │       ├── button/
│   │       │   ├── mod.rs
│   │       │   ├── ox_button.rs
│   │       │   ├── ox_icon_button.rs
│   │       │   ├── ox_button_group.rs
│   │       │   └── ox_toggle_button.rs
│   │       ├── input/
│   │       │   ├── mod.rs
│   │       │   ├── ox_text_input.rs
│   │       │   ├── ox_text_area.rs
│   │       │   ├── ox_checkbox.rs
│   │       │   ├── ox_radio.rs
│   │       │   ├── ox_switch.rs
│   │       │   └── ox_slider.rs
│   │       └── display/
│   │           ├── mod.rs
│   │           ├── ox_label.rs
│   │           ├── ox_badge.rs
│   │           ├── ox_avatar.rs
│   │           └── ox_icon.rs
│   │
│   └── oxide-cli/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── commands/
│           │   ├── mod.rs
│           │   ├── init.rs
│           │   ├── sync.rs
│           │   ├── generate.rs
│           │   ├── watch.rs
│           │   ├── validate.rs
│           │   ├── themes.rs
│           │   └── new.rs
│           └── config.rs
│
├── examples/
│   ├── showcase/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   ├── theme-switcher/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── custom-theme/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── docs/
│   ├── getting-started.md
│   ├── theming.md
│   ├── components.md
│   ├── figma-workflow.md
│   └── api/
│       └── (generated docs)
│
└── figma/
    ├── oxide-design-tokens.fig
    ├── variables.json           # exported Figma variables
    └── components/
        └── (Figma component definitions)
```

---

## 5. Theming System

### Two-Layer Token Hierarchy

**Layer 1: Primitive Tokens** — Raw design values (hex, px, etc.)  
**Layer 2: Semantic Tokens** — Named by purpose (e.g. `background.primary`, `text.primary`)

Semantic tokens reference primitives; themes swap primitives to achieve different looks.

### Rust Struct Definitions

```rust
// oxide-core/src/theme.rs

/// A complete theme definition with all token scales.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub semantic_colors: SemanticColors,
    pub spacing: SpacingScale,
    pub radius: RadiusScale,
    pub typography: TypographyScale,
    pub shadows: ShadowScale,
}

/// Semantic color mappings (Layer 2).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SemanticColors {
    pub background: BackgroundColors,
    pub surface: SurfaceColors,
    pub text: TextColors,
    pub border: BorderColors,
    pub accent: AccentColors,
    pub feedback: FeedbackColors,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundColors {
    pub primary: [f32; 4],
    pub secondary: [f32; 4],
    pub tertiary: [f32; 4],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SurfaceColors {
    pub raised: [f32; 4],
    pub overlay: [f32; 4],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextColors {
    pub primary: [f32; 4],
    pub secondary: [f32; 4],
    pub tertiary: [f32; 4],
    pub inverse: [f32; 4],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BorderColors {
    pub default: [f32; 4],
    pub strong: [f32; 4],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccentColors {
    pub primary: [f32; 4],
    pub primary_hover: [f32; 4],
    pub primary_pressed: [f32; 4],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackColors {
    pub success: [f32; 4],
    pub warning: [f32; 4],
    pub error: [f32; 4],
    pub info: [f32; 4],
}

/// Spacing scale (4px base).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpacingScale {
    pub xs: f32,   // 4
    pub sm: f32,   // 8
    pub md: f32,   // 16
    pub lg: f32,   // 24
    pub xl: f32,   // 32
    pub xxl: f32,  // 48
}

/// Border radius scale.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RadiusScale {
    pub none: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub full: f32,
}

/// Typography scale.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypographyScale {
    pub font_family: String,
    pub title_lg: TypographyToken,
    pub title_md: TypographyToken,
    pub title_sm: TypographyToken,
    pub body_lg: TypographyToken,
    pub body_md: TypographyToken,
    pub body_sm: TypographyToken,
    pub caption: TypographyToken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypographyToken {
    pub size: f32,
    pub weight: u16,
    pub line_height: f32,
}

/// Shadow scale.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShadowScale {
    pub none: Shadow,
    pub sm: Shadow,
    pub md: Shadow,
    pub lg: Shadow,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: [f32; 4],
}
```

### ThemeEngine Design (Atomic Switching)

```rust
// oxide-core/src/engine.rs

/// Global theme engine. Switching is atomic—all components
/// read the new theme on next frame; no partial updates.
pub struct ThemeEngine {
    current: Arc<RwLock<Theme>>,
    available: HashMap<String, Theme>,
}

impl ThemeEngine {
    pub fn switch(&self, theme_id: &str) -> Result<(), ThemeError>;
    pub fn current(&self) -> Theme;
    pub fn register(&mut self, theme: Theme);
}
```

---

## 6. Component Specifications (Tier 1)

### Buttons

| Component       | Variants                                      | Sizes          | States                          | API Summary                          |
|----------------|-----------------------------------------------|----------------|----------------------------------|--------------------------------------|
| **OxButton**   | Primary, Secondary, Ghost, Danger             | Small, Medium, Large | default, hover, pressed, focused, disabled | `OxButton::new(cx, "Label")`         |
| **OxIconButton** | Primary, Secondary, Ghost, Danger           | Small, Medium, Large | default, hover, pressed, focused, disabled | `OxIconButton::new(cx, icon_id)`     |
| **OxButtonGroup** | Horizontal, Vertical                        | inherits       | —                                | `OxButtonGroup::new(cx).add(...)`    |
| **OxToggleButton** | Single, Group                               | Small, Medium, Large | default, selected, hover, pressed | `OxToggleButton::new(cx, selected)`  |

**States:** `default` | `hover` | `pressed` | `focused` | `disabled`

---

### Inputs

| Component      | Variants | States                          | API Summary                          |
|----------------|----------|----------------------------------|--------------------------------------|
| **OxTextInput** | Single-line, with label, with hint | default, hover, focused, disabled, error | `OxTextInput::new(cx).placeholder("...")` |
| **OxTextArea** | Resizable, fixed height           | default, hover, focused, disabled, error | `OxTextArea::new(cx).rows(4)`        |
| **OxCheckbox** | Checked, Unchecked, Indeterminate | default, hover, pressed, focused, disabled | `OxCheckbox::new(cx, checked)`       |
| **OxRadio**    | Selected, Unselected             | default, hover, pressed, focused, disabled | `OxRadio::new(cx, selected, group)` |
| **OxSwitch**   | On, Off                           | default, hover, pressed, focused, disabled | `OxSwitch::new(cx, on)`              |
| **OxSlider**   | Single, Range (future)            | default, hover, pressed, focused, disabled | `OxSlider::new(cx, value, min, max)`|

---

### Display

| Component   | Variants                                      | Sizes / Shapes       | States        | API Summary                          |
|-------------|-----------------------------------------------|----------------------|---------------|--------------------------------------|
| **OxLabel** | Title, Subtitle, Body, Caption                | —                    | —             | `OxLabelTitle::new(cx, "Text")`      |
| **OxBadge**  | Default, Success, Warning, Error, Info        | Small, Medium        | —             | `OxBadge::new(cx, "Label", BadgeVariant::Success)` |
| **OxAvatar** | Image, Initials, Icon                         | Small (24), Medium (32), Large (48) | — | `OxAvatar::new(cx).src(url)` or `.initials("AB")` |
| **OxIcon**   | —                                            | Small, Medium, Large | default, muted | `OxIcon::new(cx, icon_id)`           |

**OxLabel variants:** `OxLabelTitle`, `OxLabelSubtitle`, `OxLabelBody`, `OxLabelCaption`

**OxAvatar shapes:** Circle (default), Square, Rounded

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
use oxide_widgets::prelude::*;
// Re-exports: OxButton, OxLabelTitle, OxLabelBody, OxBadge, OxAvatar, OxIcon,
//             OxTextInput, OxCheckbox, OxRadio, OxSwitch, OxSlider,
//             ThemeEngine, use_theme
```

### Theme Access Patterns

```rust
// Global theme (from AppState or Scope)
let theme = cx.use_theme();
let bg = theme.semantic_colors.background.primary;

// Per-component override (future)
OxButton::new(cx, "Click")
    .theme_override(|t| t.accent.primary = custom_color);
```

### Component DSL Patterns

```rust
// Makepad live_design! style
OxButton::new(cx, "Submit")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Medium)
    .disabled(false);

OxLabelTitle::new(cx, "Welcome")
    .align(Align::Center);

OxTextInput::new(cx)
    .placeholder("Enter email")
    .label("Email");
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
