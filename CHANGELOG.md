# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-03-05

### Added

- **Dark mode support** -- Every built-in theme (OpenAI, Airbnb, Notion) now ships with a carefully crafted dark variant (6 themes total)
- **ThemeMode enum** -- `ThemeMode::Light` and `ThemeMode::Dark` on each theme for easy mode detection
- **Expanded token system** -- New token categories added to `Theme`:
  - `ElevationScale` -- 5-level layered depth scale mapping UI depth to shadow presets
  - `MotionScale` -- Animation durations (fast, normal, slow) and easing factor
  - `OpacityScale` -- Disabled, hover overlay, pressed overlay, and backdrop opacity tokens
  - `FocusRingTokens` -- Color, width, and offset for keyboard navigation indicators
- **SDF icon system** -- `OxIcon` with 20 built-in GPU-rendered icons (check, close, chevrons, plus, minus, search, menu, alert, info, arrows, eye, copy, settings, star, heart, external-link) via `icon_type` uniform
- **Icon theme functions** -- `apply_icon_theme`, `apply_icon_accent_theme`, `apply_icon_secondary_theme`
- **Helper functions** -- `light_themes()` and `dark_themes()` for filtering theme lists
- **Light/dark toggle** -- Showcase and theme-switcher examples now support toggling between light and dark modes
- **CI improvements** -- Multi-OS test matrix (Ubuntu, macOS, Windows), separate fmt/clippy/test/build/docs jobs
- **New tests** -- 14 new tests for dark themes, new tokens, color range validation, focus ring, elevation, motion, opacity

### Changed

- Widget DSL defaults now use OpenAI light theme colors (teal `#10A37F`) instead of neutral grays
- `OxButton` variants have improved padding, sizing, and consistent font sizes
- `OxCard` now has border support (`border_size: 1.0`, `border_color`) for cleaner separation
- `OxDivider` uses semantic `border_default` color
- `OxButtonGhost` hover/pressed now uses `opacity.hover_overlay` and `opacity.pressed_overlay` tokens
- All skeleton/progress components use semantic surface colors
- `all_themes()` now returns 6 themes (was 3)
- Figma `tokens.json` expanded with all 6 themes and new token categories (mode, motion, opacity, focus_ring)
- Figma `tokens.schema.json` updated with new optional sections (mode, motion, opacity, focus_ring)
- Codegen (`rust.rs`) updated with `emit_motion`, `emit_opacity`, `emit_focus_ring` functions with sensible defaults

### Fixed

- Flaky `theme_engine_out_of_bounds_clamps` test due to shared global state in parallel test execution

## [0.1.0] - 2026-03-01

### Added

- Initial release with 3 built-in themes (OpenAI, Airbnb, Notion)
- 50+ UI components across 8 categories
- ThemeEngine with atomic runtime theme switching
- ThemeMap for batch theme application
- Figma Variables integration via oxide-cli
- JSON design tokens pipeline (tokens.json -> Rust structs)
