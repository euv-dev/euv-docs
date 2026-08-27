use super::super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════
    // Docs shell
    // ═══════════════════════════════════════════════════════════════════════

    pub(crate) c_docs_root {
        min-height: "100%";
        background: var!(background);
        color: var!(foreground);
    }

    // ── Top navbar ──────────────────────────────────────────────────────────

    pub(crate) c_docs_navbar {
        position: "fixed";
        top: "0px";
        left: "0px";
        right: "0px";
        height: "56px";
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        padding: format!("0px {}", var!(padding-main-horizontal));
        border-bottom: format!("1px solid {}", var!(border));
        background: var!(background);
        z-index: "100";
        media("(max-width: 767px)") {
            padding: format!("0px {}", var!(padding-main-horizontal-mobile));
        }
    }
    pub(crate) c_docs_navbar_brand {
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        font-size: var!(font-lg);
        font-weight: "700";
        letter-spacing: "-0.02em";
        color: var!(foreground);
        cursor: "pointer";
        flex-shrink: "0";
    }
    pub(crate) c_docs_navbar_logo {
        font-size: var!(font-2xl);
    }
    pub(crate) c_docs_navbar_links {
        display: "flex";
        align-items: "center";
        gap: var!(gap-section);
        margin-left: "auto";
        media("(max-width: 767px)") {
            display: "none";
        }
    }
    pub(crate) c_docs_navbar_link {
        font-size: var!(font-sm);
        font-weight: "500";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-bottom: "2px solid transparent";
        cursor: "pointer";
        hover {
            color: var!(accent);
        }
    }
    pub(crate) c_docs_navbar_link_active {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(accent);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-bottom: format!("2px solid {}", var!(accent));
        cursor: "pointer";
    }
    pub(crate) c_docs_navbar_actions {
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        margin-left: var!(gap-section);
        media("(max-width: 767px)") {
            margin-left: "auto";
        }
    }
    pub(crate) c_docs_icon_button {
        width: "36px";
        height: "36px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        border: format!("1px dashed {}", var!(border));
        cursor: "pointer";
        font-size: var!(font-base);
        hover {
            background: var!(accent-muted);
        }
    }
    pub(crate) c_docs_locale_menu {
        position: "relative";
    }
    pub(crate) c_docs_locale_dropdown {
        position: "absolute";
        top: "44px";
        right: "0px";
        min-width: "140px";
        background: var!(background);
        border: format!("1px solid {}", var!(border));
        box-shadow: var!(shadow-accent-lg);
        display: "flex";
        flex-direction: "column";
        z-index: "101";
    }
    pub(crate) c_docs_locale_item {
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        font-size: var!(font-sm);
        text-align: "left";
        cursor: "pointer";
        hover {
            background: var!(accent-muted);
        }
    }

    // ── Layout columns ──────────────────────────────────────────────────────

    pub(crate) c_docs_body {
        display: "flex";
        padding-top: "56px";
        min-height: "100vh";
    }
    pub(crate) c_docs_sidebar {
        position: "fixed";
        top: "56px";
        bottom: "0px";
        left: "0px";
        width: "260px";
        overflow-y: "auto";
        border-right: format!("1px solid {}", var!(border));
        padding: format!("{} {}", var!(space-xl), var!(space-lg));
        media("(max-width: 767px)") {
            display: "none";
        }
    }
    pub(crate) c_docs_main {
        flex: "1";
        margin-left: "260px";
        padding: format!("{} {}", var!(space-3xl), var!(padding-main-horizontal));
        display: "flex";
        justify-content: "center";
        media("(max-width: 767px)") {
            margin-left: "0px";
            padding: format!("{} {}", var!(space-xl), var!(padding-main-horizontal-mobile));
        }
    }
    pub(crate) c_docs_main_inner {
        display: "flex";
        gap: var!(space-4xl);
        width: "100%";
        max-width: "1080px";
    }
    pub(crate) c_docs_content {
        flex: "1";
        min-width: "0px";
        max-width: "760px";
    }
    pub(crate) c_docs_toc {
        width: "200px";
        flex-shrink: "0";
        media("(max-width: 1100px)") {
            display: "none";
        }
    }
    pub(crate) c_docs_toc_sticky {
        position: "sticky";
        top: "76px";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
        border-left: format!("1px solid {}", var!(border));
        padding-left: var!(space-lg);
    }
    pub(crate) c_docs_toc_title {
        font-size: var!(font-xs);
        font-weight: "700";
        text-transform: "uppercase";
        letter-spacing: "0.08em";
        color: var!(muted-foreground);
        margin-bottom: var!(space-xs);
    }
    pub(crate) c_docs_toc_link {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        cursor: "pointer";
        line-height: "1.5";
        hover {
            color: var!(accent);
        }
    }
    pub(crate) c_docs_toc_link_h3 {
        padding-left: var!(space-lg);
    }

    // ── Sidebar tree ────────────────────────────────────────────────────────

    pub(crate) c_docs_sidebar_group {
        margin-bottom: var!(space-xs);
    }
    pub(crate) c_docs_sidebar_group_title {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        width: "100%";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        font-size: var!(font-sm);
        font-weight: "600";
        cursor: "pointer";
        text-align: "left";
        hover {
            background: var!(accent-muted);
        }
    }
    pub(crate) c_docs_sidebar_group_arrow {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
    }
    pub(crate) c_docs_sidebar_group_arrow_open {
        transform: "rotate(90deg)";
    }
    pub(crate) c_docs_sidebar_children {
        display: "flex";
        flex-direction: "column";
        padding-left: var!(space-md);
        border-left: format!("1px dashed {}", var!(border));
        margin-left: var!(space-sm);
    }
    pub(crate) c_docs_sidebar_link {
        display: "block";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        font-size: var!(font-sm);
        color: var!(foreground);
        cursor: "pointer";
        hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }
    pub(crate) c_docs_sidebar_link_active {
        display: "block";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        font-size: var!(font-sm);
        background: var!(accent);
        color: var!(text-on-accent);
        font-weight: "600";
        cursor: "pointer";
    }

    // ── Home page ───────────────────────────────────────────────────────────

    pub(crate) c_docs_hero {
        text-align: "center";
        padding: format!("{} 0px", var!(space-7xl));
    }
    pub(crate) c_docs_hero_title {
        font-size: var!(font-6xl);
        font-weight: "800";
        letter-spacing: "-0.03em";
        media("(max-width: 767px)") {
            font-size: var!(font-4xl);
        }
    }
    pub(crate) c_docs_hero_tagline {
        font-size: var!(font-xl);
        color: var!(muted-foreground);
        margin-top: var!(space-lg);
        max-width: "640px";
        margin-left: "auto";
        margin-right: "auto";
        media("(max-width: 767px)") {
            font-size: var!(font-base);
        }
    }
    pub(crate) c_docs_hero_actions {
        display: "flex";
        gap: var!(gap-component);
        justify-content: "center";
        margin-top: var!(space-3xl);
        flex-wrap: "wrap";
    }
    pub(crate) c_docs_hero_button_primary {
        display: "inline-flex";
        align-items: "center";
        padding: format!("{} {}", var!(space-sm), var!(space-2xl));
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-base);
        font-weight: "600";
        border: "1.5px solid transparent";
        cursor: "pointer";
    }
    pub(crate) c_docs_hero_button_secondary {
        display: "inline-flex";
        align-items: "center";
        padding: format!("{} {}", var!(space-sm), var!(space-2xl));
        color: var!(accent);
        font-size: var!(font-base);
        font-weight: "600";
        border: format!("1.5px solid {}", var!(accent));
        cursor: "pointer";
    }
    pub(crate) c_docs_features {
        display: "grid";
        grid-template-columns: "repeat(3, 1fr)";
        gap: var!(gap-section);
        margin-top: var!(space-4xl);
        media("(max-width: 960px)") {
            grid-template-columns: "repeat(2, 1fr)";
        }
        media("(max-width: 767px)") {
            grid-template-columns: "1fr";
        }
    }
    pub(crate) c_docs_feature_card {
        border: format!("1px solid {}", var!(border));
        padding: var!(space-xl);
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
    }
    pub(crate) c_docs_feature_title {
        font-size: var!(font-lg);
        font-weight: "700";
    }
    pub(crate) c_docs_feature_details {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        line-height: "1.6";
    }

    // ── Footer / prev-next ──────────────────────────────────────────────────

    pub(crate) c_docs_footer {
        margin-top: var!(space-7xl);
        padding: format!("{} 0px", var!(space-2xl));
        border-top: format!("1px dashed {}", var!(border));
        text-align: "center";
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }
    pub(crate) c_docs_prev_next {
        display: "flex";
        justify-content: "space-between";
        gap: var!(gap-component);
        margin-top: var!(space-4xl);
        media("(max-width: 767px)") {
            flex-direction: "column";
        }
    }
    pub(crate) c_docs_prev_next_link {
        flex: "1";
        border: format!("1px solid {}", var!(border));
        padding: var!(space-lg);
        cursor: "pointer";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        hover {
            border-color: var!(accent);
        }
    }
    pub(crate) c_docs_prev_next_label {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        text-transform: "uppercase";
        letter-spacing: "0.08em";
    }
    pub(crate) c_docs_prev_next_text {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
    }
    pub(crate) c_docs_prev_next_next {
        text-align: "right";
    }
    pub(crate) c_docs_prev_next_spacer {
        flex: "1";
    }

    // ── Markdown body base (typography lives in style/css.rs) ─────────────

    pub(crate) c_docs_md_body {
        width: "100%";
        min-width: "0px";
    }

    // ── Mobile ──────────────────────────────────────────────────────────────

    pub(crate) c_docs_mobile_menu_button {
        display: "none";
        width: "40px";
        height: "40px";
        align-items: "center";
        justify-content: "center";
        font-size: var!(font-xl);
        cursor: "pointer";
        media("(max-width: 767px)") {
            display: "flex";
        }
    }
    pub(crate) c_docs_mobile_overlay {
        position: "fixed";
        top: "0px";
        left: "0px";
        right: "0px";
        bottom: "0px";
        background: var!(bg-overlay);
        z-index: "200";
    }
    pub(crate) c_docs_mobile_drawer {
        position: "fixed";
        top: "0px";
        left: "0px";
        bottom: "0px";
        width: "260px";
        background: var!(background);
        border-right: format!("1px solid {}", var!(border));
        z-index: "201";
        overflow-y: "auto";
        padding: var!(space-xl);
        transform: "translateX(-100%)";
        transition: format!("transform {} {}", var!(duration-normal), var!(ease-out));
    }
    pub(crate) c_docs_mobile_drawer_open {
        transform: "translateX(0px)";
    }
}
