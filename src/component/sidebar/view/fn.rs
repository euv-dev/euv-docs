use super::*;

/// Renders the sidebar tree for the current locale (recursive).
///
/// Groups are collapsible; the collapse state lives in the `collapsed`
/// signal so it survives re-renders. All groups start expanded.
///
/// # Arguments
///
/// - `DocsSidebarTreeProps` - The typed props.
///
/// # Returns
///
/// - `VirtualNode` - The sidebar virtual DOM tree.
#[component]
pub(crate) fn docs_sidebar_tree(node: VirtualNode<DocsSidebarTreeProps>) -> VirtualNode {
    let DocsSidebarTreeProps {
        route_signal,
        collapsed,
        items,
        prefix,
    }: DocsSidebarTreeProps = node.try_get_props().unwrap_or_default();

    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    let items: &'static [DocsSidebarItem] = items.unwrap_or(locale.sidebar);

    html! {
        div {
            for item in items.iter() {
                docs_sidebar_item {
                    route_signal
                    collapsed
                    item: *item
                    prefix: prefix.clone()
                }
            }
        }
    }
}

/// Props of the [`docs_sidebar_item`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsSidebarItemProps {
    /// The current route signal.
    pub(crate) route_signal: Signal<String>,
    /// Collapsed group keys.
    pub(crate) collapsed: Signal<Vec<String>>,
    /// The item to render.
    pub(crate) item: DocsSidebarItem,
    /// Key prefix of the parent group.
    pub(crate) prefix: String,
}

/// Renders one sidebar node: a leaf link or a collapsible group.
///
/// # Arguments
///
/// - `DocsSidebarItemProps` - The typed props.
///
/// # Returns
///
/// - `VirtualNode` - The item virtual DOM tree.
#[component]
pub(crate) fn docs_sidebar_item(node: VirtualNode<DocsSidebarItemProps>) -> VirtualNode {
    let DocsSidebarItemProps {
        route_signal,
        collapsed,
        item,
        prefix,
    }: DocsSidebarItemProps = node.try_get_props().unwrap_or_default();

    let (path, _anchor) = parse_route(&route_signal.get());

    if item.children.is_empty() {
        let Some(link) = item.link else {
            return html! {
                ""
            };
        };
        let active: bool = path == link;
        let link_class: fn() -> &'static Css = if active {
            c_docs_sidebar_link_active
        } else {
            c_docs_sidebar_link
        };
        return html! {
            a {
                class: {
                    link_class()
                }
                href: format!("#{link}")
                onclick: Router::link_handler(link)
                {
                    item.text
                }
            }
        };
    }

    let key: String = format!("{prefix}/{}", item.text);
    let open: bool = !collapsed.get().contains(&key);
    let arrow_class: fn() -> &'static Css = if open {
        c_docs_sidebar_group_arrow_open
    } else {
        c_docs_sidebar_group_arrow
    };
    let title_node: VirtualNode = match item.link {
        Some(link) => html! {
            a {
                href: format!("#{link}")
                onclick: nav_without_toggle(link)
                { item.text }
            }
        },
        None => html! {
            { item.text }
        },
    };

    html! {
        div {
            class: c_docs_sidebar_group()
            div {
                class: c_docs_sidebar_group_title()
                onclick: toggle_group(collapsed, key.clone())
                span {
                    title_node
                }
                span {
                    class: arrow_class()
                    "▸"
                }
            }
            if open {
                div {
                    class: c_docs_sidebar_children()
                    docs_sidebar_tree {
                        route_signal
                        collapsed
                        items: Some(item.children)
                        prefix: key.clone()
                    }
                }
            }
        }
    }
}

/// Navigates to a group index page without toggling the group.
///
/// Stops event propagation so the parent row's toggle handler does not
/// also fire when the link is clicked.
///
/// # Arguments
///
/// - `&'static str` - The target route.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn nav_without_toggle(link: &'static str) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        event.stop_propagation();
        Router::navigate(link);
    }))
}

/// Toggles a sidebar group's collapsed state.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The collapsed-keys signal.
/// - `String` - The group key.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn toggle_group(collapsed: Signal<Vec<String>>, key: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| {
        let mut keys: Vec<String> = collapsed.get();
        if let Some(index) = keys.iter().position(|k| k == &key) {
            keys.remove(index);
        } else {
            keys.push(key.clone());
        }
        collapsed.set(keys);
    }))
}
