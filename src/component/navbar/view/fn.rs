use super::*;

/// The dark theme name used by euv-ui theme hooks.
const THEME_DARK: &str = "dark";

/// Renders the fixed top navbar: brand, locale nav links, language dropdown,
/// theme toggle, and the mobile hamburger button.
///
/// # Arguments
///
/// - `DocsNavbarProps` - The typed props containing the shell signals.
///
/// # Returns
///
/// - `VirtualNode` - The navbar virtual DOM tree.
#[component]
pub(crate) fn docs_navbar(node: VirtualNode<DocsNavbarProps>) -> VirtualNode {
    let DocsNavbarProps {
        route_signal,
        theme_signal,
        drawer_open,
        locale_menu_open,
    }: DocsNavbarProps = node.try_get_props().unwrap_or_default();

    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    let site: &DocsSite = &crate::generated::SITE;
    let brand_title: &str = if locale.title.is_empty() {
        site.title
    } else {
        locale.title
    };

    html! {
        nav {
            class: c_docs_navbar()
            button {
                class: c_docs_mobile_menu_button()
                onclick: toggle_drawer(drawer_open)
                "≡"
            }
            a {
                class: c_docs_navbar_brand()
                href: format!("#{}", locale.prefix)
                onclick: Router::link_handler(locale.prefix)
                span {
                    class: c_docs_navbar_logo()
                    {
                        site.logo
                    }
                }
                span {
                    {
                        brand_title
                    }
                }
            }
            div {
                class: c_docs_navbar_links()
                for item in locale.navbar.iter() {
                    docs_navbar_link {
                        route_signal
                        item: *item
                    }
                }
            }
            div {
                class: c_docs_navbar_actions()
                if { site.locales.len() > 1 } {
                    div {
                        class: c_docs_locale_menu()
                        button {
                            class: c_docs_icon_button()
                            title: "Language"
                            onclick: toggle_menu(locale_menu_open)
                            "🌐"
                        }
                        if { locale_menu_open } {
                            div {
                                class: c_docs_locale_dropdown()
                                for target in site.locales.iter() {
                                    button {
                                        class: c_docs_locale_item()
                                        key: target.prefix
                                        onclick: switch_locale(route_signal, locale_menu_open, target)
                                        {
                                            target.label
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                button {
                    class: c_docs_icon_button()
                    title: "Toggle theme"
                    onclick: ThemeState::toggle(theme_signal)
                    if { theme_signal.get() == THEME_DARK } {
                        "☀"
                    } else {
                        "☾"
                    }
                }
            }
        }
    }
}

/// Toggles the mobile drawer.
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer-open signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn toggle_drawer(drawer_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| drawer_open.set(!drawer_open.get())))
}

/// Toggles the locale dropdown menu.
///
/// # Arguments
///
/// - `Signal<bool>` - The menu-open signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn toggle_menu(menu_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| menu_open.set(!menu_open.get())))
}

/// Switches the current route into another locale.
///
/// # Arguments
///
/// - `Signal<String>` - The route signal.
/// - `Signal<bool>` - The menu-open signal (closed after switching).
/// - `&'static DocsLocale` - The target locale.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn switch_locale(
    route_signal: Signal<String>,
    menu_open: Signal<bool>,
    target: &'static DocsLocale,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| {
        let (path, _anchor) = parse_route(&route_signal.get());
        menu_open.set(false);
        Router::navigate(route_in_locale(&path, target));
    }))
}

/// Props of the [`docs_navbar_link`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsNavbarLinkProps {
    /// The current route signal.
    pub(crate) route_signal: Signal<String>,
    /// The navbar item to render.
    pub(crate) item: DocsNavItem,
}

/// Renders a single navbar link (internal hash route or external URL).
///
/// # Arguments
///
/// - `DocsNavbarLinkProps` - The typed props.
///
/// # Returns
///
/// - `VirtualNode` - The link virtual DOM tree.
#[component]
pub(crate) fn docs_navbar_link(node: VirtualNode<DocsNavbarLinkProps>) -> VirtualNode {
    let DocsNavbarLinkProps { route_signal, item }: DocsNavbarLinkProps =
        node.try_get_props().unwrap_or_default();
    let external: bool = item.link.starts_with("http");
    let (path, _anchor) = parse_route(&route_signal.get());
    let active: bool = !external && item.link != "/" && path.starts_with(item.link);
    let link_class: fn() -> &'static Css = if active {
        c_docs_navbar_link_active
    } else {
        c_docs_navbar_link
    };
    if external {
        html! {
            a {
                class: c_docs_navbar_link()
                href: item.link
                target: "_blank"
                onclick: Router::external_link_handler(item.link)
                {
                    item.text
                }
            }
        }
    } else {
        html! {
            a {
                class: {
                    link_class()
                }
                href: format!("#{}", item.link)
                onclick: Router::link_handler(item.link)
                {
                    item.text
                }
            }
        }
    }
}
