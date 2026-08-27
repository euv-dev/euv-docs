use super::*;

/// Props of the [`docs_toc`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsTocProps {
    /// The current route signal.
    pub(crate) route_signal: Signal<String>,
}
