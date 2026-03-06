//! Initialize OxideUI project scaffold

use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct InitArgs {
    /// Target directory (default: current directory)
    #[arg(short, long, default_value = ".")]
    pub path: String,
}

pub fn run(args: InitArgs) -> Result<(), Box<dyn std::error::Error>> {
    let base = Path::new(&args.path);

    let figma_dir = base.join("figma");
    fs::create_dir_all(&figma_dir)?;

    let tokens = serde_json::json!({
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
    });
    fs::write(
        figma_dir.join("tokens.json"),
        serde_json::to_string_pretty(&tokens)?,
    )?;

    let src_dir = base.join("src");
    fs::create_dir_all(&src_dir)?;

    let cargo_path = base.join("Cargo.toml");
    if !cargo_path.exists() {
        let cargo = r#"[package]
name = "my-oxide-app"
version = "0.1.0"
edition = "2021"

[dependencies]
oxide-widgets = { git = "https://github.com/oxide-ui/oxide-ui", branch = "main" }
oxide-core = { git = "https://github.com/oxide-ui/oxide-ui", branch = "main" }
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "dev" }
"#;
        fs::write(cargo_path, cargo)?;
    }

    let lib_path = src_dir.join("lib.rs");
    if !lib_path.exists() {
        fs::write(
            lib_path,
            "pub use makepad_widgets;\npub use oxide_widgets;\npub mod app;\n",
        )?;
    }

    let app_path = src_dir.join("app.rs");
    if !app_path.exists() {
        fs::write(
            app_path,
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
"#,
        )?;
    }

    let main_path = src_dir.join("main.rs");
    if !main_path.exists() {
        fs::write(
            main_path,
            "fn main() {\n    my_oxide_app::app::app_main()\n}\n",
        )?;
    }

    println!("OxideUI initialized in {}", base.display());
    println!("  - figma/tokens.json created with a default theme");
    println!("  - src/main.rs, src/lib.rs, src/app.rs created");
    println!();
    println!("  cargo run                # run the app");
    println!("  oxide validate           # verify tokens");
    println!("  oxide sync               # pull from Figma (after setting token)");

    Ok(())
}
