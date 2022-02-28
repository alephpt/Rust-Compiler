
#[macro_export]
macro_rules! create_delimiter_kind {
    (Seperator) => {
        idiom_core::DelimitersKind::Seperator
    };

    (Opening $depth:expr) => {
        idiom_core::DelimitersKind::Opening($depth)
    };

    (Closing $depth:expr) => {
        idiom_core::DelimitersKind::Closing($depth)
    };
}

#[macro_export]
macro_rules! tokenize {
    (EOF) => {
        idiom_core::TokenType::EOF
    };

    (Char $raw:tt) => {
        idiom_core::TokenType::Character($raw)
    };

    (Delimit $raw:tt ($($inner:tt)+)) => {
        idiom_core::TokenType::Delimiters{ raw: $raw, kind: create_delimiter_kind($($inner) +)}
    };
}
