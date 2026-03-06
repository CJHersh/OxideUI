# Button Component

OxideUI buttons are defined with `script_mod!` and themed at runtime via `apply_*_theme` functions that patch GPU shader uniforms.

## Import

```rust
use makepad_widgets::*;
use oxide_widgets::buttons;
```

## DSL Definition

Buttons are defined inside a `script_mod!` block. Use the `:=` syntax to give them a referenceable name:

```rust
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    View{
                        flow: Down, spacing: 12., padding: 16.
                        submit := OxButton{ text: "Submit" }
                        cancel := OxButtonSecondary{ text: "Cancel" }
                        skip := OxButtonGhost{ text: "Skip" }
                        delete := OxButtonDanger{ text: "Delete" }
                    }
                }
            }
        }
    }
    app
}
```

## Variants

| Variant | Widget | Accent | Use Case |
|---------|--------|--------|----------|
| **Primary** | `OxButton` | Theme accent | Main actions |
| **Secondary** | `OxButtonSecondary` | Outlined/neutral | Secondary actions |
| **Ghost** | `OxButtonGhost` | Transparent | Tertiary, low emphasis |
| **Danger** | `OxButtonDanger` | Red | Destructive actions |
| **Small** | `OxButtonSmall` | Same as primary | Compact variant |
| **Large** | `OxButtonLarge` | Same as primary | Prominent variant |

## Sizes

Size variants are separate widgets with adjusted padding, font size, and border radius:

```rust
small := OxButtonSmall{ text: "Small" }
normal := OxButton{ text: "Normal" }
large := OxButtonLarge{ text: "Large" }
```

## Handling Clicks

```rust
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(submit)).clicked(actions) {
            // Handle click
            self.ui.redraw(cx);
        }
    }
}
```

## States

| State | Uniform | Description |
|-------|---------|-------------|
| **default** | `color` | Rest state |
| **hover** | `color_hover` | Mouse over |
| **pressed** | `color_down` | Mouse down |

Makepad's `Button` widget interpolates between these uniforms automatically using its built-in animator.

## Theme Integration

Apply the current theme at runtime with `apply_*_theme` functions. Each function patches the button's GPU shader uniforms (background color, hover/pressed states, border radius) to match the active theme.

### Per-Widget

```rust
use oxide_core::theme::engine::ThemeEngine;
use oxide_widgets::buttons;

let theme = ThemeEngine::current();
buttons::apply_button_theme(cx, &self.ui.button(cx, ids!(submit)), &theme);
buttons::apply_button_secondary_theme(cx, &self.ui.button(cx, ids!(cancel)), &theme);
self.ui.redraw(cx);
```

### Batch with ThemeMap

```rust
use oxide_widgets::ThemeMap;
use oxide_widgets::buttons;

ThemeMap::builder(&self.ui)
    .add(live_id!(submit), buttons::apply_button_theme)
    .add(live_id!(cancel), buttons::apply_button_secondary_theme)
    .add(live_id!(skip), buttons::apply_button_ghost_theme)
    .add(live_id!(delete), buttons::apply_button_danger_theme)
    .build()
    .apply_all(cx, &ThemeEngine::current());
self.ui.redraw(cx);
```

## Apply Functions Reference

| Function | Widget | Semantic Tokens Used |
|----------|--------|---------------------|
| `apply_button_theme` | OxButton | `interactive_default/hover/pressed`, `radius.md` |
| `apply_button_secondary_theme` | OxButtonSecondary | `border_default`, `surface_primary/secondary/tertiary`, `radius.md` |
| `apply_button_ghost_theme` | OxButtonGhost | `interactive_default` (transparent bg), `radius.md` |
| `apply_button_danger_theme` | OxButtonDanger | `feedback_error`, `radius.md` |
| `apply_button_small_theme` | OxButtonSmall | Same as primary + `radius.sm` |
| `apply_button_large_theme` | OxButtonLarge | Same as primary + `radius.lg` |

## DSL Customization

Override any property inline in the DSL:

```rust
custom := OxButton{
    text: "Custom"
    width: 200, height: 48
    draw_bg.color: #804DBF
    draw_bg.color_hover: #6B3FA3
    draw_bg.color_down: #5A3589
    draw_bg.border_radius: 12.0
    draw_text.color: #FFFFFF
}
```

## Props Reference

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `text` | string | `"Button"` | Button label |
| `width` | Size | Fit | Button width |
| `height` | Size | Fit | Button height |
| `padding` | Inset | varies by size | Inner padding |
| `draw_bg.color` | color | `#10A37F` | Background color (default state) |
| `draw_bg.color_hover` | color | `#0D8A6A` | Background color (hover state) |
| `draw_bg.color_down` | color | `#0C7459` | Background color (pressed state) |
| `draw_bg.border_radius` | f32 | `8.0` | Corner radius |
| `draw_text.color` | color | `#FFFFFF` | Text color |
