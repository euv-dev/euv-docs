use super::*;

/// Renders the right-side anchor TOC (`On this page`) for the current page.
///
/// Links carry the full `#<route>#<anchor>` hash so the browser drives the
/// navigation and the layout's scroll hook performs the anchor jump.
///
/// # Arguments
///
/// - `DocsTocProps` - The typed props.
///
/// # Returns
///
/// - `VirtualNode` - The TOC virtual DOM tree (empty when the page has no
///   h2/h3 headings).
#[component]
pub(crate) fn docs_toc(node: VirtualNode<DocsTocProps>) -> VirtualNode {
    let DocsTocProps { route_signal }: DocsTocProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    let Some(page) = find_page(&path) else {
        return html! {
            ""
        };
    };
    if page.headings.is_empty() {
        return html! {
            ""
        };
    }
    html! {
        div {
            class: c_docs_toc()
            div {
                class: c_docs_toc_sticky()
                div {
                    class: c_docs_toc_title()
                    {
                        locale.toc_label
                    }
                }
                for heading in page.headings.iter() {
                    a {
                        key: heading.id
                        class: if { heading.level == 3u8 } {
                            c_docs_toc_link_h3()
                        } else {
                            c_docs_toc_link()
                        }
                        href: format!("#{}#{}", page.route, heading.id)
                        {
                            heading.text
                        }
                    }
                }
            }
        }
    }
}
