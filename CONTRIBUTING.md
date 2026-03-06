# Contributing to OxideUI

Thank you for your interest in contributing to OxideUI! We welcome contributions of all kinds.

## Getting Started

1. Fork the repository and clone your fork
2. Install Rust stable via [rustup](https://rustup.rs/)
3. Run `cargo check --workspace` to verify the build
4. Create a branch for your changes

## Development Workflow

```bash
# Check compilation
cargo check --workspace

# Run tests
cargo test --workspace

# Run lints
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt --all

# Run the showcase example
cargo run -p oxide-showcase
```

## Code Style

- **Formatting**: Run `cargo fmt` before every commit
- **Linting**: All code must pass `cargo clippy -- -D warnings`
- **Naming**: `Ox` prefix for all public widgets; `snake_case` for functions/fields
- **Documentation**: All public items must have `///` doc comments

## Commit Messages

Use the conventional commit format:

```
type(scope): description
```

- **Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- **Scope**: crate or component name (e.g. `button`, `core`, `cli`)
- **Examples**:
  - `feat(button): add Danger variant`
  - `fix(theme): correct hex parsing for 8-digit colors`
  - `docs(readme): update quick start instructions`

## Pull Request Process

1. Open an issue or comment on an existing one
2. Fork and branch from `main`
3. Implement your changes and add/update tests
4. Run `cargo test --workspace` and `cargo clippy --workspace`
5. Submit a PR with a description; link the issue
6. Address review feedback

## Component Checklist

When adding or modifying a component, ensure:

- [ ] All states implemented (default, hover, pressed, focused, disabled)
- [ ] Uses semantic tokens from theme (no hardcoded colors)
- [ ] Supports size variants where applicable
- [ ] Has `script_mod!` DSL integration
- [ ] Has a matching `apply_*_theme` function
- [ ] Documented with `///` doc comments
- [ ] Showcase example updated

## Project Plan

For a full overview of the architecture, roadmap, and component specifications,
see [PROJECT_PLAN.md](PROJECT_PLAN.md).

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
Please read it before participating.

## License

By contributing to OxideUI, you agree that your contributions will be licensed
under both the MIT and Apache 2.0 licenses.
