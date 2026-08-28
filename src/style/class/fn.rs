use super::super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════
    // Docs shell layout (component styles live in euv-ui)
    // ═══════════════════════════════════════════════════════════════════════

    pub(crate) c_docs_root {
        min-height: "100%";
        background: var!(background);
        color: var!(foreground);
    }

    // ── Layout columns ──────────────────────────────────────────────────────

    pub(crate) c_docs_body {
        display: "flex";
        min-width: "0px";
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
        @media ((max-width: 767px)) {
            display: "none";
        }
    }
    pub(crate) c_docs_main {
        flex: "1";
        min-width: "0px";
        margin-left: "260px";
        padding: format!("{} {}", var!(space-3xl), var!(padding-main-horizontal));
        display: "flex";
        justify-content: "center";
        @media ((max-width: 767px)) {
            margin-left: "0px";
            padding: format!("{} {}", var!(space-xl), var!(padding-main-horizontal-mobile));
        }
    }
    pub(crate) c_docs_main_home {
        flex: "1";
        min-width: "0px";
        padding: format!("{} {}", var!(space-3xl), var!(padding-main-horizontal));
        display: "flex";
        justify-content: "center";
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-xl), var!(padding-main-horizontal-mobile));
        }
    }
}
