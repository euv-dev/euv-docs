---
title: Markdown Features
---

# Markdown Features

euv-docs renders a VuePress-flavored markdown subset at build time.

## Headings

Headings `h2` and `h3` appear in the right anchor TOC automatically.

## Inline formatting

**Bold**, *italic*, ~~strikethrough~~, and `inline code` are supported.

## Lists

- Unordered item
- Another item
  - Nested item

1. Ordered item
2. Second item

- [x] Task list item (done)
- [ ] Task list item (todo)

## Tables

| Feature | VuePress | euv-docs |
| --- | --- | --- |
| Frontmatter | ✅ | ✅ |
| Custom containers | ✅ | ✅ |
| Anchor TOC | ✅ | ✅ |

## Code blocks

```rust
fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}
```

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

::: note Custom title
Containers accept an optional custom title after the kind.
:::

## Blockquotes

> Documentation is a love letter to your future self.

## Links

- [Internal link to the guide index](./README.md)
- [External link to euv](https://github.com/euv-dev/euv)
- [Jump to Tables](#tables)
