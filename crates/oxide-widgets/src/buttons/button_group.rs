/// **Status: Planned** -- OxButtonGroup is a layout placeholder with no interactive behavior.
/// It currently renders as a horizontal `View` with zero spacing. Future versions will
/// support connected border radii, shared selection state, and keyboard navigation.
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.OxButtonGroup = mod.widgets.View{
        width: Fit height: Fit
        flow: Right spacing: 0
    }
}
