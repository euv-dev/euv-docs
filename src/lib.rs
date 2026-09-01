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
    // occupying horizontal space next to each heading.
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
    // On this docs site the design intent is "title flush left, `#`
    // appears as a hover affordance peeking into the left gutter
    // without moving the heading text". We lift the anchor out of
    // the heading's flow with `position: absolute` and pin it to the
    // heading's left edge offset by `left: -0.9em`, so:
    //
    // * Idle: anchor is invisible (opacity: 0 from upstream) and
    //   absolutely positioned — it does not participate in flow, so
    //   the heading text starts at the heading element's left edge
    //   with no phantom indent.
    // * Hover: anchor opacity flips to 1 (upstream) and the glyph
    //   appears at the same position as before, peeking into the
    //   `c_app_main` left gutter. Heading text never moves.
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
         .md-body .header-anchor { position: absolute !important; left: -0.9em !important; float: none !important; margin-left: 0 !important; padding-right: 0.2em !important; } \
         }",
    );
    App::mount("#app", app);
}
