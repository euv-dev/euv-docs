# euv-docs

A **VuePress-style documentation site generator** built with
[euv](https://github.com/euv-dev/euv) + `euv-ui`, compiled to WebAssembly.

Write markdown in `docs/` — get a full docs site with a home hero, navbar,
multi-level collapsible sidebar, right anchor TOC, prev/next links, footer,
dark mode, and i18n.

## Quick start

```bash
cargo install euv-cli

# dev server with hot reload
euv run --dev --port 8080 --index-html template.html -- --target web --out-dir www/pkg --out-name euv_docs --no-typescript --no-pack

# production build → www/
euv build --release --index-html template.html -- --target web --out-dir www/pkg --out-name euv_docs --no-typescript --no-pack
```

Open <http://localhost:8080> after `euv run`.

## Writing docs

| Source file | Route |
| --- | --- |
| `docs/README.md` | `/` (home, with frontmatter hero) |
| `docs/guide/README.md` | `/guide/` (sidebar group index) |
| `docs/guide/getting-started.md` | `/guide/getting-started.html` |
| `docs/zh/README.md` | `/zh/` (locale home) |

- **Navbar, locales, footer, UI labels** — `docs/config.toml`
- **Sidebar** — auto-generated from the file tree; order with frontmatter `order: <int>`
- **Home page** — frontmatter `home: true` + `heroText` / `tagline` / `actions` / `features` / `footer`
- **Static assets** — put them in `docs/public/`, reference as `/logo.png`

## Markdown features

- Frontmatter (YAML)
- GFM tables, task lists, strikethrough, footnotes
- Fenced code blocks
- Heading permalinks + right anchor TOC (h2/h3)
- Custom containers: `::: tip` / `::: warning` / `::: danger` / `::: note [title]`
- Internal `.md` links rewritten to routes; external links open in a new tab
- Same-page anchors: `[text](#heading-slug)`

## Internationalization

Add a `[[locales]]` entry in `docs/config.toml` (`prefix = "/zh/"`) and a
matching directory `docs/zh/`. The navbar language dropdown switches locale,
keeping the current page when a translation exists.

## License

MIT
