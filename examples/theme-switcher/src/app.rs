use makepad_widgets::*;

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
                    View{
                        width: Fill height: Fill
                        flow: Down
                        padding: 48
                        spacing: 24
                        align.x: 0.5
                        show_bg: true
                        draw_bg.color: #F5F5F5FF

                        OxLabelTitle{ text: "Theme Switcher" }
                        OxLabelSecondary{ text: "Runtime theme switching is planned for a future release" }

                        OxCard{
                            width: 480
                            OxLabelSubtitle{ text: "Sample Card" }
                            OxLabelCaption{ text: "Components render with shadcn light DSL defaults." }
                            OxTextInput{ empty_text: "Type something..." }
                            View{
                                flow: Right spacing: 8
                                OxButton{ text: "Submit" }
                                OxButtonSecondary{ text: "Cancel" }
                            }
                        }
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
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {}
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
