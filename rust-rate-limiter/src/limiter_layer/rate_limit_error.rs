#[derive(Debug, Default)]
pub struct RateLimitError(pub ());

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("Rate limited")
    }
}

impl std::error::Error for RateLimitError {}
