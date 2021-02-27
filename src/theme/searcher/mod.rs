//! Theme dependencies for in-browser search. Not included in mdbook when
//! the "search" cargo feature is disabled.

pub static JS: &[u8] = include_bytes!("searcher.js");
pub static MARK_JS: &[u8] = include_bytes!("mark.min.js");
pub static ELASTICLUNR_JS: &[u8] = include_bytes!("elasticlunr.min.js");

/// 中文的支持
pub static ZH: &[u8] = include_bytes!("zh.js");