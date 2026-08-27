use super::*;

/// Renders the 404 page for unknown routes.
///
/// # Arguments
///
/// - `DocsPageProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The 404 virtual DOM tree.
#[component]
pub(crate) fn docs_not_found(node: VirtualNode<DocsPageProps>) -> VirtualNode {
    let DocsPageProps { route_signal }: DocsPageProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    html! {
        div {
            class: c_docs_content()
            div {
                class: c_docs_hero()
                h1 {
                    class: c_docs_hero_title()
                    "404"
                }
                p {
                    class: c_docs_hero_tagline()
                    {
                        format!("Page not found: {path}")
                    }
                }
                div {
                    class: c_docs_hero_actions()
                    a {
                        class: c_docs_hero_button_primary()
                        href: format!("#{}", locale.prefix)
                        onclick: Router::link_handler(locale.prefix)
                        "Home"
                    }
                }
            }
        }
    }
}
