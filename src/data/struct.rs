/// One entry of the right-side anchor TOC (h2 / h3).
#[derive(Clone, Copy, Debug)]
pub struct DocsHeading {
    /// Heading level (2 or 3).
    pub level: u8,
    /// Slug used as the element id.
    pub id: &'static str,
    /// Heading text.
    pub text: &'static str,
}

/// One hero action button on the home page.
#[derive(Clone, Copy, Debug, Default)]
pub struct DocsAction {
    /// Button text.
    pub text: &'static str,
    /// Target route or external URL.
    pub link: &'static str,
    /// `primary` or `secondary`.
    pub kind: &'static str,
}

/// One feature card on the home page.
#[derive(Clone, Copy, Debug)]
pub struct DocsFeature {
    /// Feature title.
    pub title: &'static str,
    /// Feature details.
    pub details: &'static str,
}

/// One rendered markdown page.
#[derive(Clone, Copy, Debug)]
pub struct DocsPage {
    /// Full route (`/guide/getting-started.html`, `/zh/` …).
    pub route: &'static str,
    /// Owning locale prefix.
    pub locale: &'static str,
    /// Page title.
    pub title: &'static str,
    /// Rendered HTML body.
    pub html: &'static str,
    /// Anchor TOC entries.
    pub headings: &'static [DocsHeading],
    /// Whether this is a home page.
    pub home: bool,
    /// Hero text (home pages).
    pub hero_text: &'static str,
    /// Tagline (home pages).
    pub tagline: &'static str,
    /// Hero actions (home pages).
    pub actions: &'static [DocsAction],
    /// Feature cards (home pages).
    pub features: &'static [DocsFeature],
    /// Frontmatter footer override.
    pub footer: &'static str,
}

/// One navbar item.
#[derive(Clone, Copy, Debug, Default)]
pub struct DocsNavItem {
    /// Display text.
    pub text: &'static str,
    /// Link target.
    pub link: &'static str,
}

/// One sidebar node (leaf link or collapsible group).
#[derive(Clone, Copy, Debug, Default)]
pub struct DocsSidebarItem {
    /// Display text.
    pub text: &'static str,
    /// Leaf route; groups may link to their index page.
    pub link: Option<&'static str>,
    /// Nested children (non-empty → group).
    pub children: &'static [DocsSidebarItem],
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
    pub navbar: &'static [DocsNavItem],
    /// Sidebar tree.
    pub sidebar: &'static [DocsSidebarItem],
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
