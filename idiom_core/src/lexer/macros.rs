
#[macro_export]
macro_rules! call_delimiter_kind {
    (Seperator) => {
        idiom_core::DelimitersKind::Seperator
    };

    (Open $depth:expr) => {
        idiom_core::DelimitersKind::Opening($depth)
    };

    (Close $depth:expr) => {
        idiom_core::DelimitersKind::Closing($depth)
    };
}

#[macro_export]
macro_rules! call_numeric_kind {
    (Boolean) => {
        idiom_core::NumericKind::Bool
    };
    (WholeNo) => {
        idiom_core::NumericKind::Whole
    };
    (Fraction) => {
        idiom_core::NumericKind::Fractional
    };
    (Exponent) => {
        idiom_core::NumericKind::Exponential
    };
}

#[macro_export]
macro_rules! call_numeric_base {
    (Bin) => {
        idiom_core::NumericBase::Binary 
    };
    (Oct) => {
        idiom_core::NumericBase::Octal
    };
    (Dec) => {
        idiom_core::NumericBase::Decimal
    };
    (Hex) => {
        idiom_core::NumericBase::Hexadecimal
    };
    (B64) => {
        idiom_core::NumericBase::Base64
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

    (Num $raw:tt $base:ident $kind:ident) => {
        idiom_core::TokenType::Numeric{ raw: $raw, base: call_numeric_base!($base), kind: call_numeric_kind!($kind)}
    };

    (Delimit $raw:tt ($($inner:tt)+)) => {
        idiom_core::TokenType::Delimiters{ raw: $raw, kind: call_delimiter_kind!($($inner) +)}
    };
}
