use super::*;

/// Routes the current path to the home page, a doc page, or the 404 page.
///
/// # Arguments
///
/// - `DocsPageProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The matched page virtual DOM tree.
#[component]
pub(crate) fn docs_main(node: VirtualNode<DocsPageProps>) -> VirtualNode {
    let DocsPageProps { route_signal }: DocsPageProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    match find_page(&path) {
        Some(page) if page.home => html! {
            docs_home_page {
                route_signal
            }
        },
        Some(_) => html! {
            docs_doc_page {
                route_signal
            }
        },
        None => html! {
            docs_not_found {
                route_signal
            }
        },
    }
}

/// Renders one documentation page: markdown body, prev/next links, footer,
/// and the right anchor TOC.
///
/// # Arguments
///
/// - `DocsPageProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The doc page virtual DOM tree.
#[component]
pub(crate) fn docs_doc_page(node: VirtualNode<DocsPageProps>) -> VirtualNode {
    let DocsPageProps { route_signal }: DocsPageProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    let Some(page) = find_page(&path) else {
        return html! {
            ""
        };
    };

    let (prev, next) = prev_next(locale, page.route);
    let footer_text: &str = if page.footer.is_empty() {
        locale.footer
    } else {
        page.footer
    };
    let prev_node: VirtualNode = match prev {
        Some(item) => {
            let link: &str = item.link.unwrap_or_default();
            html! {
                a {
                    class: c_docs_prev_next_link()
                    href: format!("#{link}")
                    onclick: Router::link_handler(link)
                    span {
                        class: c_docs_prev_next_label()
                        {
                            locale.prev_label
                        }
                    }
                    span {
                        class: c_docs_prev_next_text()
                        {
                            item.text
                        }
                    }
                }
            }
        }
        None => html! {
            div {
                class: c_docs_prev_next_spacer()
            }
        },
    };
    let next_node: VirtualNode = match next {
        Some(item) => {
            let link: &str = item.link.unwrap_or_default();
            html! {
                a {
                    class: c_docs_prev_next_link()
                    class: c_docs_prev_next_next()
                    href: format!("#{link}")
                    onclick: Router::link_handler(link)
                    span {
                        class: c_docs_prev_next_label()
                        {
                            locale.next_label
                        }
                    }
                    span {
                        class: c_docs_prev_next_text()
                        {
                            item.text
                        }
                    }
                }
            }
        }
        None => html! {
            div {
                class: c_docs_prev_next_spacer()
            }
        },
    };

    html! {
        div {
            class: c_docs_main_inner()
            div {
                class: c_docs_content()
                article {
                    class: c_docs_md_body()
                    class: "md-body"
                    {
                        render_md_blocks(page.blocks)
                    }
                }
                div {
                    class: c_docs_prev_next()
                    prev_node
                    next_node
                }
                if { !footer_text.is_empty() } {
                    footer {
                        class: c_docs_footer()
                        {
                            footer_text
                        }
                    }
                }
            }
            docs_toc {
                route_signal
            }
        }
    }
}

/// Computes the prev/next sidebar leaf links around the current route.
///
/// # Arguments
///
/// - `&'static DocsLocale` - The current locale.
/// - `&str` - The current page route.
///
/// # Returns
///
/// - `(Option<&DocsSidebarItem>, Option<&DocsSidebarItem>)` - Prev and next.
fn prev_next(
    locale: &'static DocsLocale,
    route: &str,
) -> (
    Option<&'static DocsSidebarItem>,
    Option<&'static DocsSidebarItem>,
) {
    let links: Vec<&'static DocsSidebarItem> = flat_sidebar_links(locale.sidebar);
    let Some(index) = links.iter().position(|item| item.link == Some(route)) else {
        return (None, None);
    };
    let prev: Option<&'static DocsSidebarItem> = index.checked_sub(1).map(|i| links[i]);
    let next: Option<&'static DocsSidebarItem> = links.get(index + 1).copied();
    (prev, next)
}
