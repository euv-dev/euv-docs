use super::*;

/// Renders the 404 page for unknown routes, using the global euv hero
/// classes (`c_home` / `c_home_title` / `c_home_subtitle` / `c_home_actions`).
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
            class: c_page_container()
            div {
                class: c_home()
                div {
                    class: c_page_glow()
                }
                div {
                    class: c_home_content()
                    h1 {
                        class: c_home_title()
                        "404"
                    }
                    p {
                        class: c_home_subtitle()
                        {
                            format!("Page not found: {path}")
                        }
                    }
                    div {
                        class: c_home_actions()
                        a {
                            class: c_home_btn_primary()
                            href: format!("#{}", locale.prefix)
                            onclick: Router::link_handler(locale.prefix)
                            "Home"
                        }
                    }
                }
            }
        }
    }
}
