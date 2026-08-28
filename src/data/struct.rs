use super::*;

/// One rendered markdown page.
#[derive(Clone, Copy, Debug)]
pub struct DocsPage {
    /// Full route (`/guide/getting-started.html`, `/zh/` …).
    pub route: &'static str,
    /// Owning locale prefix.
    pub locale: &'static str,
    /// Page title.
    pub title: &'static str,
    /// Content block AST (rendered by `euv_markdown`).
    pub blocks: &'static [EuvMdBlock],
    /// Anchor TOC entries.
    pub headings: &'static [EuvTocItem],
    /// Whether this is a home page.
    pub home: bool,
    /// Hero text (home pages).
    pub hero_text: &'static str,
    /// Tagline (home pages).
    pub tagline: &'static str,
    /// Hero actions (home pages).
    pub actions: &'static [EuvHeroAction],
    /// Feature cards (home pages).
    pub features: &'static [EuvFeature],
    /// Frontmatter footer override.
    pub footer: &'static str,
}

/// One locale.
#[derive(Clone, Copy, Debug)]
pub struct DocsLocale {
    /// Route prefix (`/` or `/zh/`).
    pub prefix: &'static str,
    /// BCP-47 language tag.
    pub lang: &'static str,
    /// Human label for the language dropdown.
    pub label: &'static str,
    /// Locale title override.
    pub title: &'static str,
    /// Locale description.
    pub description: &'static str,
    /// Footer text.
    pub footer: &'static str,
    /// Right TOC title label.
    pub toc_label: &'static str,
    /// Prev-page link label.
    pub prev_label: &'static str,
    /// Next-page link label.
    pub next_label: &'static str,
    /// Navbar items.
    pub navbar: &'static [EuvNavbarItem],
    /// Sidebar tree.
    pub sidebar: &'static [EuvSidebarItem],
}

/// The whole generated site.
#[derive(Clone, Copy, Debug)]
pub struct DocsSite {
    /// Site title.
    pub title: &'static str,
    /// Site description.
    pub description: &'static str,
    /// Navbar logo (emoji).
    pub logo: &'static str,
    /// All locales.
    pub locales: &'static [DocsLocale],
    /// All pages.
    pub pages: &'static [DocsPage],
}
