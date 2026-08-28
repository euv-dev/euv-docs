use super::*;

/// Renders the root application shell.
///
/// All hooks live here; the returned tree is a single reactive `if` on the
/// current route (reading `route_signal` in the condition), which creates
/// the root dynamic node — every nested signal read subscribes to it, so
/// route / theme / drawer changes re-render the shell. This mirrors the
/// official euv example's `if { mobile_signal } { ... } else { ... }` root.
///
/// # Returns
///
/// - `VirtualNode` - The root virtual DOM tree.
pub(crate) fn app() -> VirtualNode {
    let route_signal: Signal<String> = App::use_signal(Router::current_route);
    let drawer_open: Signal<bool> = App::use_signal(|| false);
    let locale_menu_open: Signal<bool> = App::use_signal(|| false);
    let collapsed: Signal<Vec<String>> = App::use_signal(Vec::new);
    let mobile_signal: Signal<bool> = UseEuvLayout::use_resize();
    let theme_state: ThemeState = ThemeState::use_theme_state(mobile_signal);
    let theme_signal: Signal<String> = theme_state.get_theme();
    let root_class_signal: Signal<String> = theme_state.get_root_class();
    Router::use_hash_change(route_signal);
    Router::use_overlay_history(drawer_open, mobile_signal);
    use_anchor_scroll(route_signal);
    html! {
        if { route_is_home(&route_signal.get()) } {
            docs_shell {
                route_signal
                theme_signal
                root_class_signal
                drawer_open
                locale_menu_open
                collapsed
                is_home: true
            }
        } else {
            docs_shell {
                route_signal
                theme_signal
                root_class_signal
                drawer_open
                locale_menu_open
                collapsed
                is_home: false
            }
        }
    }
}

/// Returns whether the given route belongs to a home page.
///
/// # Arguments
///
/// - `&str` - The raw route string (may carry an `#anchor` suffix).
///
/// # Returns
///
/// - `bool` - True when the route resolves to a page with `home: true`.
fn route_is_home(route: &str) -> bool {
    let (path, _anchor) = parse_route(route);
    find_page(&path)
        .map(|page: &DocsPage| page.home)
        .unwrap_or(false)
}

/// Renders the themed shell out of euv-ui components: `euv_navbar`,
/// `euv_sidebar` (doc pages), the main column and the mobile `euv_drawer`.
///
/// # Arguments
///
/// - `DocsShellProps` - The typed props containing the shell signals.
///
/// # Returns
///
/// - `VirtualNode` - The shell virtual DOM tree.
#[component]
pub(crate) fn docs_shell(node: VirtualNode<DocsShellProps>) -> VirtualNode {
    let DocsShellProps {
        route_signal,
        theme_signal,
        root_class_signal,
        drawer_open,
        locale_menu_open,
        collapsed,
        is_home,
    }: DocsShellProps = node.try_get_props().unwrap_or_default();
    // NOTE: the aside condition must read the signal directly — a reactive
    // `if` whose condition captures a plain `bool` prop keeps the stale
    // closure from the first render and never updates.
    let main_class: fn() -> &'static Css = if is_home {
        c_docs_main_home
    } else {
        c_docs_main
    };
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    let site: &DocsSite = &crate::generated::SITE;
    let brand_title: &str = if locale.title.is_empty() {
        site.title
    } else {
        locale.title
    };
    let locale_menu: VirtualNode = locale_menu_node(route_signal, locale_menu_open);
    html! {
        div {
            class: root_class_signal
            class: c_docs_root()
            euv_navbar {
                route_signal
                brand_logo: site.logo
                brand_title: brand_title
                brand_href: locale.prefix
                items: locale.navbar
                drawer_open: Some(drawer_open)
                locale_menu
                button {
                    class: c_euv_navbar_icon_button()
                    title: "Toggle theme"
                    onclick: ThemeState::toggle(theme_signal)
                    if { theme_signal.get() == THEME_DARK } {
                        "☀"
                    } else {
                        "☾"
                    }
                }
            }
            div {
                class: c_docs_body()
                if { !route_is_home(&route_signal.get()) } {
                    aside {
                        class: c_docs_sidebar()
                        euv_sidebar {
                            route_signal
                            collapsed
                            items: locale.sidebar
                        }
                    }
                }
                main {
                    class: main_class()
                    docs_main {
                        route_signal
                    }
                }
            }
            euv_drawer {
                open: drawer_open
                euv_sidebar {
                    route_signal
                    collapsed
                    items: locale.sidebar
                    on_navigate: drawer_navigate()
                }
            }
        }
    }
}

/// Builds the locale switcher dropdown (empty when the site has one locale).
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
/// - `Signal<bool>` - The dropdown open signal.
///
/// # Returns
///
/// - `VirtualNode` - The locale menu virtual DOM tree.
fn locale_menu_node(route_signal: Signal<String>, locale_menu_open: Signal<bool>) -> VirtualNode {
    let site: &DocsSite = &crate::generated::SITE;
    if site.locales.len() <= 1 {
        return html! {
            ""
        };
    }
    let items: Vec<EuvDropdownItem> = site
        .locales
        .iter()
        .map(|target: &DocsLocale| EuvDropdownItem {
            label: target.label,
            value: target.prefix,
        })
        .collect();
    html! {
        euv_dropdown {
            open: locale_menu_open
            items: items
            on_select: switch_locale(route_signal, locale_menu_open)
            button {
                class: c_euv_navbar_icon_button()
                title: "Language"
                onclick: toggle_menu(locale_menu_open)
                "🌐"
            }
        }
    }
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

/// Switches the current route into the locale chosen from the dropdown.
///
/// # Arguments
///
/// - `Signal<String>` - The route signal.
/// - `Signal<bool>` - The menu-open signal (closed after switching).
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select handler receiving the
///   target locale prefix.
fn switch_locale(
    route_signal: Signal<String>,
    menu_open: Signal<bool>,
) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |prefix: &'static str| {
        let site: &DocsSite = &crate::generated::SITE;
        let Some(target) = site.locales.iter().find(|locale| locale.prefix == prefix) else {
            return;
        };
        let (path, _anchor) = parse_route(&route_signal.get());
        menu_open.set(false);
        Router::navigate(route_in_locale(&path, target));
    }))
}

/// Drawer navigation handler: consumes the drawer's overlay history entry
/// via `Router::overlay_back` and navigates after the popstate, so the
/// browser history stays consistent (mirrors the euv example's mobile nav).
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The navigation interceptor.
fn drawer_navigate() -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(|link: &'static str| {
        Router::overlay_back(Some(link.to_string()));
    }))
}

/// Scrolls to the in-page anchor after route changes (or to top).
///
/// Subscribes to the route signal; scrolling is deferred with a timeout so
/// it runs after the reactive DOM update. Also runs once on startup so
/// deep links with an anchor work on first load.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
fn use_anchor_scroll(route_signal: Signal<String>) {
    let schedule = move || {
        let raw: String = route_signal.get();
        let (_path, anchor) = parse_route(&raw);
        schedule_scroll(anchor);
    };
    route_signal.subscribe({
        let schedule = schedule.clone();
        move || schedule()
    });
    schedule();
}

/// Defers a scroll until after the reactive re-render.
///
/// # Arguments
///
/// - `Option<String>` - The anchor slug; `None` scrolls to the page top.
fn schedule_scroll(anchor: Option<String>) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let callback: Closure<dyn FnMut()> = Closure::once(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let scrolled: bool = anchor
            .and_then(|id| window.document().and_then(|doc| doc.get_element_by_id(&id)))
            .map(|element| {
                element.scroll_into_view();
            })
            .is_some();
        if !scrolled {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
    });
    let _: Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        100,
    );
    callback.forget();
}
