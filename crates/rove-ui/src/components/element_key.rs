//! Stable element names for the redlines and interaction tests.

/// A key for the `n`th element of a kind: the bare key for the first, `key#n` after.
pub fn nth(key: &str, n: usize) -> String {
    if n == 0 { key.to_string() } else { format!("{key}#{n}") }
}
