//! Past-session search contract validation for StealthEye Cloud S4.

pub const SEARCH_PACKET_SCHEMAS: &[&str] = &[
    "PastSessionSearchV0",
    "SearchCorpusManifestV0",
    "SearchResultCardV0",
    "SearchImportDecisionV0",
];

pub const SEARCH_CORPUS_NAMES: &[&str] = &[
    "relay",
    "seal",
    "active",
    "decisions",
    "skills",
    "worker-results",
    "browser-artifacts",
    "ci-proof",
    "docs",
];

pub fn is_search_schema(name: &str) -> bool {
    SEARCH_PACKET_SCHEMAS.contains(&name)
}

pub fn is_allowed_corpus(name: &str) -> bool {
    SEARCH_CORPUS_NAMES.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_past_session_search_schema() {
        assert!(is_search_schema("PastSessionSearchV0"));
    }

    #[test]
    fn includes_relay_and_seal_corpora() {
        assert!(is_allowed_corpus("relay"));
        assert!(is_allowed_corpus("seal"));
    }
}
