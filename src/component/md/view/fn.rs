use super::*;

/// Renders a markdown block slice into euv VirtualNodes.
///
/// Pure render functions (no components): the AST is `&'static` generated
/// data, so rendering is a direct recursive tree walk and the framework's
/// diffing applies to the produced virtual DOM.
///
/// # Arguments
///
/// - `&'static [DocsBlock]` - The blocks to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered block sequence.
pub(crate) fn render_md_blocks(blocks: &'static [DocsBlock]) -> VirtualNode {
    html! {
        for block in blocks.iter() {
            { render_md_block(block) }
        }
    }
}

/// Renders a single markdown block.
///
/// # Arguments
///
/// - `&'static DocsBlock` - The block to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered block.
fn render_md_block(block: &'static DocsBlock) -> VirtualNode {
    match block {
        DocsBlock::Heading {
            level,
            id,
            href,
            inline,
        } => render_md_heading(*level, id, href, inline),
        DocsBlock::Paragraph(inline) => {
            let content: VirtualNode = render_md_inlines(inline);
            html! {
                p {
                    content
                }
            }
        }
        DocsBlock::CodeBlock { lang, code } => html! {
            pre {
                class: format!("language-{lang}")
                code {
                    { *code }
                }
            }
        },
        DocsBlock::BlockQuote(blocks) => {
            let content: VirtualNode = render_md_blocks(blocks);
            html! {
                blockquote {
                    content
                }
            }
        }
        DocsBlock::List { ordered, items } => {
            if *ordered {
                html! {
                    ol {
                        for item in items.iter() {
                            li {
                                { render_md_blocks(item) }
                            }
                        }
                    }
                }
            } else {
                html! {
                    ul {
                        for item in items.iter() {
                            li {
                                { render_md_blocks(item) }
                            }
                        }
                    }
                }
            }
        }
        DocsBlock::Table { head, rows } => html! {
            table {
                thead {
                    tr {
                        for cell in head.iter() {
                            th {
                                { render_md_inlines(cell) }
                            }
                        }
                    }
                }
                tbody {
                    for row in rows.iter() {
                        tr {
                            for cell in row.iter() {
                                td {
                                    { render_md_inlines(cell) }
                                }
                            }
                        }
                    }
                }
            }
        },
        DocsBlock::Container {
            kind,
            title,
            blocks,
        } => {
            let content: VirtualNode = render_md_blocks(blocks);
            html! {
                div {
                    class: format!("docs-container {kind}")
                    p {
                        class: "docs-container-title"
                        { *title }
                    }
                    content
                }
            }
        }
        DocsBlock::Rule => html! {
            hr {}
        },
        DocsBlock::Html(raw) => html! {
            div {
                inner_html: *raw
            }
        },
    }
}

/// Renders one heading block with its permalink anchor.
///
/// # Arguments
///
/// - `u8` - The heading level (1–6).
/// - `&'static str` - The slug id.
/// - `&'static str` - The full `#<route>#<slug>` permalink.
/// - `&'static [DocsInline]` - The heading content.
///
/// # Returns
///
/// - `VirtualNode` - The rendered heading.
fn render_md_heading(
    level: u8,
    id: &'static str,
    href: &'static str,
    inline: &'static [DocsInline],
) -> VirtualNode {
    let anchor: VirtualNode = html! {
        a {
            class: "header-anchor"
            href: href
            span { "#" }
        }
    };
    let content: VirtualNode = render_md_inlines(inline);
    match level {
        1 => html! {
            h1 {
                id: id
                anchor
                content
            }
        },
        2 => html! {
            h2 {
                id: id
                anchor
                content
            }
        },
        3 => html! {
            h3 {
                id: id
                anchor
                content
            }
        },
        4 => html! {
            h4 {
                id: id
                anchor
                content
            }
        },
        5 => html! {
            h5 {
                id: id
                anchor
                content
            }
        },
        _ => html! {
            h6 {
                id: id
                anchor
                content
            }
        },
    }
}

/// Renders an inline slice.
///
/// # Arguments
///
/// - `&'static [DocsInline]` - The inlines to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered inline sequence.
fn render_md_inlines(inlines: &'static [DocsInline]) -> VirtualNode {
    html! {
        for inline in inlines.iter() {
            { render_md_inline(inline) }
        }
    }
}

/// Renders a single inline node.
///
/// # Arguments
///
/// - `&'static DocsInline` - The inline to render.
///
/// # Returns
///
/// - `VirtualNode` - The rendered inline.
fn render_md_inline(inline: &'static DocsInline) -> VirtualNode {
    match inline {
        DocsInline::Text(text) => html! {
            { *text }
        },
        DocsInline::Strong(children) => {
            let content: VirtualNode = render_md_inlines(children);
            html! {
                strong {
                    content
                }
            }
        }
        DocsInline::Em(children) => {
            let content: VirtualNode = render_md_inlines(children);
            html! {
                em {
                    content
                }
            }
        }
        DocsInline::Del(children) => {
            let content: VirtualNode = render_md_inlines(children);
            html! {
                del {
                    content
                }
            }
        }
        DocsInline::Code(code) => html! {
            code {
                { *code }
            }
        },
        DocsInline::Link {
            href,
            external,
            children,
        } => {
            let content: VirtualNode = render_md_inlines(children);
            if *external {
                html! {
                    a {
                        href: *href
                        target: "_blank"
                        content
                    }
                }
            } else {
                html! {
                    a {
                        href: *href
                        content
                    }
                }
            }
        }
        DocsInline::Image { src, alt } => html! {
            img {
                src: *src
                alt: *alt
            }
        },
        DocsInline::TaskMarker(checked) => {
            if *checked {
                html! {
                    span {
                        class: "task-marker"
                        "☑"
                    }
                }
            } else {
                html! {
                    span {
                        class: "task-marker"
                        "☐"
                    }
                }
            }
        }
        DocsInline::SoftBreak => html! {
            " "
        },
        DocsInline::HardBreak => html! {
            br {}
        },
        DocsInline::Html(raw) => html! {
            span {
                inner_html: *raw
            }
        },
    }
}
