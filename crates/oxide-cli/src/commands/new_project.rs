//! Create new project from template

use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct NewProjectArgs {
    /// Project name (directory and crate name)
    #[arg(value_name = "NAME")]
    pub name: String,

    /// Template: basic, dashboard, form
    #[arg(short, long, default_value = "basic")]
    pub template: String,

    /// Target directory (default: current)
    #[arg(short, long, default_value = ".")]
    pub path: String,
}

pub fn run(args: NewProjectArgs) -> Result<(), Box<dyn std::error::Error>> {
    let base = Path::new(&args.path).join(&args.name);
    fs::create_dir_all(&base)?;

    let crate_name = args.name.replace('-', "_");

    let cargo = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
oxide-widgets = {{ git = "https://github.com/CJHersh/OxideUI", branch = "main" }}
oxide-core = {{ git = "https://github.com/CJHersh/OxideUI", branch = "main" }}
makepad-widgets = {{ git = "https://github.com/makepad/makepad", rev = "8b515338a2f50c5e0e2742cdc8b8ee7278aff371" }}
"#,
        crate_name
    );
    fs::write(base.join("Cargo.toml"), cargo)?;

    let src_dir = base.join("src");
    fs::create_dir_all(&src_dir)?;

    let main_rs = format!("fn main() {{\n    {}::app::app_main()\n}}\n", crate_name);
    fs::write(src_dir.join("main.rs"), main_rs)?;

    fs::write(
        src_dir.join("lib.rs"),
        "pub use makepad_widgets;\npub use oxide_widgets;\npub mod app;\n",
    )?;

    let app_content = match args.template.as_str() {
        "dashboard" => include_dashboard_app(),
        "form" => include_form_app(),
        _ => include_basic_app(),
    };
    fs::write(src_dir.join("app.rs"), app_content)?;

    let figma_dir = base.join("figma");
    fs::create_dir_all(&figma_dir)?;
    fs::write(
        figma_dir.join("tokens.json"),
        serde_json::to_string_pretty(&default_tokens())?,
    )?;

    println!("Created project '{}' at {}", args.name, base.display());
    println!();
    println!("  cd {}", args.name);
    println!("  cargo run");
    Ok(())
}

fn default_tokens() -> serde_json::Value {
    serde_json::json!({
        "version": "1.0",
        "themes": {
            "default": {
                "name": "Default",
                "colors": {
                    "surface_primary": "#FFFFFF",
                    "surface_secondary": "#F7F7F8",
                    "surface_tertiary": "#EAEBEE",
                    "surface_inverse": "#202023",
                    "text_primary": "#202023",
                    "text_secondary": "#676A70",
                    "text_tertiary": "#8E8EA0",
                    "text_disabled": "#ACACBE",
                    "text_inverse": "#FFFFFF",
                    "text_link": "#10A37F",
                    "interactive_default": "#10A37F",
                    "interactive_hover": "#0D8A6A",
                    "interactive_pressed": "#0B7359",
                    "interactive_disabled": "#C5C5D1",
                    "border_default": "#EAEBEE",
                    "border_hover": "#D1D3D8",
                    "border_focus": "#10A37F",
                    "border_error": "#E53935",
                    "feedback_success": "#10A37F",
                    "feedback_warning": "#F59E0B",
                    "feedback_error": "#E53935",
                    "feedback_info": "#3B82F6"
                },
                "spacing": {
                    "none": 0, "xs": 4, "sm": 8, "md": 16, "lg": 24, "xl": 32, "xxl": 48
                },
                "radius": {
                    "none": 0, "sm": 4, "md": 8, "lg": 12, "xl": 16, "full": 9999
                },
                "typography": {
                    "font_family": "Inter",
                    "font_size_xs": 12, "font_size_sm": 14, "font_size_md": 16,
                    "font_size_lg": 18, "font_size_xl": 20, "font_size_xxl": 24
                }
            }
        }
    })
}

fn include_basic_app() -> &'static str {
    r#"use makepad_widgets::*;
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::themes::all_themes;
use oxide_widgets::{buttons, display, layout};
use oxide_widgets::ThemeMap;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "My OxideUI App"
                body +: {
                    View{
                        width: Fill height: Fill
                        flow: Down
                        padding: 32.
                        spacing: 24.
                        align.x: 0.5

                        title := OxLabelTitle{ text: "Hello OxideUI" }
                        subtitle := OxLabelSecondary{ text: "Build once, theme everywhere" }

                        card := OxCard{
                            width: 400
                            spacing: 16
                            card_label := OxLabelBody{ text: "Click the button to cycle themes" }
                            theme_btn := OxButton{ text: "Switch Theme" }
                        }

                        status := OxLabelCaption{ text: "Active: OpenAI" }
                    }
                }
            }
        }
    }
    app
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        crate::oxide_widgets::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }

    fn switch_theme(&self, cx: &mut Cx) {
        ThemeEngine::next_theme();
        let theme = ThemeEngine::current();

        ThemeMap::builder(&self.ui)
            .add(live_id!(title), display::apply_label_title_theme)
            .add(live_id!(subtitle), display::apply_label_secondary_theme)
            .add(live_id!(card), layout::apply_card_theme)
            .add(live_id!(card_label), display::apply_label_body_theme)
            .add(live_id!(theme_btn), buttons::apply_button_theme)
            .add(live_id!(status), display::apply_label_caption_theme)
            .build()
            .apply_all(cx, &theme);

        self.ui.label(cx, ids!(status))
            .set_text(cx, &format!("Active: {}", theme.name));
        self.ui.redraw(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        ThemeEngine::init(all_themes());
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(theme_btn)).clicked(actions) {
            self.switch_theme(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
"#
}

fn include_dashboard_app() -> &'static str {
    r#"use makepad_widgets::*;
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::themes::all_themes;
use oxide_widgets::{buttons, display, layout};
use oxide_widgets::ThemeMap;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "OxideUI Dashboard"
                window.inner_size: vec2(900, 600)
                body +: {
                    View{
                        width: Fill height: Fill
                        flow: Down
                        padding: 32.
                        spacing: 24.

                        View{
                            width: Fill height: Fit
                            flow: Right
                            align.y: 0.5
                            title := OxLabelTitle{ text: "Dashboard" }
                            View{ width: Fill height: Fit }
                            theme_btn := OxButtonSecondary{ text: "Switch Theme" }
                        }

                        OxDivider{}

                        View{
                            width: Fill height: Fit
                            flow: Right spacing: 16.

                            stat1 := OxCard{
                                width: Fill
                                stat1_title := OxLabelCaption{ text: "Total Users" }
                                stat1_value := OxLabelTitle{ text: "1,234" }
                            }
                            stat2 := OxCard{
                                width: Fill
                                stat2_title := OxLabelCaption{ text: "Revenue" }
                                stat2_value := OxLabelTitle{ text: "$56,789" }
                            }
                            stat3 := OxCard{
                                width: Fill
                                stat3_title := OxLabelCaption{ text: "Growth" }
                                stat3_value := OxLabelTitle{ text: "+12.3%" }
                            }
                        }

                        main_card := OxCard{
                            spacing: 12
                            main_label := OxLabelSubtitle{ text: "Recent Activity" }
                            main_body := OxLabelBody{ text: "No recent activity to display." }
                            action_btn := OxButton{ text: "Refresh" }
                        }

                        status := OxLabelCaption{ text: "Active: OpenAI" }
                    }
                }
            }
        }
    }
    app
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        crate::oxide_widgets::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }

    fn switch_theme(&self, cx: &mut Cx) {
        ThemeEngine::next_theme();
        let theme = ThemeEngine::current();

        ThemeMap::builder(&self.ui)
            .add(live_id!(title), display::apply_label_title_theme)
            .add(live_id!(theme_btn), buttons::apply_button_secondary_theme)
            .add(live_id!(stat1), layout::apply_card_theme)
            .add(live_id!(stat1_title), display::apply_label_caption_theme)
            .add(live_id!(stat1_value), display::apply_label_title_theme)
            .add(live_id!(stat2), layout::apply_card_theme)
            .add(live_id!(stat2_title), display::apply_label_caption_theme)
            .add(live_id!(stat2_value), display::apply_label_title_theme)
            .add(live_id!(stat3), layout::apply_card_theme)
            .add(live_id!(stat3_title), display::apply_label_caption_theme)
            .add(live_id!(stat3_value), display::apply_label_title_theme)
            .add(live_id!(main_card), layout::apply_card_theme)
            .add(live_id!(main_label), display::apply_label_subtitle_theme)
            .add(live_id!(main_body), display::apply_label_body_theme)
            .add(live_id!(action_btn), buttons::apply_button_theme)
            .add(live_id!(status), display::apply_label_caption_theme)
            .build()
            .apply_all(cx, &theme);

        self.ui.label(cx, ids!(status))
            .set_text(cx, &format!("Active: {}", theme.name));
        self.ui.redraw(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        ThemeEngine::init(all_themes());
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(theme_btn)).clicked(actions) {
            self.switch_theme(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
"#
}

fn include_form_app() -> &'static str {
    r#"use makepad_widgets::*;
use oxide_core::theme::engine::ThemeEngine;
use oxide_core::theme::themes::all_themes;
use oxide_widgets::{buttons, display, inputs, layout};
use oxide_widgets::ThemeMap;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let app = startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "OxideUI Form"
                body +: {
                    View{
                        width: Fill height: Fill
                        flow: Down
                        padding: 32.
                        spacing: 24.
                        align.x: 0.5

                        title := OxLabelTitle{ text: "Contact Form" }

                        form_card := OxCard{
                            width: 480
                            spacing: 16

                            name_label := OxLabelBody{ text: "Name" }
                            name_input := OxTextInput{ empty_text: "Enter your name" }

                            email_label := OxLabelBody{ text: "Email" }
                            email_input := OxTextInput{ empty_text: "you@example.com" }

                            message_label := OxLabelBody{ text: "Message" }
                            message_input := OxTextArea{ empty_text: "Type your message..." }

                            View{
                                flow: Right spacing: 8
                                submit_btn := OxButton{ text: "Submit" }
                                cancel_btn := OxButtonSecondary{ text: "Cancel" }
                            }
                        }

                        View{
                            flow: Right spacing: 12
                            theme_btn := OxButtonGhost{ text: "Switch Theme" }
                            status := OxLabelCaption{ text: "Active: OpenAI" }
                        }
                    }
                }
            }
        }
    }
    app
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        crate::oxide_widgets::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }

    fn switch_theme(&self, cx: &mut Cx) {
        ThemeEngine::next_theme();
        let theme = ThemeEngine::current();

        ThemeMap::builder(&self.ui)
            .add(live_id!(title), display::apply_label_title_theme)
            .add(live_id!(form_card), layout::apply_card_theme)
            .add(live_id!(name_label), display::apply_label_body_theme)
            .add(live_id!(name_input), inputs::apply_text_input_theme)
            .add(live_id!(email_label), display::apply_label_body_theme)
            .add(live_id!(email_input), inputs::apply_text_input_theme)
            .add(live_id!(message_label), display::apply_label_body_theme)
            .add(live_id!(message_input), inputs::apply_text_area_theme)
            .add(live_id!(submit_btn), buttons::apply_button_theme)
            .add(live_id!(cancel_btn), buttons::apply_button_secondary_theme)
            .add(live_id!(theme_btn), buttons::apply_button_ghost_theme)
            .add(live_id!(status), display::apply_label_caption_theme)
            .build()
            .apply_all(cx, &theme);

        self.ui.label(cx, ids!(status))
            .set_text(cx, &format!("Active: {}", theme.name));
        self.ui.redraw(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        ThemeEngine::init(all_themes());
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(theme_btn)).clicked(actions) {
            self.switch_theme(cx);
        }
        if self.ui.button(cx, ids!(submit_btn)).clicked(actions) {
            self.ui.label(cx, ids!(status))
                .set_text(cx, "Submitted!");
            self.ui.redraw(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
"#
}
