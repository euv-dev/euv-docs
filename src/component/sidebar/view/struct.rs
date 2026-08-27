use super::*;

/// Props of the [`docs_sidebar_tree`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsSidebarTreeProps {
    /// The current route signal.
    pub(crate) route_signal: Signal<String>,
    /// Collapsed group keys.
    pub(crate) collapsed: Signal<Vec<String>>,
    /// The sidebar slice to render (defaults to the current locale root).
    pub(crate) items: Option<&'static [DocsSidebarItem]>,
    /// Key prefix for nested groups.
    pub(crate) prefix: String,
}
