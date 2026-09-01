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
    // glyph with body text on hover.
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
    //   below, no phantom indent.
    // * Hover: `#` glyph appears at the heading's left edge,
    //   touching the heading text on its baseline. The heading text
    //   stays at the same x as the body paragraphs below, so the
    //   whole column reads as a single left-aligned block — the `#`
    //   is a leading inline decoration, not a margin placeholder.
    //
    // To achieve both states on the desktop, we:
    //
    // * Lift the anchor out of flow with `position: absolute` so it
    //   does not push the heading text in the idle state.
    // * Pin the anchor at `left: -0.87em` (~ `#` glyph width + 1px slack
    //   for the heading text's first-character side-bearing) so the
    //   glyph's right edge sits at the heading's left edge, with the
    //   glyph visually touching the heading text on the same baseline
    //   and the heading text aligned with the body paragraph text below.
    // * Keep the heading's `padding-left` at `0` so the heading text
    //   never shifts on hover; the `#` floats in the heading's
    //   left-margin zone without disturbing the column.
    //
    // The override is scoped to desktop only because mobile uses a
    // different mechanism (inline-flex + heading padding-left); that
    // path already produces the right gutter placement and the hover
    // affordance there is unaffected.
    //
    // Loaded after `EUV_MD_CSS` so cascade order places these rules
    // after the upstream defaults; `!important` keeps the rule safe
    // against future selector-specificity bumps from upstream.
    Css::inject_css(
        "@media (min-width: 768px) { \
         .md-body h1, .md-body h2, .md-body h3, .md-body h4, .md-body h5, .md-body h6 { padding-left: 0 !important; } \
         .md-body .header-anchor { position: absolute !important; left: -0.87em !important; right: auto !important; float: none !important; margin-left: 0 !important; padding-right: 0 !important; } \
         }",
    );
    App::mount("#app", app);
}
