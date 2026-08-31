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
    // Site-level CSS override: hide the per-heading `#` anchor glyph.
    //
    // euv-ui's `euv_markdown` wraps each heading text in
    // `<a class="header-anchor"><span>#</span></a>`. The default `.md-body
    // .header-anchor` rule renders that anchor with `opacity: 0` and
    // reveals it on `h1:hover` / `h2:hover` / etc. The hover-revealed
    // glyph is visually intrusive on this docs site, so we override it to
    // never paint. The heading element itself still carries the `id`, so
    // URL-hash deep linking continues to work; only the decorative hover
    // glyph is removed.
    //
    // Loaded after `EUV_MD_CSS` so the cascade order places these rules
    // after the upstream default; the `!important` qualifier guarantees
    // victory regardless of any later selector-specificity bumps from
    // upstream.
    Css::inject_css(
        ".md-body .header-anchor { display: none !important; } \
         .md-body h1:hover .header-anchor, \
         .md-body h2:hover .header-anchor, \
         .md-body h3:hover .header-anchor, \
         .md-body h4:hover .header-anchor, \
         .md-body h5:hover .header-anchor, \
         .md-body h6:hover .header-anchor { opacity: 0 !important; pointer-events: none !important; }",
    );
    App::mount("#app", app);
}
