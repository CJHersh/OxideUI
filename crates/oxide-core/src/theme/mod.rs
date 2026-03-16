// Token architecture note:
//
// Widget DSL defaults (hex colors in script_mod! definitions) are intentionally
// duplicated from the shadcn light theme values defined in `themes::shadcn`.
// This dual-source design exists because:
//
// 1. DSL defaults provide zero-config rendering -- widgets look correct without
//    any ThemeEngine setup.
// 2. Theme structs enable runtime switching -- `apply_*_theme` functions override
//    DSL defaults by setting shader uniforms from Theme tokens.
//
// If you change token values in `themes/shadcn.rs`, you must also update the
// corresponding hex values in `oxide-widgets` DSL definitions. The test
// `token_drift_detection` in `tests.rs` guards against accidental divergence.
//
// Long-term: Build-time codegen from `figma/tokens.json` will eliminate this
// duplication (tracked as a roadmap item).

pub mod engine;
#[cfg(test)]
mod tests;
pub mod themes;
pub mod tokens;

pub use engine::*;
pub use tokens::*;
