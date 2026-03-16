use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "OxideUI Showcase"
                window.inner_size: vec2(1200, 800)
                body +: {
                    View{
                        width: Fill height: Fill
                        flow: Right
                        show_bg: true
                        draw_bg.color: #FFFFFFFF

                        View{
                            width: 260 height: Fill
                            flow: Down
                            padding: Inset{top: 24. right: 16. bottom: 16. left: 16.}
                            spacing: 0
                            show_bg: true
                            draw_bg.color: #FFFFFFFF

                            View{
                                width: Fill height: Fit
                                flow: Down spacing: 4
                                padding.bottom: 24
                                OxLabelTitle{ text: "OxideUI" }
                                OxLabelCaption{ text: "Component Library" }
                            }

                            OxDivider{}

                            View{
                                width: Fill height: Fit
                                padding: Inset{top: 20. right: 0. bottom: 8. left: 4.}
                                OxLabelSecondary{ text: "COMPONENTS" }
                            }

                            View{
                                width: Fill height: Fit
                                flow: Down spacing: 4

                                nav_buttons := OxNavButton{ text: "Buttons" }
                                nav_inputs := OxNavButton{ text: "Inputs" }
                                nav_display := OxNavButton{ text: "Display" }
                                nav_feedback := OxNavButton{ text: "Feedback" }
                                nav_layout := OxNavButton{ text: "Layout" }
                            }

                            View{
                                width: Fill height: Fill
                                show_bg: true
                                draw_bg.color: #FFFFFFFF
                            }

                            OxDivider{}

                            View{
                                width: Fill height: Fit
                                flow: Down spacing: 6
                                padding: Inset{top: 12. right: 4. bottom: 4. left: 4.}
                                OxLabelCaption{ text: "v0.1.0" }
                                OxLabelCaption{ text: "Built with Makepad" }
                            }
                        }

                        SolidView{
                            width: 1 height: Fill
                            draw_bg +: { color: #E5E5E5 }
                        }

                        View{
                            width: Fill height: Fill
                            show_bg: true
                            draw_bg.color: #F5F5F5FF

                            ScrollYView{
                                width: Fill height: Fill
                                flow: Down
                                padding: Inset{top: 48. right: 48. bottom: 48. left: 48.}
                                spacing: 24

                                OxLabelTitle{ text: "Component Showcase" }
                                View{
                                    width: Fill height: Fit
                                    padding.bottom: 8
                                    OxLabelSecondary{ text: "Production-ready components with GPU-accelerated rendering" }
                                }

                                section_buttons := View{
                                    width: Fill height: Fit
                                    flow: Down spacing: 24

                                    OxCard{
                                        OxLabelSubtitle{ text: "Buttons" }
                                        OxLabelCaption{ text: "Primary, secondary, outline, ghost, and danger variants with hover and press states." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 12 align.y: 0.5
                                            OxButton{ text: "Primary" }
                                            OxButtonSecondary{ text: "Secondary" }
                                            OxButtonOutline{ text: "Outline" }
                                            OxButtonGhost{ text: "Ghost" }
                                            OxButtonDanger{ text: "Danger" }
                                        }
                                        OxDivider{}
                                        OxLabelCaption{ text: "Size variants: small, medium, and large." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 12 align.y: 0.5
                                            OxButtonSmall{ text: "Small" }
                                            OxButton{ text: "Medium" }
                                            OxButtonLarge{ text: "Large" }
                                        }
                                    }
                                }

                                section_inputs := View{
                                    width: Fill height: Fit
                                    flow: Down spacing: 24
                                    visible: false

                                    OxCard{
                                        OxLabelSubtitle{ text: "Text Inputs" }
                                        OxLabelCaption{ text: "Standard text input with focus animation." }
                                        OxTextInput{ empty_text: "Type something..." }
                                    }

                                    OxCard{
                                        OxLabelSubtitle{ text: "Controls" }
                                        OxLabelCaption{ text: "Checkbox, radio, switch, and slider controls." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 32 align.y: 0.5
                                            OxCheckbox{ text: "Accept terms" }
                                            OxRadio{ text: "Option A" }
                                            OxSwitch{}
                                        }
                                        OxDivider{}
                                        OxLabelCaption{ text: "Slider control" }
                                        OxSlider{}
                                    }
                                }

                                section_display := View{
                                    width: Fill height: Fit
                                    flow: Down spacing: 24
                                    visible: false

                                    OxCard{
                                        OxLabelSubtitle{ text: "Typography" }
                                        OxLabelCaption{ text: "Type hierarchy with consistent sizing." }
                                        OxLabelTitle{ text: "Title Label (24px)" }
                                        OxLabelBody{ text: "Body text for content (16px)" }
                                        OxLabelCaption{ text: "Caption text (12px)" }
                                        OxLabelLink{ text: "Link text" }
                                    }

                                    OxCard{
                                        OxLabelSubtitle{ text: "Badges" }
                                        OxLabelCaption{ text: "Status indicators with semantic colors." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 8 align.y: 0.5
                                            OxBadge{}
                                            OxBadgeSuccess{}
                                            OxBadgeWarning{}
                                            OxBadgeError{}
                                            OxBadgeInfo{}
                                        }
                                    }

                                    OxCard{
                                        OxLabelSubtitle{ text: "Avatars" }
                                        OxLabelCaption{ text: "User avatars in four sizes." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 16 align.y: 0.5
                                            OxAvatarSmall{}
                                            OxAvatar{}
                                            OxAvatarLarge{}
                                            OxAvatarXLarge{}
                                        }
                                    }

                                    OxCard{
                                        OxLabelSubtitle{ text: "Icons" }
                                        OxLabelCaption{ text: "SVG icons rendered via Makepad's DrawSvg system." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 12 align.y: 0.5
                                            OxIconCheck{}
                                            OxIconClose{}
                                            OxIconChevronRight{}
                                            OxIconChevronDown{}
                                            OxIconPlus{}
                                            OxIconMinus{}
                                            OxIconSearch{}
                                            OxIconMenu{}
                                            OxIconAlertCircle{}
                                            OxIconInfo{}
                                            OxIconArrowRight{}
                                            OxIconEye{}
                                            OxIconCopy{}
                                            OxIconSettings{}
                                            OxIconStar{}
                                            OxIconHeart{}
                                            OxIconExternalLink{}
                                        }
                                    }
                                }

                                section_feedback := View{
                                    width: Fill height: Fit
                                    flow: Down spacing: 24
                                    visible: false

                                    OxCard{
                                        OxLabelSubtitle{ text: "Alerts" }
                                        OxLabelCaption{ text: "Contextual feedback messages." }
                                        OxAlert{}
                                        OxAlertSuccess{}
                                        OxAlertWarning{}
                                        OxAlertError{}
                                    }

                                    OxCard{
                                        OxLabelSubtitle{ text: "Progress & Skeleton" }
                                        OxLabelCaption{ text: "Loading indicators and placeholder content." }
                                        OxProgress{}
                                        OxDivider{}
                                        View{
                                            width: Fill height: Fit
                                            flow: Down spacing: 8
                                            OxSkeleton{}
                                            View{
                                                width: Fill height: Fit
                                                flow: Right spacing: 12 align.y: 0.5
                                                OxSkeletonCircle{}
                                                View{
                                                    width: Fill height: Fit
                                                    flow: Down spacing: 6
                                                    OxSkeletonText{}
                                                    OxSkeletonText{ width: 200 }
                                                }
                                            }
                                        }
                                    }
                                }

                                section_layout := View{
                                    width: Fill height: Fit
                                    flow: Down spacing: 24
                                    visible: false

                                    OxCard{
                                        OxLabelSubtitle{ text: "Cards & Layout" }
                                        OxLabelCaption{ text: "Container components for structuring content." }
                                        OxCard{
                                            draw_bg +: { color: #FAFAFA }
                                            OxLabelBody{ text: "Nested card with different surface color" }
                                            OxLabelCaption{ text: "Cards can be nested and themed independently." }
                                        }
                                        OxDivider{}
                                        OxLabelCaption{ text: "Horizontal grid layout:" }
                                        OxGrid{
                                            OxCard{
                                                width: Fill
                                                draw_bg +: { color: #FAFAFA }
                                                OxLabelBody{ text: "Column 1" }
                                            }
                                            OxCard{
                                                width: Fill
                                                draw_bg +: { color: #FAFAFA }
                                                OxLabelBody{ text: "Column 2" }
                                            }
                                            OxCard{
                                                width: Fill
                                                draw_bg +: { color: #FAFAFA }
                                                OxLabelBody{ text: "Column 3" }
                                            }
                                        }
                                    }
                                }
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

    fn set_active_nav(&self, cx: &mut Cx, section: &str) {
        for s in ["buttons", "inputs", "display", "feedback", "layout"] {
            let is_active = s == section;
            let (nav, sec) = match s {
                "buttons" => (ids!(nav_buttons), ids!(section_buttons)),
                "inputs" => (ids!(nav_inputs), ids!(section_inputs)),
                "display" => (ids!(nav_display), ids!(section_display)),
                "feedback" => (ids!(nav_feedback), ids!(section_feedback)),
                "layout" => (ids!(nav_layout), ids!(section_layout)),
                _ => continue,
            };
            let btn = self.ui.button(cx, nav);
            if is_active {
                oxide_widgets::buttons::set_nav_button_active(cx, &btn);
            } else {
                oxide_widgets::buttons::set_nav_button_inactive(cx, &btn);
            }
            self.ui.view(cx, sec).set_visible(cx, is_active);
        }
        self.ui.redraw(cx);
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.set_active_nav(cx, "buttons");
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(nav_buttons)).clicked(actions) {
            self.set_active_nav(cx, "buttons");
        }
        if self.ui.button(cx, ids!(nav_inputs)).clicked(actions) {
            self.set_active_nav(cx, "inputs");
        }
        if self.ui.button(cx, ids!(nav_display)).clicked(actions) {
            self.set_active_nav(cx, "display");
        }
        if self.ui.button(cx, ids!(nav_feedback)).clicked(actions) {
            self.set_active_nav(cx, "feedback");
        }
        if self.ui.button(cx, ids!(nav_layout)).clicked(actions) {
            self.set_active_nav(cx, "layout");
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
