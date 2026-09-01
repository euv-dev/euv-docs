//! euv-docs — a VuePress-style documentation site powered by euv + euv-ui.
//!
//! Markdown sources live in `docs/`; `build.rs` compiles them into the
//! `DocsSite` static consumed here at runtime.

mod component;
mod data;
mod router;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/docs_gen.rs"));
}

pub use std::{cell::RefCell, fmt::Debug, rc::Rc};

use {
    data::*,
    euv::{wasm_bindgen::prelude::*, web_sys::*, *},
    euv_ui::*,
    router::*,
};

use crate::component::*;

/// WASM entry point: injects global styles and mounts the app.
#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    inject_app_global_css();
    Css::inject_css(EUV_MD_CSS);
    // Site-level CSS override: stop the invisible `#` anchor from
    // occupying horizontal space next to each heading, and align the
    // glyph with body text on hover across all viewport widths.
    //
    // euv-ui's `euv_markdown` wraps every heading in
    // `<a class="header-anchor"><span>#</span></a>`. The upstream
    // desktop rule is `float: left; margin-left: -0.9em; opacity: 0`
    // and reveals the glyph on `h*:hover` via `opacity: 1`. The
    // mobile rule is `display: inline-flex; width: 1.6em;
    // margin-left: -1.6em` with a matching `padding-left: 1.6em` on
    // the heading. Both rules reserve horizontal space for the
    // anchor even when it is invisible (opacity: 0), so heading
    // text is pushed right by ~12px (desktop) or ~1.6em (mobile)
    // regardless of hover state. The text "looks indented" all the
    // time, even though the glyph itself is only ever painted on
    // hover.
    //
    // On this docs site the design intent is:
    //
    // * Idle: title flush left at the same x as body paragraphs
    //   below, no phantom indent — at every viewport width.
    // * Hover: `#` glyph appears with its right edge touching the
    //   heading text on its baseline. The heading text stays at the
    //   same x as the body paragraphs below, so the whole column
    //   reads as a single left-aligned block — the `#` is a leading
    //   inline decoration, not a margin placeholder.
    //
    // To achieve both states on every viewport, we override both
    // euv-ui's desktop and mobile rules above with one neutral rule:
    //
    // * Lift the anchor out of flow with `position: absolute` so it
    //   does not push the heading text in the idle state.
    // * Reset the upstream `padding-left: 1.6em` on the heading
    //   (mobile rule) so the heading text is flush left at idle.
    //   On desktop the heading's `padding-left` was already 0 after
    //   PR #22, so this is a no-op there.
    // * Override the upstream mobile `display: inline-flex` plus
    //   `width: 1.6em` on the anchor with `display: block; width:
    //   auto`. The mobile anchor uses a flex box that pushes the
    //   `#` glyph to its left edge via `justify-content: flex-start`,
    //   which combined with `width: 1.6em` made the anchor box far
    //   wider than the `#` glyph itself — `transform: translateX(-100%)`
    //   then parked the entire flex container to the left of the
    //   heading, leaving a ~20px visible gap between the `#` and the
    //   heading text on mobile. Switching to `display: block; width:
    //   auto` shrinks the anchor box to the `#` glyph's natural
    //   width, so `translateX(-100%)` parks exactly the glyph at
    //   the heading's left edge.
    // * Reset `font-size: inherit` and `line-height: inherit` so the
    //   `#` glyph renders at the heading's own font size on mobile
    //   (instead of the upstream 0.85em scale), so h1 / h2 / h3 all
    //   sit at the same vertical size as the heading text.
    // * Pin the anchor's right edge to the heading's left edge via
    //   `left: 0; transform: translateX(-100%)`. With the block layout
    //   above, this works for any font size, so h1 (30px) and h2/h3
    //   (24px / 20.4px) on mobile all place the `#` glyph with its
    //   right edge exactly at the paragraph-text x.
    // * Drop the upstream `margin-left` and `padding-right` slack on
    //   the anchor — both would push the glyph away from the
    //   heading text in the new layout.
    //
    // Loaded after `EUV_MD_CSS` so cascade order places these rules
    // after the upstream defaults; `!important` keeps the rule safe
    // against future selector-specificity bumps from upstream.
    Css::inject_css(
        ".md-body h1, .md-body h2, .md-body h3, .md-body h4, .md-body h5, .md-body h6 { padding-left: 0 !important; } \
         .md-body .header-anchor { position: absolute !important; left: 0 !important; right: auto !important; float: none !important; display: block !important; width: auto !important; margin-left: 0 !important; margin-right: 0 !important; padding: 0 !important; font-size: inherit !important; line-height: inherit !important; transform: translateX(-100%) !important; text-decoration: none !important; vertical-align: baseline !important; }",
    );
    App::mount("#app", app);
}
