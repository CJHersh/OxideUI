use crate::theme::themes::*;
use crate::theme::*;
use makepad_widgets::makepad_draw::*;

// ── hex_to_vec4 ──────────────────────────────────────────────────

#[test]
fn hex_to_vec4_six_digit() {
    let c = hex_to_vec4("#10A37F");
    assert!((c.x - 0x10 as f32 / 255.0).abs() < 1e-5);
    assert!((c.y - 0xA3 as f32 / 255.0).abs() < 1e-5);
    assert!((c.z - 0x7F as f32 / 255.0).abs() < 1e-5);
    assert!((c.w - 1.0).abs() < 1e-5);
}

#[test]
fn hex_to_vec4_eight_digit() {
    let c = hex_to_vec4("#10A37F80");
    assert!((c.x - 0x10 as f32 / 255.0).abs() < 1e-5);
    assert!((c.y - 0xA3 as f32 / 255.0).abs() < 1e-5);
    assert!((c.z - 0x7F as f32 / 255.0).abs() < 1e-5);
    assert!((c.w - 0x80 as f32 / 255.0).abs() < 1e-5);
}

#[test]
fn hex_to_vec4_without_hash() {
    let c = hex_to_vec4("FF0000");
    assert!((c.x - 1.0).abs() < 1e-5);
    assert!(c.y.abs() < 1e-5);
    assert!(c.z.abs() < 1e-5);
    assert!((c.w - 1.0).abs() < 1e-5);
}

#[test]
fn hex_to_vec4_white() {
    let c = hex_to_vec4("#FFFFFF");
    assert!((c.x - 1.0).abs() < 1e-5);
    assert!((c.y - 1.0).abs() < 1e-5);
    assert!((c.z - 1.0).abs() < 1e-5);
    assert!((c.w - 1.0).abs() < 1e-5);
}

#[test]
fn hex_to_vec4_black() {
    let c = hex_to_vec4("#000000");
    assert!(c.x.abs() < 1e-5);
    assert!(c.y.abs() < 1e-5);
    assert!(c.z.abs() < 1e-5);
    assert!((c.w - 1.0).abs() < 1e-5);
}

#[test]
fn hex_to_vec4_invalid_returns_black() {
    let c = hex_to_vec4("xyz");
    assert!(c.x.abs() < 1e-5);
    assert!(c.y.abs() < 1e-5);
    assert!(c.z.abs() < 1e-5);
    assert!((c.w - 1.0).abs() < 1e-5);
}

#[test]
fn hex_to_vec4_empty_returns_black() {
    let c = hex_to_vec4("");
    assert!(c.x.abs() < 1e-5);
    assert!(c.y.abs() < 1e-5);
    assert!(c.z.abs() < 1e-5);
    assert!((c.w - 1.0).abs() < 1e-5);
}

#[test]
fn hex_to_vec4_full_alpha() {
    let c = hex_to_vec4("#FF0000FF");
    assert!((c.x - 1.0).abs() < 1e-5);
    assert!(c.y.abs() < 1e-5);
    assert!(c.z.abs() < 1e-5);
    assert!((c.w - 1.0).abs() < 1e-5);
}

#[test]
fn hex_to_vec4_zero_alpha() {
    let c = hex_to_vec4("#FF000000");
    assert!((c.x - 1.0).abs() < 1e-5);
    assert!(c.y.abs() < 1e-5);
    assert!(c.z.abs() < 1e-5);
    assert!(c.w.abs() < 1e-5);
}

// ── ThemeEngine ──────────────────────────────────────────────────

#[test]
fn theme_engine_init_and_current() {
    ThemeEngine::init(all_themes());
    let theme = ThemeEngine::current();
    assert_eq!(theme.name, "shadcn");
}

#[test]
fn theme_engine_switch_by_index() {
    ThemeEngine::init(all_themes());
    ThemeEngine::switch(1);
    let theme = ThemeEngine::current();
    assert_eq!(theme.name, "shadcn Dark");
    ThemeEngine::switch(0);
}

#[test]
fn theme_engine_switch_by_name() {
    ThemeEngine::init(all_themes());
    assert!(ThemeEngine::switch_by_name("shadcn Dark"));
    assert_eq!(ThemeEngine::current().name, "shadcn Dark");

    assert!(!ThemeEngine::switch_by_name("NonExistent"));
    assert_eq!(ThemeEngine::current().name, "shadcn Dark");

    ThemeEngine::switch(0);
}

#[test]
fn theme_engine_switch_by_name_dark() {
    ThemeEngine::init(all_themes());
    assert!(ThemeEngine::switch_by_name("shadcn Dark"));
    assert_eq!(ThemeEngine::current().name, "shadcn Dark");
    assert_eq!(ThemeEngine::current().mode, ThemeMode::Dark);
    ThemeEngine::switch(0);
}

#[test]
fn theme_engine_available_themes() {
    ThemeEngine::init(all_themes());
    let names = ThemeEngine::available_themes();
    assert!(names.contains(&"shadcn"));
    assert!(names.contains(&"shadcn Dark"));
    assert_eq!(names.len(), 2);
}

#[test]
fn theme_engine_count() {
    ThemeEngine::init(all_themes());
    assert_eq!(ThemeEngine::theme_count(), 2);
}

#[test]
fn theme_engine_next_wraps() {
    ThemeEngine::init(all_themes());
    let count = ThemeEngine::theme_count();
    ThemeEngine::switch(count - 1);
    ThemeEngine::next_theme();
    assert_eq!(ThemeEngine::current_index(), 0);
    ThemeEngine::switch(0);
}

#[test]
fn theme_engine_prev_wraps() {
    ThemeEngine::init(all_themes());
    ThemeEngine::switch(0);
    ThemeEngine::prev_theme();
    assert_eq!(ThemeEngine::current_index(), ThemeEngine::theme_count() - 1);
    ThemeEngine::switch(0);
}

#[test]
fn theme_engine_out_of_bounds_clamps() {
    ThemeEngine::init(all_themes());
    ThemeEngine::switch(999);
    let idx = ThemeEngine::current_index();
    let count = ThemeEngine::theme_count();
    assert!(idx < count, "index {} should be < count {}", idx, count);
    ThemeEngine::switch(0);
}

// ── Token defaults ───────────────────────────────────────────────

#[test]
fn spacing_scale_default_values() {
    let s = SpacingScale::default();
    assert_eq!(s.none, 0.0);
    assert_eq!(s.xs, 8.0);
    assert_eq!(s.sm, 12.0);
    assert_eq!(s.md, 16.0);
    assert_eq!(s.lg, 20.0);
    assert_eq!(s.xl, 24.0);
    assert_eq!(s.xxl, 32.0);
}

#[test]
fn radius_scale_default_values() {
    let r = RadiusScale::default();
    assert_eq!(r.none, 0.0);
    assert_eq!(r.xs, 2.0);
    assert_eq!(r.sm, 4.0);
    assert_eq!(r.md, 6.0);
    assert_eq!(r.lg, 8.0);
    assert_eq!(r.xl, 12.0);
    assert_eq!(r.full, 9999.0);
}

#[test]
fn typography_scale_default_values() {
    let t = TypographyScale::default();
    assert_eq!(t.font_family, "Inter");
    assert_eq!(t.font_size_xs, 12.0);
    assert_eq!(t.font_size_sm, 14.0);
    assert_eq!(t.font_size_md, 16.0);
    assert_eq!(t.font_size_lg, 18.0);
    assert_eq!(t.font_size_xl, 20.0);
    assert_eq!(t.font_size_xxl, 24.0);
}

#[test]
fn shadow_none_is_zero() {
    let s = Shadow::none();
    assert_eq!(s.offset_x, 0.0);
    assert_eq!(s.offset_y, 0.0);
    assert_eq!(s.blur, 0.0);
    assert_eq!(s.spread, 0.0);
}

#[test]
fn shadow_new_stores_values() {
    let s = Shadow::new(1.0, 2.0, 3.0, 4.0, vec4(0.5, 0.5, 0.5, 1.0));
    assert_eq!(s.offset_x, 1.0);
    assert_eq!(s.offset_y, 2.0);
    assert_eq!(s.blur, 3.0);
    assert_eq!(s.spread, 4.0);
    assert!((s.color.x - 0.5).abs() < 1e-5);
}

#[test]
fn elevation_scale_defaults() {
    let e = ElevationScale::default();
    assert_eq!(e.level0.blur, 0.0);
    assert!(e.level1.blur > 0.0);
    assert!(e.level2.blur > e.level1.blur);
    assert!(e.level3.blur > e.level2.blur);
    assert!(e.level4.blur > e.level3.blur);
}

#[test]
fn motion_scale_defaults() {
    let m = MotionScale::default();
    assert_eq!(m.duration_fast, 0.1);
    assert_eq!(m.duration_normal, 0.2);
    assert_eq!(m.duration_slow, 0.35);
    assert!(m.ease > 0.0);
}

#[test]
fn opacity_scale_defaults() {
    let o = OpacityScale::default();
    assert!(o.disabled > 0.0 && o.disabled < 1.0);
    assert!(o.hover_overlay > 0.0 && o.hover_overlay < 1.0);
    assert!(o.pressed_overlay > o.hover_overlay);
    assert!(o.backdrop > 0.0 && o.backdrop < 1.0);
}

#[test]
fn focus_ring_defaults() {
    let f = FocusRingTokens::default();
    assert!(f.width > 0.0);
    assert!(f.offset >= 0.0);
    assert!(f.color.w > 0.0);
}

#[test]
fn theme_mode_default_is_light() {
    assert_eq!(ThemeMode::default(), ThemeMode::Light);
}

// ── shadcn theme definitions ─────────────────────────────────────

#[test]
fn shadcn_theme_has_correct_primary() {
    let t = shadcn_theme();
    assert_eq!(t.name, "shadcn");
    assert_eq!(t.mode, ThemeMode::Light);
    let expected = hex_to_vec4("#171717");
    assert!((t.colors.interactive_default.x - expected.x).abs() < 1e-5);
    assert!((t.colors.interactive_default.y - expected.y).abs() < 1e-5);
    assert!((t.colors.interactive_default.z - expected.z).abs() < 1e-5);
}

#[test]
fn shadcn_dark_theme_has_correct_primary() {
    let t = shadcn_dark_theme();
    assert_eq!(t.name, "shadcn Dark");
    assert_eq!(t.mode, ThemeMode::Dark);
    let expected = hex_to_vec4("#F5F5F5");
    assert!((t.colors.interactive_default.x - expected.x).abs() < 1e-5);
    assert!((t.colors.interactive_default.y - expected.y).abs() < 1e-5);
    assert!((t.colors.interactive_default.z - expected.z).abs() < 1e-5);
}

#[test]
fn all_themes_returns_two() {
    let themes = all_themes();
    assert_eq!(themes.len(), 2);
    assert_eq!(themes[0].name, "shadcn");
    assert_eq!(themes[1].name, "shadcn Dark");
}

#[test]
fn light_themes_are_light() {
    for theme in light_themes() {
        assert_eq!(
            theme.mode,
            ThemeMode::Light,
            "{} should be Light",
            theme.name
        );
    }
}

#[test]
fn dark_themes_are_dark() {
    for theme in dark_themes() {
        assert_eq!(theme.mode, ThemeMode::Dark, "{} should be Dark", theme.name);
    }
}

#[test]
fn dark_surfaces_are_darker_than_light() {
    fn luminance(c: Vec4) -> f32 {
        0.299 * c.x + 0.587 * c.y + 0.114 * c.z
    }
    let light = shadcn_theme();
    let dark = shadcn_dark_theme();
    assert!(luminance(light.colors.surface_primary) > luminance(dark.colors.surface_primary));
}

#[test]
fn dark_text_is_lighter_than_light_text() {
    fn luminance(c: Vec4) -> f32 {
        0.299 * c.x + 0.587 * c.y + 0.114 * c.z
    }
    let light = shadcn_theme();
    let dark = shadcn_dark_theme();
    assert!(luminance(dark.colors.text_primary) > luminance(light.colors.text_primary));
}

#[test]
fn dark_shadows_are_stronger_than_light() {
    let light = shadcn_theme();
    let dark = shadcn_dark_theme();
    assert!(dark.shadows.sm.color.w > light.shadows.sm.color.w);
    assert!(dark.shadows.md.color.w > light.shadows.md.color.w);
}

#[test]
fn themes_typography_has_font_family() {
    for theme in all_themes() {
        assert!(!theme.typography.font_family.is_empty());
    }
}

#[test]
fn all_themes_have_valid_focus_ring() {
    for theme in all_themes() {
        assert!(
            theme.focus_ring.width > 0.0,
            "{} focus_ring.width",
            theme.name
        );
        assert!(
            theme.focus_ring.color.w > 0.0,
            "{} focus_ring.color alpha",
            theme.name
        );
    }
}

#[test]
fn all_themes_colors_in_range() {
    fn check_color(c: Vec4, label: &str) {
        assert!(c.x >= 0.0 && c.x <= 1.0, "{} r={}", label, c.x);
        assert!(c.y >= 0.0 && c.y <= 1.0, "{} g={}", label, c.y);
        assert!(c.z >= 0.0 && c.z <= 1.0, "{} b={}", label, c.z);
        assert!(c.w >= 0.0 && c.w <= 1.0, "{} a={}", label, c.w);
    }
    for theme in all_themes() {
        let c = &theme.colors;
        check_color(
            c.surface_primary,
            &format!("{}.surface_primary", theme.name),
        );
        check_color(c.text_primary, &format!("{}.text_primary", theme.name));
        check_color(
            c.interactive_default,
            &format!("{}.interactive_default", theme.name),
        );
        check_color(c.border_default, &format!("{}.border_default", theme.name));
        check_color(
            c.feedback_success,
            &format!("{}.feedback_success", theme.name),
        );
        check_color(c.feedback_error, &format!("{}.feedback_error", theme.name));
    }
}

#[test]
fn all_themes_have_radius_xs() {
    for theme in all_themes() {
        assert!(
            theme.radius.xs >= 0.0,
            "{} radius.xs should be non-negative",
            theme.name
        );
        assert!(
            theme.radius.xs <= theme.radius.sm,
            "{} radius.xs <= sm",
            theme.name
        );
    }
}

// ── Brand themes still compile (not in all_themes but available) ─

#[test]
fn openai_theme_compiles() {
    let t = openai_theme();
    assert_eq!(t.name, "OpenAI");
    assert_eq!(t.mode, ThemeMode::Light);
}

#[test]
fn airbnb_theme_compiles() {
    let t = airbnb_theme();
    assert_eq!(t.name, "Airbnb");
    assert_eq!(t.mode, ThemeMode::Light);
}

#[test]
fn notion_theme_compiles() {
    let t = notion_theme();
    assert_eq!(t.name, "Notion");
    assert_eq!(t.mode, ThemeMode::Light);
}

#[test]
fn brand_dark_themes_compile() {
    assert_eq!(openai_dark_theme().mode, ThemeMode::Dark);
    assert_eq!(airbnb_dark_theme().mode, ThemeMode::Dark);
    assert_eq!(notion_dark_theme().mode, ThemeMode::Dark);
}
