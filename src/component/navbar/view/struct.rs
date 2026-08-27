use super::*;

/// Props of the [`docs_navbar`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsNavbarProps {
    /// The current route signal.
    pub(crate) route_signal: Signal<String>,
    /// The current theme name signal.
    pub(crate) theme_signal: Signal<String>,
    /// Mobile drawer open state.
    pub(crate) drawer_open: Signal<bool>,
    /// Locale dropdown open state.
    pub(crate) locale_menu_open: Signal<bool>,
}
