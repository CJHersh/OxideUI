use makepad_widgets::*;
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::themes::all_themes;
use oxide_widgets::theme_apply::ThemeMap;
use oxide_widgets::{buttons, display, inputs, layout};

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "OxideUI Theme Switcher"
                window.inner_size: vec2(800, 600)
                body +: {
                    content_bg := View{
                        width: Fill height: Fill
                        flow: Down
                        padding: 48
                        spacing: 24
                        align.x: 0.5
                        show_bg: true
                        draw_bg.color: #F4F4F5FF

                        title_label := OxLabelTitle{ text: "Theme Switcher" }
                        subtitle_label := OxLabelSecondary{ text: "Click a theme to switch instantly" }

                        View{
                            flow: Right spacing: 12 align.y: 0.5
                            btn_openai := OxButton{ text: "OpenAI" }
                            btn_airbnb := OxButton{
                                text: "Airbnb"
                                draw_bg +: {
                                    color: uniform(#FF5A5F)
                                    color_hover: uniform(#E04E52)
                                    color_down: uniform(#C84347)
                                }
                            }
                            btn_notion := OxButton{
                                text: "Notion"
                                draw_bg +: {
                                    color: uniform(#346CA3)
                                    color_hover: uniform(#2D5F90)
                                    color_down: uniform(#26527D)
                                }
                            }
                            btn_toggle_dark := OxButtonGhost{ text: "Toggle Dark" }
                        }

                        sample_card := OxCard{
                            width: 480
                            card_subtitle := OxLabelSubtitle{ text: "Sample Card" }
                            OxLabelCaption{ text: "This card responds to theme changes in real-time." }
                            card_input := OxTextInput{ empty_text: "Type something..." }
                            View{
                                flow: Right spacing: 8
                                submit_btn := OxButton{ text: "Submit" }
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

        let content_bg = self.ui.view(cx, ids!(content_bg));
        content_bg.set_uniform(
            cx,
            live_id!(color),
            &[
                theme.colors.surface_secondary.x,
                theme.colors.surface_secondary.y,
                theme.colors.surface_secondary.z,
                theme.colors.surface_secondary.w,
            ],
        );
        content_bg.redraw(cx);

        let map = ThemeMap::builder(&self.ui)
            .add(live_id!(title_label), display::apply_label_title_theme)
            .add(
                live_id!(subtitle_label),
                display::apply_label_secondary_theme,
            )
            .add(live_id!(sample_card), layout::apply_card_theme)
            .add(live_id!(card_subtitle), display::apply_label_subtitle_theme)
            .add(live_id!(card_input), inputs::apply_text_input_theme)
            .add(live_id!(submit_btn), buttons::apply_button_theme)
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
        ThemeEngine::init(all_themes());
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
        if self.ui.button(cx, ids!(btn_toggle_dark)).clicked(actions) {
            let current = ThemeEngine::current();
            let name = if current.name.contains("Dark") {
                current.name.replace(" Dark", "").to_string()
            } else {
                format!("{} Dark", current.name)
            };
            self.switch_theme(cx, &name);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
