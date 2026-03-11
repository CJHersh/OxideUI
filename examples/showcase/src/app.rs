use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            font_family: FontFamily{
                latin := FontMember{res: crate_resource("self:resources/Inter-Regular.ttf") asc: -0.1 desc: 0.0}
            }
            main_window := Window{
                window.title: "OxideUI Showcase"
                window.inner_size: vec2(1200, 800)
                body +: {
                    View{
                        width: Fill height: Fill
                        flow: Right

                        View{
                            width: 260 height: Fill
                            flow: Down
                            padding: Inset{top: 32. right: 16. bottom: 16. left: 16.}
                            spacing: 0
                            show_bg: true
                            draw_bg.color: #FAFAFAFF

                            View{
                                width: Fill height: Fit
                                flow: Down spacing: 2
                                padding.bottom: 24
                                Label{
                                    draw_text +: {
                                        color: uniform(#0A0A0A)
                                        text_style +: { font_size: 22. }
                                    }
                                    text: "OxideUI"
                                }
                                Label{
                                    draw_text +: {
                                        color: uniform(#A3A3A3)
                                        text_style +: { font_size: 11. }
                                    }
                                    text: "Component Library"
                                }
                            }

                            OxDivider{}

                            View{
                                width: Fill height: Fit
                                padding: Inset{top: 20. right: 0. bottom: 12. left: 4.}
                                Label{
                                    draw_text +: {
                                        color: uniform(#A3A3A3)
                                        text_style +: { font_size: 10. }
                                    }
                                    text: "COMPONENTS"
                                }
                            }

                            View{
                                width: Fill height: Fit
                                flow: Down spacing: 2

                                View{
                                    width: Fill height: Fit flow: Overlay
                                    nav_on_buttons := ButtonFlat{
                                        width: Fill text: "Buttons"
                                        draw_bg +: { color: uniform(#F5F5F5) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#0A0A0A) text_style +: { font_size: 13. } }
                                    }
                                    nav_off_buttons := ButtonFlat{
                                        width: Fill text: "Buttons" visible: false
                                        draw_bg +: { color: uniform(#FAFAFA) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#737373) text_style +: { font_size: 13. } }
                                    }
                                }
                                View{
                                    width: Fill height: Fit flow: Overlay
                                    nav_on_inputs := ButtonFlat{
                                        width: Fill text: "Inputs" visible: false
                                        draw_bg +: { color: uniform(#F5F5F5) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#0A0A0A) text_style +: { font_size: 13. } }
                                    }
                                    nav_off_inputs := ButtonFlat{
                                        width: Fill text: "Inputs"
                                        draw_bg +: { color: uniform(#FAFAFA) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#737373) text_style +: { font_size: 13. } }
                                    }
                                }
                                View{
                                    width: Fill height: Fit flow: Overlay
                                    nav_on_display := ButtonFlat{
                                        width: Fill text: "Display" visible: false
                                        draw_bg +: { color: uniform(#F5F5F5) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#0A0A0A) text_style +: { font_size: 13. } }
                                    }
                                    nav_off_display := ButtonFlat{
                                        width: Fill text: "Display"
                                        draw_bg +: { color: uniform(#FAFAFA) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#737373) text_style +: { font_size: 13. } }
                                    }
                                }
                                View{
                                    width: Fill height: Fit flow: Overlay
                                    nav_on_feedback := ButtonFlat{
                                        width: Fill text: "Feedback" visible: false
                                        draw_bg +: { color: uniform(#F5F5F5) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#0A0A0A) text_style +: { font_size: 13. } }
                                    }
                                    nav_off_feedback := ButtonFlat{
                                        width: Fill text: "Feedback"
                                        draw_bg +: { color: uniform(#FAFAFA) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#737373) text_style +: { font_size: 13. } }
                                    }
                                }
                                View{
                                    width: Fill height: Fit flow: Overlay
                                    nav_on_layout := ButtonFlat{
                                        width: Fill text: "Layout" visible: false
                                        draw_bg +: { color: uniform(#F5F5F5) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#0A0A0A) text_style +: { font_size: 13. } }
                                    }
                                    nav_off_layout := ButtonFlat{
                                        width: Fill text: "Layout"
                                        draw_bg +: { color: uniform(#FAFAFA) border_radius: uniform(6.0) }
                                        draw_text +: { color: uniform(#737373) text_style +: { font_size: 13. } }
                                    }
                                }
                            }

                            View{ width: Fill height: Fill }

                            OxDivider{}

                            View{
                                width: Fill height: Fit
                                flow: Down spacing: 6
                                padding: Inset{top: 12. right: 4. bottom: 4. left: 4.}
                                Label{
                                    draw_text +: { color: uniform(#A3A3A3) text_style +: { font_size: 10. } }
                                    text: "v0.1.0"
                                }
                                Label{
                                    draw_text +: { color: uniform(#A3A3A3) text_style +: { font_size: 10. } }
                                    text: "Built with Makepad"
                                }
                            }
                        }

                        SolidView{
                            width: 1 height: Fill
                            draw_bg +: { color: uniform(#E5E5E5) }
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
                                        OxLabelCaption{ text: "SDF-rendered vector icons that scale perfectly at any size." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 12 align.y: 0.5
                                            OxIcon{ draw_bg.icon_type: 0.0 }
                                            OxIcon{ draw_bg.icon_type: 1.0 }
                                            OxIcon{ draw_bg.icon_type: 2.0 }
                                            OxIcon{ draw_bg.icon_type: 4.0 }
                                            OxIcon{ draw_bg.icon_type: 6.0 }
                                            OxIcon{ draw_bg.icon_type: 7.0 }
                                            OxIcon{ draw_bg.icon_type: 8.0 }
                                            OxIcon{ draw_bg.icon_type: 9.0 }
                                            OxIcon{ draw_bg.icon_type: 10.0 }
                                            OxIcon{ draw_bg.icon_type: 11.0 }
                                            OxIcon{ draw_bg.icon_type: 12.0 }
                                            OxIcon{ draw_bg.icon_type: 14.0 }
                                            OxIcon{ draw_bg.icon_type: 15.0 }
                                            OxIcon{ draw_bg.icon_type: 16.0 }
                                            OxIcon{ draw_bg.icon_type: 17.0 }
                                            OxIcon{ draw_bg.icon_type: 18.0 }
                                            OxIcon{ draw_bg.icon_type: 19.0 }
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
                                            draw_bg +: { color: uniform(#FAFAFA) }
                                            OxLabelBody{ text: "Nested card with different surface color" }
                                            OxLabelCaption{ text: "Cards can be nested and themed independently." }
                                        }
                                        OxDivider{}
                                        OxLabelCaption{ text: "Horizontal grid layout:" }
                                        OxGrid{
                                            OxCard{
                                                width: Fill
                                                draw_bg +: { color: uniform(#FAFAFA) }
                                                OxLabelBody{ text: "Column 1" }
                                            }
                                            OxCard{
                                                width: Fill
                                                draw_bg +: { color: uniform(#FAFAFA) }
                                                OxLabelBody{ text: "Column 2" }
                                            }
                                            OxCard{
                                                width: Fill
                                                draw_bg +: { color: uniform(#FAFAFA) }
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
            let (on, off, sec) = match s {
                "buttons" => (ids!(nav_on_buttons), ids!(nav_off_buttons), ids!(section_buttons)),
                "inputs" => (ids!(nav_on_inputs), ids!(nav_off_inputs), ids!(section_inputs)),
                "display" => (ids!(nav_on_display), ids!(nav_off_display), ids!(section_display)),
                "feedback" => (ids!(nav_on_feedback), ids!(nav_off_feedback), ids!(section_feedback)),
                "layout" => (ids!(nav_on_layout), ids!(nav_off_layout), ids!(section_layout)),
                _ => continue,
            };
            self.ui.button(cx, on).set_visible(cx, is_active);
            self.ui.button(cx, off).set_visible(cx, !is_active);
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
    fn handle_startup(&mut self, _cx: &mut Cx) {}

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(nav_on_buttons)).clicked(actions)
            || self.ui.button(cx, ids!(nav_off_buttons)).clicked(actions)
        {
            self.set_active_nav(cx, "buttons");
        }
        if self.ui.button(cx, ids!(nav_on_inputs)).clicked(actions)
            || self.ui.button(cx, ids!(nav_off_inputs)).clicked(actions)
        {
            self.set_active_nav(cx, "inputs");
        }
        if self.ui.button(cx, ids!(nav_on_display)).clicked(actions)
            || self.ui.button(cx, ids!(nav_off_display)).clicked(actions)
        {
            self.set_active_nav(cx, "display");
        }
        if self.ui.button(cx, ids!(nav_on_feedback)).clicked(actions)
            || self.ui.button(cx, ids!(nav_off_feedback)).clicked(actions)
        {
            self.set_active_nav(cx, "feedback");
        }
        if self.ui.button(cx, ids!(nav_on_layout)).clicked(actions)
            || self.ui.button(cx, ids!(nav_off_layout)).clicked(actions)
        {
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
