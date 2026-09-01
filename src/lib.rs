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
    // occupying horizontal space next to each heading, while keeping
    // the glyph baseline-aligned with the heading text on hover.
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
    // * Idle: title flush left, no phantom indent.
    // * Hover: `#` glyph appears, baseline-aligned with the heading
    //   text, with the project's standard `0.2em` inline gap.
    //
    // To achieve both states without the placeholder problem, on
    // desktop we:
    //
    // * Lift it out of flow with `position: absolute; left: 0` so it
    //   does not push the heading text in the idle state.
    // * Reserve space for it on hover only by animating the heading's
    //   `padding-left` from `0` to `1.1em` (~ glyph width 0.83em +
    //   gap 0.2em + small slack). On hover the heading content shifts
    //   right by exactly the room the glyph needs, so the `#` and
    //   text sit on the same baseline with a clean `0.2em` gap.
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
         .md-body h1, .md-body h2, .md-body h3, .md-body h4, .md-body h5, .md-body h6 { padding-left: 0 !important; transition: padding-left 0.15s ease-out !important; } \
         .md-body h1:hover, .md-body h2:hover, .md-body h3:hover, .md-body h4:hover, .md-body h5:hover, .md-body h6:hover { padding-left: 1.1em !important; } \
         .md-body .header-anchor { position: absolute !important; left: 0 !important; float: none !important; margin-left: 0 !important; padding-right: 0.2em !important; } \
         }",
    );
    App::mount("#app", app);
}
