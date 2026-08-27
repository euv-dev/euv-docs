//! Build script for euv-docs.
//!
//! Scans the `docs/` directory, parses `docs/config.toml` plus every
//! `**/*.md` file (frontmatter + VuePress-flavored markdown), and generates
//! `docs_gen.rs` into `OUT_DIR`. The generated file constructs a single
//! `DocsSite` static consumed by the WASM app at runtime.

use std::{
    collections::HashSet,
    env, fs,
    path::{Component, Path, PathBuf},
};

use {
    pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, html},
    serde::Deserialize,
    serde_yaml::Value as Yaml,
};

// ═══════════════════════════════════════════════════════════════════════════
// Config schema (docs/config.toml)
// ═══════════════════════════════════════════════════════════════════════════

/// Root of `docs/config.toml`.
#[derive(Debug, Deserialize)]
struct Config {
    /// `[site]` table.
    site: SiteConfig,
    /// `[[locales]]` array.
    locales: Vec<LocaleConfig>,
}

/// `[site]` table.
#[derive(Debug, Deserialize)]
struct SiteConfig {
    /// Site title shown in the navbar.
    title: String,
    /// Site description (meta).
    description: Option<String>,
    /// Emoji / short logo text in the navbar.
    logo: Option<String>,
}

/// One `[[locales]]` entry.
#[derive(Debug, Deserialize)]
struct LocaleConfig {
    /// Route prefix, e.g. `/` or `/zh/`.
    prefix: String,
    /// BCP-47 language tag.
    lang: String,
    /// Human label in the language dropdown, e.g. `简体中文`.
    label: String,
    /// Locale-specific site title override.
    title: Option<String>,
    /// Locale-specific description override.
    description: Option<String>,
    /// Footer text for this locale.
    footer: Option<String>,
    /// Right TOC title label (default `On this page`).
    toc_label: Option<String>,
    /// Prev-page link label (default `Previous`).
    prev_label: Option<String>,
    /// Next-page link label (default `Next`).
    next_label: Option<String>,
    /// Navbar items for this locale.
    navbar: Option<Vec<NavItemConfig>>,
}

/// One navbar item.
#[derive(Debug, Deserialize, Clone)]
struct NavItemConfig {
    /// Display text.
    text: String,
    /// Link target (`/guide/` internal or `https://…` external).
    link: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Page / sidebar model (build-time)
// ═══════════════════════════════════════════════════════════════════════════

/// A heading extracted for the right-side anchor TOC.
#[derive(Debug, Clone)]
struct Heading {
    /// 2 or 3.
    level: u8,
    /// Slug used as the element id.
    id: String,
    /// Plain text content.
    text: String,
}

/// A parsed markdown page.
#[derive(Debug)]
struct Page {
    /// Full route, e.g. `/guide/getting-started.html` or `/zh/`.
    route: String,
    /// Locale prefix this page belongs to.
    locale: String,
    /// Page title (frontmatter `title` or first heading).
    title: String,
    /// Rendered HTML body.
    html: String,
    /// Anchor TOC entries (h2/h3).
    headings: Vec<Heading>,
    /// Home page flag (frontmatter `home: true`).
    home: bool,
    /// Hero text (home pages).
    hero_text: String,
    /// Tagline (home pages).
    tagline: String,
    /// Hero actions.
    actions: Vec<(String, String, String)>,
    /// Feature cards.
    features: Vec<(String, String)>,
    /// Frontmatter footer override.
    footer: String,
    /// Frontmatter `order` (sidebar sorting).
    order: i64,
}

/// A sidebar tree node.
#[derive(Debug)]
struct SideItem {
    /// Display text.
    text: String,
    /// Optional link route (group index or leaf page).
    link: Option<String>,
    /// Nested children (non-empty → group).
    children: Vec<SideItem>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Entry
// ═══════════════════════════════════════════════════════════════════════════

/// Entry point of the build script.
fn main() {
    let manifest_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let docs_dir: PathBuf = manifest_dir.join("docs");
    let out_dir: String = env::var("OUT_DIR").expect("OUT_DIR");

    println!("cargo:rerun-if-changed=docs");

    let config_raw: String =
        fs::read_to_string(docs_dir.join("config.toml")).expect("docs/config.toml is required");
    let config: Config = toml::from_str(&config_raw).expect("docs/config.toml must be valid TOML");

    // Locale prefixes other than the root locale, used to split pages.
    let locale_dirs: Vec<String> = config
        .locales
        .iter()
        .filter(|l| l.prefix != "/")
        .map(|l| l.prefix.trim_matches('/').to_string())
        .collect();

    // Collect markdown files.
    let mut md_files: Vec<PathBuf> = Vec::new();
    collect_md(&docs_dir, &mut md_files);
    md_files.sort();

    let mut pages: Vec<Page> = Vec::new();
    for file in &md_files {
        pages.push(process_page(&docs_dir, file, &locale_dirs));
    }

    // Copy docs/public assets into www/ so the dev server and the build
    // output can serve them from the site root.
    let public_dir: PathBuf = docs_dir.join("public");
    if public_dir.is_dir() {
        let www_dir: PathBuf = manifest_dir.join("www");
        copy_dir(&public_dir, &www_dir);
    }

    // Build the sidebar tree per locale (auto-generated from the file tree).
    let mut sidebars: Vec<(String, Vec<SideItem>)> = Vec::new();
    for locale in &config.locales {
        let root: PathBuf = if locale.prefix == "/" {
            docs_dir.clone()
        } else {
            docs_dir.join(locale.prefix.trim_matches('/'))
        };
        let items: Vec<SideItem> = build_sidebar(&root, &root, &locale.prefix, &pages, true);
        sidebars.push((locale.prefix.clone(), items));
    }

    let code: String = codegen(&config, &pages, &sidebars);
    fs::write(PathBuf::from(out_dir).join("docs_gen.rs"), code).expect("write docs_gen.rs");
}

// ═══════════════════════════════════════════════════════════════════════════
// Filesystem helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Recursively collects `*.md` files, skipping `config.toml` and `public/`.
fn collect_md(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "public") {
                continue;
            }
            collect_md(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

/// Recursively copies a directory tree.
fn copy_dir(src: &Path, dst: &Path) {
    let Ok(entries) = fs::read_dir(src) else {
        return;
    };
    let _ = fs::create_dir_all(dst);
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        let target: PathBuf = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &target);
        } else {
            let _ = fs::copy(&path, &target);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Page processing
// ═══════════════════════════════════════════════════════════════════════════

/// Parses one markdown file into a [`Page`].
fn process_page(docs_dir: &Path, file: &Path, locale_dirs: &[String]) -> Page {
    let raw: String = fs::read_to_string(file).expect("read md");
    let (frontmatter, body) = split_frontmatter(&raw);

    let rel: &Path = file.strip_prefix(docs_dir).expect("strip docs dir");
    let mut segments: Vec<String> = rel
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect();

    // Locale detection from the first path segment.
    let locale: String = if let Some(first) = segments.first() {
        if locale_dirs.contains(first) {
            let prefix: String = format!("/{first}/");
            segments.remove(0);
            prefix
        } else {
            "/".to_string()
        }
    } else {
        "/".to_string()
    };

    // Route computation (VuePress convention).
    let route: String = route_for(&segments, &locale);

    let fm_title: Option<String> = yaml_str(&frontmatter, "title");
    let order: i64 = yaml_i64(&frontmatter, "order").unwrap_or(0);

    let (html, headings, first_h1) = render_markdown(body, &route);

    let title: String = fm_title
        .or(first_h1)
        .unwrap_or_else(|| prettify(stem_of(&segments)));

    let home: bool = yaml_bool(&frontmatter, "home");
    let hero_text: String = yaml_str(&frontmatter, "heroText")
        .or_else(|| yaml_str(&frontmatter, "hero_text"))
        .unwrap_or_default();
    let tagline: String = yaml_str(&frontmatter, "tagline").unwrap_or_default();
    let footer: String = yaml_str(&frontmatter, "footer").unwrap_or_default();

    let actions: Vec<(String, String, String)> = yaml_list(&frontmatter, "actions")
        .iter()
        .map(|item| {
            (
                yaml_str(item, "text").unwrap_or_default(),
                yaml_str(item, "link").unwrap_or_default(),
                yaml_str(item, "type").unwrap_or_else(|| "primary".to_string()),
            )
        })
        .collect();

    let features: Vec<(String, String)> = yaml_list(&frontmatter, "features")
        .iter()
        .map(|item| {
            (
                yaml_str(item, "title").unwrap_or_default(),
                yaml_str(item, "details").unwrap_or_default(),
            )
        })
        .collect();

    Page {
        route,
        locale,
        title,
        html,
        headings,
        home,
        hero_text,
        tagline,
        actions,
        features,
        footer,
        order,
    }
}

/// Computes the VuePress-style route for a page.
///
/// - `README.md` / `index.md` → directory route with trailing slash.
/// - `foo.md` → `/foo.html`.
fn route_for(segments: &[String], locale: &str) -> String {
    let stem: String = stem_of(segments);
    let dir_parts: &[String] = if segments.is_empty() {
        &[]
    } else {
        &segments[..segments.len() - 1]
    };
    let dir_path: String = if dir_parts.is_empty() {
        String::new()
    } else {
        format!("{}/", dir_parts.join("/"))
    };
    if stem == "README" || stem == "index" {
        let base: String = format!("/{dir_path}");
        join_locale_route(locale, &base)
    } else {
        let base: String = format!("/{dir_path}{stem}.html");
        join_locale_route(locale, &base)
    }
}

/// Joins a locale prefix with a base route.
fn join_locale_route(locale: &str, base: &str) -> String {
    if locale == "/" {
        base.to_string()
    } else {
        format!("{}{}", locale.trim_end_matches('/'), base)
    }
}

/// Returns the file stem of the last segment.
fn stem_of(segments: &[String]) -> String {
    segments
        .last()
        .map(|s| s.trim_end_matches(".md").to_string())
        .unwrap_or_default()
}

/// Converts a file/dir name into a human title (`getting-started` → `Getting Started`).
fn prettify(name: String) -> String {
    let mut out: String = String::new();
    let mut capitalize: bool = true;
    for ch in name.chars() {
        if ch == '-' || ch == '_' {
            out.push(' ');
            capitalize = true;
        } else if capitalize {
            out.extend(ch.to_uppercase());
            capitalize = false;
        } else {
            out.push(ch);
        }
    }
    if out.is_empty() {
        "Index".to_string()
    } else {
        out
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Frontmatter helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Splits a markdown source into (frontmatter YAML value, body).
fn split_frontmatter(raw: &str) -> (Yaml, &str) {
    let trimmed: &str = raw.trim_start();
    if !trimmed.starts_with("---") {
        return (Yaml::Null, raw);
    }
    let after_open: &str = &trimmed[3..];
    let Some(after_open) = after_open.strip_prefix(['\n', '\r'].as_ref()) else {
        return (Yaml::Null, raw);
    };
    let Some(end) = after_open.find("\n---") else {
        return (Yaml::Null, raw);
    };
    let fm_src: &str = &after_open[..end];
    let body: &str = &after_open[end + 4..];
    let yaml: Yaml = serde_yaml::from_str(fm_src).unwrap_or(Yaml::Null);
    (yaml, body)
}

/// Reads a string field from a YAML mapping.
fn yaml_str(value: &Yaml, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Reads an i64 field from a YAML mapping.
fn yaml_i64(value: &Yaml, key: &str) -> Option<i64> {
    value.get(key).and_then(|v| v.as_i64())
}

/// Reads a bool field from a YAML mapping.
fn yaml_bool(value: &Yaml, key: &str) -> bool {
    value.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
}

/// Reads a list field from a YAML mapping.
fn yaml_list<'a>(value: &'a Yaml, key: &str) -> &'a [Yaml] {
    value
        .get(key)
        .and_then(|v| v.as_sequence())
        .map(|s| s.as_slice())
        .unwrap_or(&[])
}

// ═══════════════════════════════════════════════════════════════════════════
// Markdown rendering (VuePress-flavored subset)
// ═══════════════════════════════════════════════════════════════════════════

/// Renders markdown to HTML, extracting headings and the first h1.
fn render_markdown(body: &str, route: &str) -> (String, Vec<Heading>, Option<String>) {
    let segments: Vec<Segment> = split_containers(body);
    let mut out: String = String::new();
    let mut headings: Vec<Heading> = Vec::new();
    let mut first_h1: Option<String> = None;
    let mut used_slugs: HashSet<String> = HashSet::new();

    for segment in segments {
        match segment {
            Segment::Markdown(src) => {
                render_cmark(
                    &src,
                    route,
                    &mut out,
                    &mut headings,
                    &mut first_h1,
                    &mut used_slugs,
                );
            }
            Segment::Container { kind, title, body } => {
                let mut inner: String = String::new();
                let nested: Vec<Segment> = split_containers(&body);
                for seg in nested {
                    if let Segment::Markdown(src) = seg {
                        render_cmark(
                            &src,
                            route,
                            &mut inner,
                            &mut headings,
                            &mut first_h1,
                            &mut used_slugs,
                        );
                    }
                }
                let label: String = title.unwrap_or_else(|| kind.to_uppercase());
                out.push_str(&format!(
                    "<div class=\"docs-container {kind}\"><p class=\"docs-container-title\">{label}</p>{inner}</div>"
                ));
            }
        }
    }
    (out, headings, first_h1)
}

/// A source segment: plain markdown or a `:::` custom container.
enum Segment {
    /// Plain markdown.
    Markdown(String),
    /// `::: kind [title]` container.
    Container {
        /// Container kind (tip / warning / danger / …).
        kind: String,
        /// Optional custom title.
        title: Option<String>,
        /// Raw markdown body.
        body: String,
    },
}

/// Splits source into markdown / container segments (containers do not nest).
fn split_containers(src: &str) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();
    let mut buf: String = String::new();
    let mut in_container: bool = false;
    let mut kind: String = String::new();
    let mut title: Option<String> = None;
    let mut body: String = String::new();

    for line in src.lines() {
        let trimmed: &str = line.trim_end();
        if !in_container && trimmed.starts_with(":::") {
            let rest: &str = trimmed[3..].trim();
            if rest.is_empty() {
                // Stray `:::` — treat as plain text.
                buf.push_str(line);
                buf.push('\n');
                continue;
            }
            if !buf.trim().is_empty() {
                segments.push(Segment::Markdown(std::mem::take(&mut buf)));
            } else {
                buf.clear();
            }
            let mut parts = rest.splitn(2, char::is_whitespace);
            kind = parts.next().unwrap_or("info").to_string();
            title = parts
                .next()
                .map(str::trim)
                .filter(|t| !t.is_empty())
                .map(str::to_string);
            in_container = true;
            body.clear();
            continue;
        }
        if in_container && trimmed == ":::" {
            segments.push(Segment::Container {
                kind: std::mem::take(&mut kind),
                title: title.take(),
                body: std::mem::take(&mut body),
            });
            in_container = false;
            continue;
        }
        if in_container {
            body.push_str(line);
            body.push('\n');
        } else {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    if in_container {
        // Unterminated container — emit as markdown.
        buf.push_str(&format!("::: {kind}\n{body}"));
    }
    if !buf.trim().is_empty() {
        segments.push(Segment::Markdown(buf));
    }
    segments
}

/// Renders one markdown chunk with pulldown-cmark.
fn render_cmark(
    src: &str,
    route: &str,
    out: &mut String,
    headings: &mut Vec<Heading>,
    first_h1: &mut Option<String>,
    used_slugs: &mut HashSet<String>,
) {
    let options: Options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES;

    let parser = Parser::new_ext(src, options);
    let mut events: Vec<Event> = Vec::new();
    let mut iter = parser.into_iter().peekable();

    while let Some(event) = iter.next() {
        match event {
            Event::Start(Tag::Heading { level, id: _, .. }) => {
                let mut inner_events: Vec<Event> = Vec::new();
                for ev in iter.by_ref() {
                    if matches!(ev, Event::End(TagEnd::Heading(_))) {
                        break;
                    }
                    inner_events.push(ev);
                }
                let mut inner_html: String = String::new();
                html::push_html(&mut inner_html, inner_events.iter().cloned());
                let text: String = plain_text(&inner_events);
                let slug: String = unique_slug(&slugify(&text), used_slugs);
                let n: u8 = heading_level_num(level);
                if n == 2 || n == 3 {
                    headings.push(Heading {
                        level: n,
                        id: slug.clone(),
                        text: text.clone(),
                    });
                }
                if n == 1 && first_h1.is_none() {
                    *first_h1 = Some(text);
                }
                events.push(Event::Html(
                    format!(
                        "<h{n} id=\"{slug}\"><a class=\"header-anchor\" href=\"#{route}#{slug}\" aria-hidden=\"true\">#</a>{inner_html}</h{n}>"
                    )
                    .into(),
                ));
            }
            Event::Start(Tag::Link {
                dest_url, title, ..
            }) => {
                let (href, external) = rewrite_link(&dest_url, route);
                let title_attr: String = if title.is_empty() {
                    String::new()
                } else {
                    format!(" title=\"{}\"", escape_attr(&title))
                };
                let extra: String = if external {
                    " target=\"_blank\" rel=\"noopener noreferrer\"".to_string()
                } else {
                    String::new()
                };
                events.push(Event::Html(
                    format!("<a href=\"{}{title_attr}\"{extra}>", escape_attr(&href)).into(),
                ));
            }
            Event::End(TagEnd::Link) => {
                events.push(Event::Html("</a>".into()));
            }
            other => events.push(other),
        }
    }

    html::push_html(out, events.into_iter());
}

/// Extracts plain text from inline events.
fn plain_text(events: &[Event]) -> String {
    let mut text: String = String::new();
    for event in events {
        match event {
            Event::Text(t) | Event::Code(t) => text.push_str(t),
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            _ => {}
        }
    }
    text.trim().to_string()
}

/// Converts a heading level enum to a number.
fn heading_level_num(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Slugifies heading text (keeps CJK characters, VuePress-style).
fn slugify(text: &str) -> String {
    let mut out: String = String::new();
    let mut last_dash: bool = false;
    for ch in text.chars().flat_map(char::to_lowercase) {
        if ch.is_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if !last_dash && !out.is_empty() {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "section".to_string()
    } else {
        out
    }
}

/// Ensures slug uniqueness within a page.
fn unique_slug(base: &str, used: &mut HashSet<String>) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }
    let mut i: usize = 2;
    loop {
        let candidate: String = format!("{base}-{i}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        i += 1;
    }
}

/// Rewrites a markdown link target into a site URL.
///
/// Returns `(href, external)`.
fn rewrite_link(dest: &str, route: &str) -> (String, bool) {
    if dest.starts_with("http://") || dest.starts_with("https://") || dest.starts_with("mailto:") {
        return (dest.to_string(), true);
    }
    if let Some(anchor) = dest.strip_prefix('#') {
        return (format!("#{route}#{anchor}"), false);
    }
    let (path_part, anchor_part) = match dest.split_once('#') {
        Some((p, a)) => (p, Some(a)),
        None => (dest, None),
    };
    let href: String = if path_part.ends_with(".md") || path_part.ends_with(".md/") {
        let resolved: String = resolve_relative(route, path_part);
        format!("#{resolved}")
    } else if path_part.starts_with('/') {
        // Site-root-absolute link: map into the hash router.
        format!("#{path_part}")
    } else {
        // Non-markdown relative link (asset) — leave untouched.
        dest.to_string()
    };
    let href: String = match anchor_part {
        Some(a) if path_part.ends_with(".md") || path_part.starts_with('/') => {
            format!("{href}#{a}")
        }
        _ => href,
    };
    (href, false)
}

/// Resolves a relative markdown path against the current page route.
fn resolve_relative(route: &str, rel: &str) -> String {
    let rel: &str = rel.trim_end_matches('/');
    let mut stack: Vec<String> = Vec::new();
    if let Some(stripped) = rel.strip_prefix('/') {
        // Root-relative inside the current locale.
        let locale_prefix: String = current_locale_prefix(route);
        for seg in stripped.split('/') {
            stack.push(seg.to_string());
        }
        return md_path_to_route(&format!("{locale_prefix}{}", stack.join("/")));
    }
    // Directory of the current page route.
    let dir: &str = if route.ends_with('/') {
        route
    } else {
        match route.rfind('/') {
            Some(idx) => &route[..=idx],
            None => "/",
        }
    };
    for seg in dir.trim_matches('/').split('/') {
        if !seg.is_empty() {
            stack.push(seg.to_string());
        }
    }
    for seg in rel.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            s => stack.push(s.to_string()),
        }
    }
    md_path_to_route(&format!("/{}", stack.join("/")))
}

/// Extracts the locale prefix from a route (`/zh/guide/` → `/zh/`).
fn current_locale_prefix(route: &str) -> String {
    let trimmed: &str = route.trim_start_matches('/');
    match trimmed.split('/').next() {
        Some(first) if !first.is_empty() && !first.contains('.') => {
            // Ambiguous: treat known non-first-segment heuristics conservatively.
            // Locale prefixes are detected at build time; links starting with a
            // locale dir are already absolute, so only preserve root `/` here.
            let _ = first;
            "/".to_string()
        }
        _ => "/".to_string(),
    }
}

/// Converts a markdown path (`/guide/foo.md`) to a route (`/guide/foo.html`).
fn md_path_to_route(path: &str) -> String {
    let path: &str = path.trim_end_matches('/');
    if path.ends_with("README.md") || path.ends_with("index.md") {
        let dir: &str = &path[..path.rfind('/').unwrap_or(0) + 1];
        return dir.to_string();
    }
    if let Some(stripped) = path.strip_suffix(".md") {
        return format!("{stripped}.html");
    }
    path.to_string()
}

/// Escapes a string for use inside an HTML attribute.
fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// ═══════════════════════════════════════════════════════════════════════════
// Sidebar generation
// ═══════════════════════════════════════════════════════════════════════════

/// Recursively builds the sidebar tree for one locale directory.
fn build_sidebar(
    dir: &Path,
    locale_root: &Path,
    locale: &str,
    pages: &[Page],
    top_level: bool,
) -> Vec<SideItem> {
    let mut items: Vec<SideItem> = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return items;
    };
    let mut entries: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    entries.sort();

    for path in entries {
        let name: String = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if path.is_dir() {
            if name == "public" {
                continue;
            }
            let children: Vec<SideItem> = build_sidebar(&path, locale_root, locale, pages, false);
            if children.is_empty() {
                continue;
            }
            // Group title + optional index link from the directory README.
            let readme_route: String = {
                let rel_segments: Vec<String> = path
                    .strip_prefix(locale_root)
                    .map(|p| {
                        p.components()
                            .filter_map(|c| match c {
                                Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
                                _ => None,
                            })
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or_default();
                let mut segs: Vec<String> = rel_segments;
                segs.push("README.md".to_string());
                route_for(&segs, locale)
            };
            let readme_page: Option<&Page> = pages
                .iter()
                .find(|p| p.route == readme_route && p.locale == locale);
            let (text, link) = match readme_page {
                Some(page) => (page.title.clone(), Some(page.route.clone())),
                None => (prettify(name), None),
            };
            items.push(SideItem {
                text,
                link,
                children,
            });
        } else if name.ends_with(".md") && name != "README.md" && name != "index.md" {
            let rel_segments: Vec<String> = path
                .strip_prefix(locale_root)
                .map(|p| {
                    p.components()
                        .filter_map(|c| match c {
                            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
                            _ => None,
                        })
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default();
            let route: String = route_for(&rel_segments, locale);
            let Some(page) = pages
                .iter()
                .find(|p| p.route == route && p.locale == locale)
            else {
                continue;
            };
            items.push(SideItem {
                text: page.title.clone(),
                link: Some(route),
                children: Vec::new(),
            });
        }
    }

    // Sort: frontmatter `order` first, then alphabetical.
    let order_of = |item: &SideItem| -> i64 {
        item.link
            .as_ref()
            .and_then(|route| {
                pages
                    .iter()
                    .find(|p| &p.route == route && p.locale == locale)
            })
            .map(|p| p.order)
            .unwrap_or(0)
    };
    let _ = top_level;
    items.sort_by(|a, b| {
        order_of(a)
            .cmp(&order_of(b))
            .then_with(|| a.text.cmp(&b.text))
    });
    items
}

// ═══════════════════════════════════════════════════════════════════════════
// Code generation
// ═══════════════════════════════════════════════════════════════════════════

/// Emits the generated Rust source.
fn codegen(config: &Config, pages: &[Page], sidebars: &[(String, Vec<SideItem>)]) -> String {
    let mut code: String = String::new();
    code.push_str(
        "// @generated by build.rs — do not edit.\n//\n// Constructed from docs/config.toml and docs/**/*.md at build time.\n\n",
    );

    // Pages.
    let mut pages_code: String = String::new();
    for page in pages {
        let headings: String = page
            .headings
            .iter()
            .map(|h| {
                format!(
                    "crate::data::DocsHeading {{ level: {}, id: {:?}, text: {:?} }}",
                    h.level, h.id, h.text
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let actions: String = page
            .actions
            .iter()
            .map(|(text, link, kind)| {
                format!(
                    "crate::data::DocsAction {{ text: {:?}, link: {:?}, kind: {:?} }}",
                    text, link, kind
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let features: String = page
            .features
            .iter()
            .map(|(title, details)| {
                format!(
                    "crate::data::DocsFeature {{ title: {:?}, details: {:?} }}",
                    title, details
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        pages_code.push_str(&format!(
            "crate::data::DocsPage {{ route: {:?}, locale: {:?}, title: {:?}, html: {:?}, headings: &[{}], home: {}, hero_text: {:?}, tagline: {:?}, actions: &[{}], features: &[{}], footer: {:?} }},\n",
            page.route,
            page.locale,
            page.title,
            page.html,
            headings,
            page.home,
            page.hero_text,
            page.tagline,
            actions,
            features,
            page.footer,
        ));
    }

    // Locales (with sidebars).
    let mut locales_code: String = String::new();
    for locale in &config.locales {
        let sidebar_src: &Vec<SideItem> = sidebars
            .iter()
            .find(|(prefix, _)| prefix == &locale.prefix)
            .map(|(_, items)| items)
            .expect("sidebar for locale");
        let navbar: String = locale
            .navbar
            .as_ref()
            .map(|items| {
                items
                    .iter()
                    .map(|item| {
                        format!(
                            "crate::data::DocsNavItem {{ text: {:?}, link: {:?} }}",
                            item.text, item.link
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .unwrap_or_default();
        let sidebar_code: String = emit_sidebar(sidebar_src);
        locales_code.push_str(&format!(
            "crate::data::DocsLocale {{ prefix: {:?}, lang: {:?}, label: {:?}, title: {:?}, description: {:?}, footer: {:?}, toc_label: {:?}, prev_label: {:?}, next_label: {:?}, navbar: &[{}], sidebar: {} }},\n",
            locale.prefix,
            locale.lang,
            locale.label,
            locale.title.clone().unwrap_or_default(),
            locale.description.clone().unwrap_or_default(),
            locale.footer.clone().unwrap_or_default(),
            locale
                .toc_label
                .clone()
                .unwrap_or_else(|| "On this page".to_string()),
            locale
                .prev_label
                .clone()
                .unwrap_or_else(|| "Previous".to_string()),
            locale
                .next_label
                .clone()
                .unwrap_or_else(|| "Next".to_string()),
            navbar,
            sidebar_code,
        ));
    }

    code.push_str(&format!(
        "/// The generated site model.\npub static SITE: crate::data::DocsSite = crate::data::DocsSite {{\n    title: {:?},\n    description: {:?},\n    logo: {:?},\n    locales: &[{}],\n    pages: &[{}],\n}};\n",
        config.site.title,
        config.site.description.clone().unwrap_or_default(),
        config.site.logo.clone().unwrap_or_else(|| "📘".to_string()),
        locales_code,
        pages_code,
    ));
    code
}

/// Recursively emits a sidebar slice expression.
fn emit_sidebar(items: &[SideItem]) -> String {
    let inner: String = items
        .iter()
        .map(|item| {
            let link: String = match &item.link {
                Some(route) => format!("Some({route:?})"),
                None => "None".to_string(),
            };
            let children: String = emit_sidebar(&item.children);
            format!(
                "crate::data::DocsSidebarItem {{ text: {:?}, link: {}, children: {} }}",
                item.text, link, children
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!("&[{inner}]")
}
