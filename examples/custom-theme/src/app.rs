use makepad_widgets::*;
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::{
    hex_to_vec4, ElevationScale, FocusRingTokens, MotionScale, OpacityScale, RadiusScale,
    SemanticColors, ShadowScale, SpacingScale, Theme, ThemeMode, TypographyScale,
};
use oxide_widgets::theme_apply::ThemeMap;
use oxide_widgets::{buttons, display, layout};

fn h(hex: &str) -> makepad_widgets::makepad_draw::Vec4 {
    hex_to_vec4(hex)
}

fn purple_theme() -> Theme {
    Theme {
        name: "Purple",
        mode: ThemeMode::Light,
        colors: SemanticColors {
            surface_primary: h("#FFFFFF"),
            surface_secondary: h("#F5F0FA"),
            surface_tertiary: h("#D8CDE2"),
            surface_inverse: h("#2D1A4C"),
            text_primary: h("#2D1A4C"),
            text_secondary: h("#5C4D6D"),
            text_tertiary: h("#8A7A9A"),
            text_disabled: h("#B5A8C2"),
            text_inverse: h("#FFFFFF"),
            text_link: h("#804DBF"),
            interactive_default: h("#804DBF"),
            interactive_hover: h("#6B3FA3"),
            interactive_pressed: h("#5A3589"),
            interactive_disabled: h("#D3C9D8"),
            border_default: h("#D8CDE2"),
            border_hover: h("#D9CAD8"),
            border_focus: h("#804DBF"),
            border_error: h("#D53935"),
            feedback_success: h("#10A37F"),
            feedback_warning: h("#F59A10"),
            feedback_error: h("#D53935"),
            feedback_info: h("#804DBF"),
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale::default(),
        typography: TypographyScale {
            font_family: "Inter",
            ..Default::default()
        },
        shadows: ShadowScale::default(),
        elevation: ElevationScale::default(),
        motion: MotionScale::default(),
        opacity: OpacityScale::default(),
        focus_ring: FocusRingTokens {
            color: h("#804DBF"),
            width: 2.0,
            offset: 2.0,
        },
    }
}

fn all_themes_with_purple() -> Vec<Theme> {
    let mut themes = oxide_core::theme::themes::all_themes();
    themes.push(purple_theme());
    themes
}

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "OxideUI Custom Theme"
                window.inner_size: vec2(800, 600)
                body +: {
                    View{
                        width: Fill height: Fill
                        flow: Down
                        padding: 32
                        spacing: 24
                        align.x: 0.5

                        title_label := OxLabelTitle{ text: "Custom Theme Demo" }
                        subtitle_label := OxLabelSecondary{ text: "Demonstrates creating a custom purple theme" }

                        View{
                            flow: Right spacing: 12
                            btn_openai := OxButton{ text: "OpenAI" }
                            btn_airbnb := OxButton{ text: "Airbnb" }
                            btn_notion := OxButton{ text: "Notion" }
                            btn_purple := OxButton{ text: "Purple (Custom)" }
                        }

                        sample_card := OxCard{
                            width: 400
                            card_subtitle := OxLabelSubtitle{ text: "Custom Theme Card" }
                            card_body := OxLabelBody{ text: "Switch to Purple to see the custom theme." }
                            View{
                                flow: Right spacing: 8
                                action_btn := OxButton{ text: "Action" }
                                cancel_btn := OxButtonSecondary{ text: "Cancel" }
                            }
                        }

                        current_theme := OxLabelCaption{ text: "Active: OpenAI" }
                    }
                }
            }
        }
    }
    app
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

        let map = ThemeMap::builder(&self.ui)
            .add(live_id!(title_label), display::apply_label_title_theme)
            .add(
                live_id!(subtitle_label),
                display::apply_label_secondary_theme,
            )
            .add(live_id!(btn_openai), buttons::apply_button_theme)
            .add(live_id!(btn_airbnb), buttons::apply_button_theme)
            .add(live_id!(btn_notion), buttons::apply_button_theme)
            .add(live_id!(btn_purple), buttons::apply_button_theme)
            .add(live_id!(sample_card), layout::apply_card_theme)
            .add(live_id!(card_subtitle), display::apply_label_subtitle_theme)
            .add(live_id!(card_body), display::apply_label_body_theme)
            .add(live_id!(action_btn), buttons::apply_button_theme)
            .add(live_id!(cancel_btn), buttons::apply_button_secondary_theme)
            .add(live_id!(current_theme), display::apply_label_caption_theme)
            .build();

        map.apply_all(cx, &theme);

        self.ui
            .label(cx, ids!(current_theme))
            .set_text(cx, &format!("Active: {}", name));
        self.ui.redraw(cx);
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        ThemeEngine::init(all_themes_with_purple());
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(btn_openai)).clicked(actions) {
            self.switch_theme(cx, "OpenAI");
        }
        if self.ui.button(cx, ids!(btn_airbnb)).clicked(actions) {
            self.switch_theme(cx, "Airbnb");
        }
        if self.ui.button(cx, ids!(btn_notion)).clicked(actions) {
            self.switch_theme(cx, "Notion");
        }
        if self.ui.button(cx, ids!(btn_purple)).clicked(actions) {
            self.switch_theme(cx, "Purple");
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
