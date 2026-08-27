use super::*;

/// Renders the home page in the euv hero style: radial glow, big title,
/// tagline, action buttons, and a borderless feature grid — reusing the
/// global `c_home_*` / `c_feature_*` classes from `euv-ui`.
///
/// # Arguments
///
/// - `DocsPageProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The home page virtual DOM tree.
#[component]
pub(crate) fn docs_home_page(node: VirtualNode<DocsPageProps>) -> VirtualNode {
    let DocsPageProps { route_signal }: DocsPageProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    let site: &DocsSite = &crate::generated::SITE;
    let Some(page) = find_page(&path) else {
        return html! {
            ""
        };
    };

    let hero_title: &str = if page.hero_text.is_empty() {
        if locale.title.is_empty() {
            site.title
        } else {
            locale.title
        }
    } else {
        page.hero_text
    };
    let footer_text: &str = if page.footer.is_empty() {
        locale.footer
    } else {
        page.footer
    };

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
                        {
                            hero_title
                        }
                    }
                    if { !page.tagline.is_empty() } {
                        p {
                            class: c_home_subtitle()
                            {
                                page.tagline
                            }
                        }
                    }
                    if { !page.actions.is_empty() } {
                        div {
                            class: c_home_actions()
                            for action in page.actions.iter() {
                                docs_home_action {
                                    action: *action
                                }
                            }
                        }
                    }
                }
            }
            if { !page.features.is_empty() } {
                div {
                    class: c_home_feature_grid()
                    for feature in page.features.iter() {
                        div {
                            class: c_feature_card()
                            key: feature.title
                            div {
                                class: c_feature_header()
                                if { !feature.icon.is_empty() } {
                                    span {
                                        class: c_feature_icon()
                                        {
                                            feature.icon
                                        }
                                    }
                                }
                                span {
                                    class: c_feature_name()
                                    {
                                        feature.title
                                    }
                                }
                            }
                            p {
                                class: c_feature_desc()
                                {
                                    feature.details
                                }
                            }
                        }
                    }
                }
            }
            if { !page.blocks.is_empty() } {
                article {
                    class: c_docs_md_body()
                    class: "md-body"
                    {
                        render_md_blocks(page.blocks)
                    }
                }
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
    }
}

/// Props of the [`docs_home_action`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsHomeActionProps {
    /// The hero action to render.
    pub(crate) action: DocsAction,
}

/// Renders one hero action button (internal route or external URL) with the
/// global `c_home_btn_primary` / `c_home_btn_secondary` classes.
///
/// # Arguments
///
/// - `DocsHomeActionProps` - The typed props.
///
/// # Returns
///
/// - `VirtualNode` - The action button virtual DOM tree.
#[component]
pub(crate) fn docs_home_action(node: VirtualNode<DocsHomeActionProps>) -> VirtualNode {
    let DocsHomeActionProps { action }: DocsHomeActionProps =
        node.try_get_props().unwrap_or_default();
    let external: bool = action.link.starts_with("http");
    let button_class: fn() -> &'static Css = if action.kind == "primary" {
        c_home_btn_primary
    } else {
        c_home_btn_secondary
    };
    if external {
        html! {
            a {
                class: {
                    button_class()
                }
                href: action.link
                target: "_blank"
                onclick: Router::external_link_handler(action.link)
                {
                    action.text
                }
            }
        }
    } else {
        html! {
            a {
                class: {
                    button_class()
                }
                href: format!("#{}", action.link)
                onclick: Router::link_handler(action.link)
                {
                    action.text
                }
            }
        }
    }
}
