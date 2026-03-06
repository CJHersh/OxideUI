use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxButtonGroup = mod.widgets.View{
        width: Fit height: Fit
        flow: Right spacing: 0
    }
}
