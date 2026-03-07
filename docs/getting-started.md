# Getting Started with OxideUI

OxideUI is the industry's first Multi-Skin Rust component library. Build once, then toggle between pre-built themes with a single line of code.

## Prerequisites

- **Rust 1.70+** — install via [rustup](https://rustup.rs/)
- **Makepad** — OxideUI is built on [Makepad](https://github.com/makepad/makepad) (dev branch)
- **Platform deps (Linux)** — `sudo apt install libx11-dev libxcursor-dev libgl-dev libasound2-dev` (or your distro's equivalents). macOS and Windows need no extra system packages.

## Try it now

The fastest way to see OxideUI in action is to clone the repo and run the showcase:

```bash
git clone https://github.com/CJHersh/OxideUI.git
cd OxideUI
cargo run -p oxide-showcase
```

## Installation

### Option 1: CLI (recommended)

```bash
git clone https://github.com/CJHersh/OxideUI.git
cd OxideUI
cargo install --path crates/oxide-cli
oxide new my-app
cd my-app
cargo run
```

### Option 2: Add to existing project

Add to your `Cargo.toml`:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
oxide-widgets = { git = "https://github.com/CJHersh/OxideUI", branch = "main" }
oxide-core = { git = "https://github.com/CJHersh/OxideUI", branch = "main" }
makepad-widgets = { git = "https://github.com/makepad/makepad", rev = "8b515338a2f50c5e0e2742cdc8b8ee7278aff371" }
```

## First App

Create `src/main.rs`, `src/lib.rs`, and `src/app.rs`:

**src/lib.rs:**
```rust
pub use makepad_widgets;
pub use oxide_widgets;
pub mod app;
```

**src/app.rs:**
```rust
use makepad_widgets::*;
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::themes::all_themes;
use oxide_widgets::{buttons, display, layout};
use oxide_widgets::ThemeMap;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "My OxideUI App"
                body +: {
                    View{
                        width: Fill height: Fill
                        flow: Down
                        padding: 16.
                        spacing: 16.
                        title := OxLabelTitle{ text: "Welcome to OxideUI" }
                        card := OxCard{
                            get_started := OxButton{ text: "Get Started" }
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

    fn switch_theme(&self, cx: &mut Cx, name: &str) {
        ThemeEngine::switch_by_name(name);
        let theme = ThemeEngine::current();
        ThemeMap::builder(&self.ui)
            .add(live_id!(title), display::apply_label_title_theme)
            .add(live_id!(card), layout::apply_card_theme)
            .add(live_id!(get_started), buttons::apply_button_theme)
            .build()
            .apply_all(cx, &theme);
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

**src/main.rs:**
```rust
fn main() {
    my_app::app::app_main()
}
```

## Running on Platforms

| Platform | Command |
|----------|---------|
| **Desktop** | `cargo run` |
| **Web** | `cargo makepad wasm run -p my-app` |
| **Android** | `cargo makepad android run -p my-app` |
| **iOS** | `cargo makepad ios run -p my-app` |

## Switching Themes

Wire a click on the `get_started` button (defined in the First App above) to cycle themes:

```rust
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(get_started)).clicked(actions) {
            ThemeEngine::next_theme();
            self.switch_theme(cx, ThemeEngine::current().name);
        }
    }
}
```

## Available Components

| Category | Components |
|----------|------------|
| **Buttons** | OxButton, OxButtonSecondary, OxButtonGhost, OxButtonDanger, OxIconButton, OxToggleButton |
| **Inputs** | OxTextInput, OxTextArea, OxCheckbox, OxRadio, OxSwitch, OxSlider |
| **Display** | OxLabel (Title, Subtitle, Body, Caption), OxBadge, OxAvatar, OxIcon |
| **Layout** | OxCard, OxDivider, OxStack, OxGrid |
| **Feedback** | OxAlert, OxProgress, OxSkeleton |
| **Navigation** | OxTabs, OxBreadcrumb, OxPagination |
| **Overlay** | OxTooltip, OxPopover, OxDrawer, OxMenu |
| **Data** | OxTable, OxList, OxTimeline |

## Next Steps

- [Theming Guide](theming.md) — Pre-built themes, custom themes
- [Figma Workflow](figma-workflow.md) — Sync design tokens from Figma
- [Component Reference](components/button.md) — Detailed component docs
