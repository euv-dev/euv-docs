/// Raw stylesheet for rendered markdown bodies.
///
/// `class!` cannot express descendant selectors, so markdown typography is
/// injected once at startup through `Css::inject_css`. All colors reference
/// the euv-ui theme CSS variables, so light/dark switching works unchanged.
pub(crate) const DOCS_MD_CSS: &str = r#"
.md-body {
    line-height: 1.7;
    font-size: var(--font-base);
    word-wrap: break-word;
}
.md-body h1, .md-body h2, .md-body h3, .md-body h4, .md-body h5, .md-body h6 {
    position: relative;
    font-weight: 700;
    letter-spacing: -0.01em;
    margin-top: 1.8em;
    margin-bottom: 0.6em;
    scroll-margin-top: 72px;
    line-height: 1.3;
}
.md-body h1 {
    font-size: var(--font-3xl);
    margin-top: 0;
    padding-bottom: 0.4em;
    border-bottom: 1px solid var(--border);
}
.md-body h2 {
    font-size: var(--font-2xl);
    padding-bottom: 0.3em;
    border-bottom: 1px dashed var(--border);
}
.md-body h3 { font-size: var(--font-xl); }
.md-body h4 { font-size: var(--font-lg); }
.md-body h5, .md-body h6 { font-size: var(--font-base); }
.md-body .header-anchor {
    float: left;
    margin-left: -0.9em;
    padding-right: 0.2em;
    opacity: 0;
    color: var(--muted-foreground);
    font-weight: 400;
    transition: opacity 0.15s ease-out;
    user-select: none;
}
.md-body h1:hover .header-anchor,
.md-body h2:hover .header-anchor,
.md-body h3:hover .header-anchor,
.md-body h4:hover .header-anchor,
.md-body h5:hover .header-anchor,
.md-body h6:hover .header-anchor {
    opacity: 1;
}
.md-body p, .md-body ul, .md-body ol, .md-body blockquote, .md-body pre, .md-body table {
    margin: 1em 0;
}
.md-body ul, .md-body ol {
    padding-left: 1.4em;
}
.md-body ul { list-style: disc; }
.md-body ol { list-style: decimal; }
.md-body ul ul, .md-body ul ol, .md-body ol ul, .md-body ol ol {
    margin: 0.25em 0;
}
.md-body li { margin: 0.25em 0; }
.md-body li input[type="checkbox"] {
    margin-right: 0.4em;
    accent-color: var(--accent);
}
.md-body a {
    color: var(--accent);
    font-weight: 500;
    text-decoration: underline;
    text-underline-offset: 3px;
    text-decoration-style: dashed;
    text-decoration-color: var(--border);
}
.md-body a:hover {
    text-decoration-style: solid;
    text-decoration-color: var(--accent);
}
.md-body strong { font-weight: 700; }
.md-body em { font-style: italic; }
.md-body del { opacity: 0.6; }
.md-body hr {
    border: none;
    border-top: 1px dashed var(--border);
    margin: 2em 0;
}
.md-body blockquote {
    margin: 1em 0;
    padding: 0.4em 1em;
    border-left: 4px solid var(--border);
    color: var(--muted-foreground);
}
.md-body blockquote p { margin: 0.4em 0; }
.md-body code {
    font-family: ui-monospace, monospace;
    font-size: 0.875em;
    padding: 0.15em 0.4em;
    background: var(--accent-muted);
    border: 1px solid var(--border);
}
.md-body pre {
    padding: 1em 1.2em;
    overflow-x: auto;
    border: 1px solid var(--border);
    background: var(--accent-muted);
}
.md-body pre code {
    padding: 0;
    border: none;
    background: transparent;
    font-size: 0.875rem;
    line-height: 1.6;
}
.md-body table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-sm);
    display: block;
    overflow-x: auto;
}
.md-body table thead { border-bottom: 2px solid var(--border); }
.md-body table th, .md-body table td {
    padding: 0.5em 0.9em;
    border: 1px solid var(--border);
    text-align: left;
}
.md-body table th { font-weight: 700; }
.md-body table tbody tr:nth-child(2n) { background: var(--accent-muted); }
.md-body img { max-width: 100%; }
.md-body .docs-container {
    margin: 1.2em 0;
    padding: 0.1em 1.2em;
    border-left: 4px solid var(--foreground);
    background: var(--accent-muted);
}
.md-body .docs-container .docs-container-title {
    font-weight: 700;
    font-size: var(--font-sm);
    letter-spacing: 0.04em;
    text-transform: uppercase;
    margin: 0.8em 0 0.4em;
}
.md-body .docs-container.tip { border-left-style: solid; }
.md-body .docs-container.warning { border-left-style: dashed; }
.md-body .docs-container.danger {
    border-left: 4px double var(--foreground);
}
.md-body .docs-container.details {
    border-left: 1px solid var(--border);
    background: transparent;
}
.md-body .footnote-definition { font-size: var(--font-sm); color: var(--muted-foreground); }
"#;
