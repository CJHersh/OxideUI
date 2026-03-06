use makepad_widgets::*;
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::themes::all_themes;
use oxide_widgets::theme_apply::{set_widget_draw_uniform, v4, ThemeMap};
use oxide_widgets::{buttons, display, feedback, inputs, layout};

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

                        sidebar := View{
                            width: 260 height: Fill
                            flow: Down
                            padding: Inset{top: 32. right: 16. bottom: 16. left: 16.}
                            spacing: 0
                            show_bg: true
                            draw_bg.color: #F9FAFBFF

                            View{
                                width: Fill height: Fit
                                flow: Down spacing: 2
                                padding.bottom: 24
                                logo_title := Label{
                                    draw_text +: {
                                        color: uniform(#111827)
                                        text_style +: { font_size: 22. }
                                    }
                                    text: "OxideUI"
                                }
                                logo_subtitle := Label{
                                    draw_text +: {
                                        color: uniform(#9CA3AF)
                                        text_style +: { font_size: 11. }
                                    }
                                    text: "Component Library"
                                }
                            }

                            divider_1 := OxDivider{ draw_bg.color: #E5E7EBFF }

                            View{
                                width: Fill height: Fit
                                padding: Inset{top: 20. right: 0. bottom: 12. left: 4.}
                                section_themes_label := Label{
                                    draw_text +: {
                                        color: uniform(#9CA3AF)
                                        text_style +: { font_size: 10. }
                                    }
                                    text: "THEMES"
                                }
                            }

                            View{
                                width: Fill height: Fit
                                flow: Down spacing: 2
                                padding.bottom: 8

                                View{
                                    width: Fill height: Fit
                                    flow: Right align.y: 0.5 spacing: 8
                                    openai_indicator := RoundedView{
                                        width: 3 height: 20
                                        draw_bg.color: #10A37FFF
                                        draw_bg.border_radius: 1.5
                                    }
                                    btn_openai := ButtonFlat{
                                        width: Fill text: "OpenAI"
                                        draw_bg +: {
                                            color: uniform(#F3F4F6)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#111827)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                }
                                View{
                                    width: Fill height: Fit
                                    flow: Right align.y: 0.5 spacing: 8
                                    airbnb_indicator := RoundedView{
                                        width: 3 height: 20
                                        visible: false
                                        draw_bg.color: #FF5A5FFF
                                        draw_bg.border_radius: 1.5
                                    }
                                    btn_airbnb := ButtonFlat{
                                        width: Fill text: "Airbnb"
                                        draw_bg +: {
                                            color: uniform(#F9FAFB)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#6B7280)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                }
                                View{
                                    width: Fill height: Fit
                                    flow: Right align.y: 0.5 spacing: 8
                                    notion_indicator := RoundedView{
                                        width: 3 height: 20
                                        visible: false
                                        draw_bg.color: #346CA3FF
                                        draw_bg.border_radius: 1.5
                                    }
                                    btn_notion := ButtonFlat{
                                        width: Fill text: "Notion"
                                        draw_bg +: {
                                            color: uniform(#F9FAFB)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#6B7280)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                }
                            }

                            View{
                                width: Fill height: Fit
                                padding: Inset{top: 8. right: 0. bottom: 12. left: 4.}
                                section_mode_label := Label{
                                    draw_text +: {
                                        color: uniform(#9CA3AF)
                                        text_style +: { font_size: 10. }
                                    }
                                    text: "MODE"
                                }
                            }

                            View{
                                width: Fill height: Fit
                                padding.bottom: 8
                                mode_toggle_bg := RoundedView{
                                    width: Fill height: Fit
                                    show_bg: true
                                    draw_bg +: {
                                        color: uniform(#F3F4F6)
                                        border_radius: uniform(8.0)
                                    }
                                    flow: Right spacing: 2
                                    padding: Inset{top: 3. right: 3. bottom: 3. left: 3.}

                                    btn_light := ButtonFlat{
                                        width: Fill text: "Light"
                                        draw_bg +: {
                                            color: uniform(#FFFFFF)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#111827)
                                            text_style +: { font_size: 12. }
                                        }
                                    }
                                    btn_dark := ButtonFlat{
                                        width: Fill text: "Dark"
                                        draw_bg +: {
                                            color: uniform(#F3F4F6)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#6B7280)
                                            text_style +: { font_size: 12. }
                                        }
                                    }
                                }
                            }

                            divider_2 := OxDivider{ draw_bg.color: #E5E7EBFF }

                            View{
                                width: Fill height: Fit
                                padding: Inset{top: 20. right: 0. bottom: 12. left: 4.}
                                section_components_label := Label{
                                    draw_text +: {
                                        color: uniform(#9CA3AF)
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
                                        draw_bg +: {
                                            color: uniform(#F3F4F6)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#111827)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                    nav_off_buttons := ButtonFlat{
                                        width: Fill text: "Buttons"
                                        visible: false
                                        draw_bg +: {
                                            color: uniform(#F9FAFB)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#6B7280)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                }
                                View{
                                    width: Fill height: Fit flow: Overlay
                                    nav_on_inputs := ButtonFlat{
                                        width: Fill text: "Inputs"
                                        visible: false
                                        draw_bg +: {
                                            color: uniform(#F3F4F6)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#111827)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                    nav_off_inputs := ButtonFlat{
                                        width: Fill text: "Inputs"
                                        draw_bg +: {
                                            color: uniform(#F9FAFB)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#6B7280)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                }
                                View{
                                    width: Fill height: Fit flow: Overlay
                                    nav_on_display := ButtonFlat{
                                        width: Fill text: "Display"
                                        visible: false
                                        draw_bg +: {
                                            color: uniform(#F3F4F6)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#111827)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                    nav_off_display := ButtonFlat{
                                        width: Fill text: "Display"
                                        draw_bg +: {
                                            color: uniform(#F9FAFB)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#6B7280)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                }
                                View{
                                    width: Fill height: Fit flow: Overlay
                                    nav_on_feedback := ButtonFlat{
                                        width: Fill text: "Feedback"
                                        visible: false
                                        draw_bg +: {
                                            color: uniform(#F3F4F6)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#111827)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                    nav_off_feedback := ButtonFlat{
                                        width: Fill text: "Feedback"
                                        draw_bg +: {
                                            color: uniform(#F9FAFB)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#6B7280)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                }
                                View{
                                    width: Fill height: Fit flow: Overlay
                                    nav_on_layout := ButtonFlat{
                                        width: Fill text: "Layout"
                                        visible: false
                                        draw_bg +: {
                                            color: uniform(#F3F4F6)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#111827)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                    nav_off_layout := ButtonFlat{
                                        width: Fill text: "Layout"
                                        draw_bg +: {
                                            color: uniform(#F9FAFB)
                                            border_radius: uniform(6.0)
                                        }
                                        draw_text +: {
                                            color: uniform(#6B7280)
                                            text_style +: { font_size: 13. }
                                        }
                                    }
                                }
                            }

                            View{ width: Fill height: Fill }

                            footer_divider := OxDivider{ draw_bg.color: #E5E7EBFF }

                            View{
                                width: Fill height: Fit
                                flow: Down spacing: 6
                                padding: Inset{top: 12. right: 4. bottom: 4. left: 4.}

                                View{
                                    width: Fill height: Fit
                                    flow: Right spacing: 8 align.y: 0.5

                                    View{
                                        width: 8 height: 8
                                        flow: Overlay
                                        theme_dot_openai := RoundedView{
                                            width: 8 height: 8
                                            draw_bg.color: #10A37FFF
                                            draw_bg.border_radius: 4.0
                                        }
                                        theme_dot_airbnb := RoundedView{
                                            width: 8 height: 8
                                            visible: false
                                            draw_bg.color: #FF5A5FFF
                                            draw_bg.border_radius: 4.0
                                        }
                                        theme_dot_notion := RoundedView{
                                            width: 8 height: 8
                                            visible: false
                                            draw_bg.color: #346CA3FF
                                            draw_bg.border_radius: 4.0
                                        }
                                    }
                                    current_theme := Label{
                                        draw_text +: {
                                            color: uniform(#6B7280)
                                            text_style +: { font_size: 11. }
                                        }
                                        text: "OpenAI"
                                    }
                                }

                                footer_version := Label{
                                    draw_text +: {
                                        color: uniform(#9CA3AF)
                                        text_style +: { font_size: 10. }
                                    }
                                    text: "v0.1.0"
                                }
                                footer_branding := Label{
                                    draw_text +: {
                                        color: uniform(#9CA3AF)
                                        text_style +: { font_size: 10. }
                                    }
                                    text: "Built with Makepad"
                                }
                            }
                        }

                        sidebar_border := SolidView{
                            width: 1 height: Fill
                            draw_bg +: {
                                color: uniform(#E5E7EB)
                            }
                        }

                        content_bg := View{
                            width: Fill height: Fill
                            show_bg: true
                            draw_bg.color: #F4F4F5FF

                            ScrollYView{
                                width: Fill height: Fill
                                flow: Down
                                padding: Inset{top: 48. right: 48. bottom: 48. left: 48.}
                                spacing: 24

                                showcase_title := OxLabelTitle{ text: "Component Showcase" }
                                View{
                                    width: Fill height: Fit
                                    padding.bottom: 8
                                    showcase_subtitle := OxLabelSecondary{ text: "Production-ready components with GPU-accelerated rendering" }
                                }

                                section_buttons := View{
                                    width: Fill height: Fit
                                    flow: Down spacing: 24

                                    buttons_card := OxCard{
                                        buttons_title := OxLabelSubtitle{ text: "Buttons" }
                                        OxLabelCaption{ text: "Primary, secondary, ghost, and danger variants with hover and press states." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 12 align.y: 0.5
                                            btn_primary := OxButton{ text: "Primary" }
                                            btn_secondary := OxButtonSecondary{ text: "Secondary" }
                                            btn_ghost := OxButtonGhost{ text: "Ghost" }
                                            btn_danger := OxButtonDanger{ text: "Danger" }
                                        }
                                        OxDivider{}
                                        OxLabelCaption{ text: "Size variants: small, medium, and large." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 12 align.y: 0.5
                                            btn_small := OxButtonSmall{ text: "Small" }
                                            btn_medium := OxButton{ text: "Medium" }
                                            btn_large := OxButtonLarge{ text: "Large" }
                                        }
                                    }
                                }

                                section_inputs := View{
                                    width: Fill height: Fit
                                    flow: Down spacing: 24
                                    visible: false

                                    inputs_card := OxCard{
                                        inputs_title := OxLabelSubtitle{ text: "Text Inputs" }
                                        OxLabelCaption{ text: "Standard text input with focus animation." }
                                        demo_input := OxTextInput{ empty_text: "Type something..." }
                                    }

                                    controls_card := OxCard{
                                        controls_title := OxLabelSubtitle{ text: "Controls" }
                                        OxLabelCaption{ text: "Checkbox, radio, switch, and slider controls." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 32 align.y: 0.5
                                            demo_checkbox := OxCheckbox{ text: "Accept terms" }
                                            demo_radio := OxRadio{ text: "Option A" }
                                            demo_switch := OxSwitch{}
                                        }
                                        OxDivider{}
                                        OxLabelCaption{ text: "Slider control" }
                                        demo_slider := OxSlider{}
                                    }
                                }

                                section_display := View{
                                    width: Fill height: Fit
                                    flow: Down spacing: 24
                                    visible: false

                                    labels_card := OxCard{
                                        labels_title := OxLabelSubtitle{ text: "Typography" }
                                        OxLabelCaption{ text: "Type hierarchy with consistent sizing." }
                                        demo_title := OxLabelTitle{ text: "Title Label (24px)" }
                                        demo_body := OxLabelBody{ text: "Body text for content (16px)" }
                                        demo_caption := OxLabelCaption{ text: "Caption text (12px)" }
                                        demo_link := OxLabelLink{ text: "Link text" }
                                    }

                                    badges_card := OxCard{
                                        badges_title := OxLabelSubtitle{ text: "Badges" }
                                        OxLabelCaption{ text: "Status indicators with semantic colors." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 8 align.y: 0.5
                                            demo_badge := OxBadge{}
                                            demo_badge_success := OxBadgeSuccess{}
                                            demo_badge_warning := OxBadgeWarning{}
                                            demo_badge_error := OxBadgeError{}
                                            demo_badge_info := OxBadgeInfo{}
                                        }
                                    }

                                    avatars_card := OxCard{
                                        avatars_title := OxLabelSubtitle{ text: "Avatars" }
                                        OxLabelCaption{ text: "User avatars in four sizes." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 16 align.y: 0.5
                                            demo_avatar_sm := OxAvatarSmall{}
                                            demo_avatar := OxAvatar{}
                                            demo_avatar_lg := OxAvatarLarge{}
                                            demo_avatar_xl := OxAvatarXLarge{}
                                        }
                                    }

                                    icons_card := OxCard{
                                        icons_title := OxLabelSubtitle{ text: "Icons" }
                                        OxLabelCaption{ text: "SDF-rendered vector icons that scale perfectly at any size." }
                                        View{
                                            width: Fill height: Fit
                                            flow: Right spacing: 12 align.y: 0.5
                                            icon_check := OxIcon{ draw_bg.icon_type: 0.0 }
                                            icon_close := OxIcon{ draw_bg.icon_type: 1.0 }
                                            icon_chevron_r := OxIcon{ draw_bg.icon_type: 2.0 }
                                            icon_chevron_d := OxIcon{ draw_bg.icon_type: 4.0 }
                                            icon_plus := OxIcon{ draw_bg.icon_type: 6.0 }
                                            icon_minus := OxIcon{ draw_bg.icon_type: 7.0 }
                                            icon_search := OxIcon{ draw_bg.icon_type: 8.0 }
                                            icon_menu := OxIcon{ draw_bg.icon_type: 9.0 }
                                            icon_alert := OxIcon{ draw_bg.icon_type: 10.0 }
                                            icon_info := OxIcon{ draw_bg.icon_type: 11.0 }
                                            icon_arrow_r := OxIcon{ draw_bg.icon_type: 12.0 }
                                            icon_eye := OxIcon{ draw_bg.icon_type: 14.0 }
                                            icon_copy := OxIcon{ draw_bg.icon_type: 15.0 }
                                            icon_settings := OxIcon{ draw_bg.icon_type: 16.0 }
                                            icon_star := OxIcon{ draw_bg.icon_type: 17.0 }
                                            icon_heart := OxIcon{ draw_bg.icon_type: 18.0 }
                                            icon_external := OxIcon{ draw_bg.icon_type: 19.0 }
                                        }
                                    }
                                }

                                section_feedback := View{
                                    width: Fill height: Fit
                                    flow: Down spacing: 24
                                    visible: false

                                    feedback_card := OxCard{
                                        feedback_title := OxLabelSubtitle{ text: "Alerts" }
                                        OxLabelCaption{ text: "Contextual feedback messages." }
                                        demo_alert := OxAlert{}
                                        demo_alert_success := OxAlertSuccess{}
                                        demo_alert_warning := OxAlertWarning{}
                                        demo_alert_error := OxAlertError{}
                                    }

                                    progress_card := OxCard{
                                        progress_title := OxLabelSubtitle{ text: "Progress & Skeleton" }
                                        OxLabelCaption{ text: "Loading indicators and placeholder content." }
                                        demo_progress := OxProgress{}
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

                                    layout_card := OxCard{
                                        layout_title := OxLabelSubtitle{ text: "Cards & Layout" }
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
        let sections = ["buttons", "inputs", "display", "feedback", "layout"];
        for s in sections {
            let is_active = s == section;
            match s {
                "buttons" => {
                    self.ui
                        .button(cx, ids!(nav_on_buttons))
                        .set_visible(cx, is_active);
                    self.ui
                        .button(cx, ids!(nav_off_buttons))
                        .set_visible(cx, !is_active);
                    self.ui
                        .view(cx, ids!(section_buttons))
                        .set_visible(cx, is_active);
                }
                "inputs" => {
                    self.ui
                        .button(cx, ids!(nav_on_inputs))
                        .set_visible(cx, is_active);
                    self.ui
                        .button(cx, ids!(nav_off_inputs))
                        .set_visible(cx, !is_active);
                    self.ui
                        .view(cx, ids!(section_inputs))
                        .set_visible(cx, is_active);
                }
                "display" => {
                    self.ui
                        .button(cx, ids!(nav_on_display))
                        .set_visible(cx, is_active);
                    self.ui
                        .button(cx, ids!(nav_off_display))
                        .set_visible(cx, !is_active);
                    self.ui
                        .view(cx, ids!(section_display))
                        .set_visible(cx, is_active);
                }
                "feedback" => {
                    self.ui
                        .button(cx, ids!(nav_on_feedback))
                        .set_visible(cx, is_active);
                    self.ui
                        .button(cx, ids!(nav_off_feedback))
                        .set_visible(cx, !is_active);
                    self.ui
                        .view(cx, ids!(section_feedback))
                        .set_visible(cx, is_active);
                }
                "layout" => {
                    self.ui
                        .button(cx, ids!(nav_on_layout))
                        .set_visible(cx, is_active);
                    self.ui
                        .button(cx, ids!(nav_off_layout))
                        .set_visible(cx, !is_active);
                    self.ui
                        .view(cx, ids!(section_layout))
                        .set_visible(cx, is_active);
                }
                _ => {}
            }
        }
        self.ui.redraw(cx);
    }

    fn switch_theme(&self, cx: &mut Cx, name: &str) {
        ThemeEngine::switch_by_name(name);
        let theme = ThemeEngine::current();

        let content_bg = self.ui.view(cx, ids!(content_bg));
        content_bg.set_uniform(
            cx,
            live_id!(color),
            &v4(theme.colors.surface_secondary),
        );
        content_bg.redraw(cx);

        let map = ThemeMap::builder(&self.ui)
            .add(live_id!(showcase_title), display::apply_label_title_theme)
            .add(
                live_id!(showcase_subtitle),
                display::apply_label_secondary_theme,
            )
            .add(live_id!(buttons_card), layout::apply_card_theme)
            .add(live_id!(buttons_title), display::apply_label_subtitle_theme)
            .add(live_id!(btn_primary), buttons::apply_button_theme)
            .add(
                live_id!(btn_secondary),
                buttons::apply_button_secondary_theme,
            )
            .add(live_id!(btn_ghost), buttons::apply_button_ghost_theme)
            .add(live_id!(btn_danger), buttons::apply_button_danger_theme)
            .add(live_id!(btn_small), buttons::apply_button_small_theme)
            .add(live_id!(btn_medium), buttons::apply_button_theme)
            .add(live_id!(btn_large), buttons::apply_button_large_theme)
            .add(live_id!(inputs_card), layout::apply_card_theme)
            .add(live_id!(inputs_title), display::apply_label_subtitle_theme)
            .add(live_id!(demo_input), inputs::apply_text_input_theme)
            .add(live_id!(controls_card), layout::apply_card_theme)
            .add(
                live_id!(controls_title),
                display::apply_label_subtitle_theme,
            )
            .add(live_id!(demo_checkbox), inputs::apply_checkbox_theme)
            .add(live_id!(demo_radio), inputs::apply_radio_theme)
            .add(live_id!(demo_switch), inputs::apply_switch_theme)
            .add(live_id!(demo_slider), inputs::apply_slider_theme)
            .add(live_id!(labels_card), layout::apply_card_theme)
            .add(live_id!(labels_title), display::apply_label_subtitle_theme)
            .add(live_id!(demo_title), display::apply_label_title_theme)
            .add(live_id!(demo_body), display::apply_label_body_theme)
            .add(live_id!(demo_caption), display::apply_label_caption_theme)
            .add(live_id!(demo_link), display::apply_label_link_theme)
            .add(live_id!(badges_card), layout::apply_card_theme)
            .add(live_id!(badges_title), display::apply_label_subtitle_theme)
            .add(live_id!(demo_badge), display::apply_badge_theme)
            .add(
                live_id!(demo_badge_success),
                display::apply_badge_success_theme,
            )
            .add(
                live_id!(demo_badge_warning),
                display::apply_badge_warning_theme,
            )
            .add(live_id!(demo_badge_error), display::apply_badge_error_theme)
            .add(live_id!(demo_badge_info), display::apply_badge_info_theme)
            .add(live_id!(avatars_card), layout::apply_card_theme)
            .add(live_id!(avatars_title), display::apply_label_subtitle_theme)
            .add(live_id!(demo_avatar_sm), display::apply_avatar_theme)
            .add(live_id!(demo_avatar), display::apply_avatar_theme)
            .add(live_id!(demo_avatar_lg), display::apply_avatar_theme)
            .add(live_id!(demo_avatar_xl), display::apply_avatar_theme)
            .add(live_id!(icons_card), layout::apply_card_theme)
            .add(live_id!(icons_title), display::apply_label_subtitle_theme)
            .add(live_id!(icon_check), display::apply_icon_theme)
            .add(live_id!(icon_close), display::apply_icon_theme)
            .add(live_id!(icon_chevron_r), display::apply_icon_theme)
            .add(live_id!(icon_chevron_d), display::apply_icon_theme)
            .add(live_id!(icon_plus), display::apply_icon_theme)
            .add(live_id!(icon_minus), display::apply_icon_theme)
            .add(live_id!(icon_search), display::apply_icon_theme)
            .add(live_id!(icon_menu), display::apply_icon_theme)
            .add(live_id!(icon_alert), display::apply_icon_theme)
            .add(live_id!(icon_info), display::apply_icon_theme)
            .add(live_id!(icon_arrow_r), display::apply_icon_theme)
            .add(live_id!(icon_eye), display::apply_icon_theme)
            .add(live_id!(icon_copy), display::apply_icon_theme)
            .add(live_id!(icon_settings), display::apply_icon_theme)
            .add(live_id!(icon_star), display::apply_icon_theme)
            .add(live_id!(icon_heart), display::apply_icon_theme)
            .add(live_id!(icon_external), display::apply_icon_theme)
            .add(live_id!(feedback_card), layout::apply_card_theme)
            .add(
                live_id!(feedback_title),
                display::apply_label_subtitle_theme,
            )
            .add(live_id!(demo_alert), feedback::apply_alert_theme)
            .add(
                live_id!(demo_alert_success),
                feedback::apply_alert_success_theme,
            )
            .add(
                live_id!(demo_alert_warning),
                feedback::apply_alert_warning_theme,
            )
            .add(
                live_id!(demo_alert_error),
                feedback::apply_alert_error_theme,
            )
            .add(live_id!(progress_card), layout::apply_card_theme)
            .add(
                live_id!(progress_title),
                display::apply_label_subtitle_theme,
            )
            .add(live_id!(demo_progress), feedback::apply_progress_theme)
            .add(live_id!(layout_card), layout::apply_card_theme)
            .add(live_id!(layout_title), display::apply_label_subtitle_theme)
            .build();

        map.apply_all(cx, &theme);

        let base = name.replace(" Dark", "");
        let is_dark = name.contains("Dark");

        self.update_sidebar_chrome(cx, is_dark, &base);

        self.ui
            .view(cx, ids!(openai_indicator))
            .set_visible(cx, base == "OpenAI");
        self.ui
            .view(cx, ids!(airbnb_indicator))
            .set_visible(cx, base == "Airbnb");
        self.ui
            .view(cx, ids!(notion_indicator))
            .set_visible(cx, base == "Notion");

        self.ui
            .view(cx, ids!(theme_dot_openai))
            .set_visible(cx, base == "OpenAI");
        self.ui
            .view(cx, ids!(theme_dot_airbnb))
            .set_visible(cx, base == "Airbnb");
        self.ui
            .view(cx, ids!(theme_dot_notion))
            .set_visible(cx, base == "Notion");

        self.ui.label(cx, ids!(current_theme)).set_text(cx, name);
        self.ui.redraw(cx);
    }

    /// Updates all sidebar chrome colors for the current light/dark mode.
    ///
    /// Active button backgrounds stay light (#F3F4F6) in both modes so that
    /// the fixed dark text (#111827) remains readable. Only the sidebar surface,
    /// dividers, labels, inactive button backgrounds, and the mode-toggle
    /// container change between modes.
    fn update_sidebar_chrome(&self, cx: &mut Cx, is_dark: bool, active_theme_base: &str) {
        let sidebar_bg;
        let primary_text;
        let section_text;
        let secondary_text;
        let divider_col;
        let inactive_btn_bg;
        let toggle_container;

        if is_dark {
            sidebar_bg      = vec4(0.094, 0.094, 0.106, 1.0); // #18181B
            primary_text    = vec4(0.980, 0.980, 0.980, 1.0); // #FAFAFA
            section_text    = vec4(0.322, 0.322, 0.357, 1.0); // #52525B
            secondary_text  = vec4(0.443, 0.443, 0.478, 1.0); // #71717A
            divider_col     = vec4(0.153, 0.153, 0.165, 1.0); // #27272A
            inactive_btn_bg = vec4(0.094, 0.094, 0.106, 1.0); // #18181B
            toggle_container= vec4(0.153, 0.153, 0.165, 1.0); // #27272A
        } else {
            sidebar_bg      = vec4(0.976, 0.980, 0.984, 1.0); // #F9FAFB
            primary_text    = vec4(0.067, 0.094, 0.153, 1.0); // #111827
            section_text    = vec4(0.612, 0.639, 0.686, 1.0); // #9CA3AF
            secondary_text  = vec4(0.420, 0.447, 0.502, 1.0); // #6B7280
            divider_col     = vec4(0.898, 0.906, 0.922, 1.0); // #E5E7EB
            inactive_btn_bg = vec4(0.976, 0.980, 0.984, 1.0); // #F9FAFB
            toggle_container= vec4(0.953, 0.957, 0.965, 1.0); // #F3F4F6
        }

        let active_btn_bg = vec4(0.953, 0.957, 0.965, 1.0); // #F3F4F6 in both modes
        let toggle_active = vec4(1.0, 1.0, 1.0, 1.0);       // #FFFFFF pill

        // -- sidebar background --
        self.ui.view(cx, ids!(sidebar)).set_uniform(cx, live_id!(color), &v4(sidebar_bg));

        // -- dividers and border --
        for id in [live_id!(divider_1), live_id!(divider_2), live_id!(footer_divider)] {
            let w = self.ui.widget(cx, &[id]);
            set_widget_draw_uniform(cx, w.area(), live_id!(color), &v4(divider_col));
            w.redraw(cx);
        }
        let bw = self.ui.widget(cx, &[live_id!(sidebar_border)]);
        set_widget_draw_uniform(cx, bw.area(), live_id!(color), &v4(divider_col));
        bw.redraw(cx);

        // -- labels --
        let labels: &[(LiveId, Vec4)] = &[
            (live_id!(logo_title), primary_text),
            (live_id!(logo_subtitle), section_text),
            (live_id!(section_themes_label), section_text),
            (live_id!(section_mode_label), section_text),
            (live_id!(section_components_label), section_text),
            (live_id!(current_theme), secondary_text),
            (live_id!(footer_version), section_text),
            (live_id!(footer_branding), section_text),
        ];
        for &(id, col) in labels {
            let w = self.ui.widget(cx, &[id]);
            set_widget_draw_uniform(cx, w.area(), live_id!(color), &v4(col));
            w.redraw(cx);
        }

        // -- theme buttons (bg only; text color stays fixed) --
        let theme_btns = [
            ("OpenAI", live_id!(btn_openai)),
            ("Airbnb", live_id!(btn_airbnb)),
            ("Notion", live_id!(btn_notion)),
        ];
        for (name, id) in theme_btns {
            let bg = if name == active_theme_base { active_btn_bg } else { inactive_btn_bg };
            let w = self.ui.widget(cx, &[id]);
            set_widget_draw_uniform(cx, w.area(), live_id!(color), &v4(bg));
            w.redraw(cx);
        }

        // -- mode toggle container --
        let tw = self.ui.widget(cx, &[live_id!(mode_toggle_bg)]);
        set_widget_draw_uniform(cx, tw.area(), live_id!(color), &v4(toggle_container));
        tw.redraw(cx);

        // -- mode toggle buttons (bg only) --
        let (light_bg, dark_bg) = if is_dark {
            (toggle_container, toggle_active)
        } else {
            (toggle_active, toggle_container)
        };
        let lw = self.ui.widget(cx, &[live_id!(btn_light)]);
        set_widget_draw_uniform(cx, lw.area(), live_id!(color), &v4(light_bg));
        lw.redraw(cx);
        let dw = self.ui.widget(cx, &[live_id!(btn_dark)]);
        set_widget_draw_uniform(cx, dw.area(), live_id!(color), &v4(dark_bg));
        dw.redraw(cx);

        // -- nav buttons (bg only) --
        let nav_on = [
            live_id!(nav_on_buttons), live_id!(nav_on_inputs), live_id!(nav_on_display),
            live_id!(nav_on_feedback), live_id!(nav_on_layout),
        ];
        let nav_off = [
            live_id!(nav_off_buttons), live_id!(nav_off_inputs), live_id!(nav_off_display),
            live_id!(nav_off_feedback), live_id!(nav_off_layout),
        ];
        for id in nav_on {
            let w = self.ui.widget(cx, &[id]);
            set_widget_draw_uniform(cx, w.area(), live_id!(color), &v4(active_btn_bg));
            w.redraw(cx);
        }
        for id in nav_off {
            let w = self.ui.widget(cx, &[id]);
            set_widget_draw_uniform(cx, w.area(), live_id!(color), &v4(inactive_btn_bg));
            w.redraw(cx);
        }
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
        if self.ui.button(cx, ids!(btn_light)).clicked(actions) {
            let current = ThemeEngine::current();
            let base = current.name.replace(" Dark", "");
            self.switch_theme(cx, &base);
        }
        if self.ui.button(cx, ids!(btn_dark)).clicked(actions) {
            let current = ThemeEngine::current();
            let base = current.name.replace(" Dark", "");
            let dark_name = format!("{} Dark", base);
            self.switch_theme(cx, &dark_name);
        }

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
