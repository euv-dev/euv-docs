use super::*;

/// Renders the root application shell.
///
/// Layout: fixed top navbar, fixed left sidebar (desktop), scrollable
/// content column with right anchor TOC, and a footer.
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
    use_anchor_scroll(route_signal);
    html! {
        div {
            class: root_class_signal
            class: c_docs_root()
            docs_navbar {
                route_signal
                theme_signal
                drawer_open
                locale_menu_open
            }
            div {
                class: c_docs_body()
                aside {
                    class: c_docs_sidebar()
                    docs_sidebar_tree {
                        route_signal
                        collapsed
                    }
                }
                main {
                    class: c_docs_main()
                    docs_main {
                        route_signal
                    }
                }
            }
            if { drawer_open } {
                div {
                    class: c_docs_mobile_overlay()
                    onclick: close_drawer(drawer_open)
                }
                div {
                    class: c_docs_mobile_drawer()
                    class: c_docs_mobile_drawer_open()
                    docs_sidebar_tree {
                        route_signal
                        collapsed
                    }
                }
            }
        }
    }
}

/// Closes the mobile navigation drawer.
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer-open signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_drawer(drawer_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| drawer_open.set(false)))
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
