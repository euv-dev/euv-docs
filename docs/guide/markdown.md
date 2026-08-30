---
title: Markdown Features
---

# Markdown Features

euv-docs renders a VuePress-flavored markdown subset at build time.

## Headings

Headings `h2` and `h3` appear in the right anchor TOC automatically. `h1`,
`h4`, `h5`, and `h6` render as plain headings and are not added to the TOC.

### h3 sub-heading

#### h4 sub-sub-heading

##### h5 sub-sub-sub-heading

###### h6 deepest heading

## Inline formatting

**Bold**, *italic*, ~~strikethrough~~, and `inline code` are supported.

Bold and italic can nest: ***bold italic***, **bold with `inline code` inside**, and
*a mix of `code` and emphasis*.

Inline links also nest: **[getting started](./getting-started.md)** and
*[external link to euv](https://github.com/euv-dev/euv)*.

A footnote reference renders inline as `[^example]` — it shows as plain
`[^example]` text (no auto-numbering, no footnote list).

## Hard break vs soft break

This paragraph ends with two trailing spaces, so the next line  
starts a new line in the same paragraph (hard break, `<br>`).

This paragraph ends with no trailing whitespace, so the next line
glues onto the previous one as a single paragraph (soft break, just a
space).

## Lists

- Unordered item
- Another item
  - Nested unordered item
  - Another nested item
    1. Nested ordered item under unordered
    2. Second nested ordered
  - Back to unordered

1. Ordered item
2. Second item
3. Third item with a [link](./getting-started.md)

- [x] Task list item (done)
- [ ] Task list item (todo)
- [x] Completed task with `inline code`

## Tables

| Feature | VuePress | euv-docs | Notes |
| :--- | :---: | ---: | --- |
| Frontmatter | ✅ | ✅ | parsed at build time |
| Custom containers | ✅ | ✅ | `::: tip / warning / danger / note` |
| Anchor TOC | ✅ | ✅ | h2 + h3 only |
| Table alignment | ✅ | ✅ | left / center / right |

## Code blocks

Fenced blocks with a language tag get syntax class on the wrapper:

```rust
fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}
```

```bash
euv build --release -- --target web --out-dir www/pkg
```

```toml
[site]
title = "euv-docs"

[[locales]]
prefix = "/"
lang = "en-US"
label = "English"
```

```text
plain text fence, no syntax class
```

```markdown
# markdown fence, no syntax class
```

Indented code blocks (4-space indent) also work, but fenced form is
preferred because the language tag drives the wrapper class.

    // 4-space indented block renders as code
    fn main() {}

## Custom containers

::: tip
Tips are rendered with a solid border.
:::

::: warning
Warnings use a dashed border.
:::

::: danger
Danger containers use a double border.
:::

::: info
The full set of supported kinds is `tip`, `warning`, `danger`, `info`,
`note`, and `details` (defined in euv-ui's container CSS).
:::

::: note Custom title
Containers accept an optional custom title after the kind.
:::

::: details Click to expand
Hidden content rendered inside an expandable block.
:::

## Blockquotes

> Documentation is a love letter to your future self.

Nested blockquotes:

> First level.
>
> > Second level.
> >
> > Even **bold** and `code` work inside.
>
> Back to first level.

## Horizontal rules

Three or more of `-`, `_`, or `*` on a line:

---

***

___

## Images

![euv logo](https://euv-dev.github.io/euv-docs/favicon.svg)

Images render with the alt text and the source URL preserved.

## Links

- [Internal link to the guide index](./README.md)
- [External link to euv](https://github.com/euv-dev/euv)
- [Jump to Tables](#tables)
- [Mail link](mailto:nobody@example.com)
- [Link with title](https://github.com/euv-dev/euv "euv repository on GitHub")

## Escaped characters

Backslash escapes the next character: \*literal asterisks\*, \[not a link\],
\<not a tag\>, \`not code\`. The backslash is consumed and the character
renders as plain text.

## Inline and block HTML

A span of inline HTML is allowed inside paragraphs:

This paragraph has a <sup>superscript</sup> and a <sub>subscript</sub>
inlined with raw HTML tags.

A block of raw HTML passes through verbatim:

<div style="border: 1px dashed currentColor; padding: 8px 12px;">
  This block is rendered as raw HTML, not markdown.
</div>
