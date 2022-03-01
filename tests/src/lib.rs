#[cfg(test)]
#[macro_use]
extern crate idiom_core;

pub use idiom_core::*;

#[cfg(test)]
mod tests {

    #[test]
    fn type_eof() {
        assert_eq!(tokenize!(EOF), crate::TokenType::EOF)
    }

    #[test]
    fn type_delimitters() {
        assert_eq!(tokenize!(Delimit ',' (Open 0)), crate::TokenType::Delimiters{ raw: ',', kind: crate::DelimitersKind::Opening(0) });
        assert_eq!(tokenize!(Delimit '-' (Close 0)), crate::TokenType::Delimiters{ raw: '-', kind: crate::DelimitersKind::Closing(0) });
        assert_eq!(tokenize!(Delimit '|' (Open 0)), crate::TokenType::Delimiters{ raw: '|', kind: crate::DelimitersKind::Opening(0) });
        assert_eq!(tokenize!(Delimit '_' (Close 0)), crate::TokenType::Delimiters{ raw: '_', kind: crate::DelimitersKind::Closing(0) });
    }

    #[test]
    fn type_numerics() {
        assert_eq!(tokenize!(Num ("634".to_string()) Den WholeNo), 
                   crate::TokenType::Numeric{ 
                       raw: "634".to_string(), 
                       base: crate::NumericBase::Decimal, 
                       kind: crate::NumericKind::Whole 
                   });

        assert_eq!(tokenize!(Num ("1011010".to_string()) Bin WholeNo), 
                   crate::TokenType::Numeric{ 
                       raw: "1011010".to_string(), 
                       base: crate::NumericBase::Binary, 
                       kind: crate::NumericKind::Whole 
                   });

        assert_eq!(tokenize!(Num ("9F3204AC".to_string()) Hex WholeNo), 
                   crate::TokenType::Numeric{ 
                       raw: "9F3204AC".to_string(), 
                       base: crate::NumericBase::Hexadecimal, 
                       kind: crate::NumericKind::Whole 
                   });
        assert_eq!(tokenize!(Num ("/.^7HU2,".to_string()) B64 WholeNo), 
                   crate::TokenType::Numeric{ 
                       raw: "/.^7HU2,".to_string(), 
                       base: crate::NumericBase::Base64, 
                       kind: crate::NumericKind::Whole 
                   });
        assert_eq!(tokenize!(Num ("1e+194".to_string()) Den Exponent), 
                   crate::TokenType::Numeric{ 
                       raw: "1e+194".to_string(), 
                       base: crate::NumericBase::Decimal, 
                       kind: crate::NumericKind::Exponential 
                   });
        assert_eq!(tokenize!(Num ("0.11235".to_string()) Den Fraction), 
                   crate::TokenType::Numeric{ 
                       raw: "0.11235".to_string(), 
                       base: crate::NumericBase::Decimal, 
                       kind: crate::NumericKind::Fractional
                   });
        assert_eq!(tokenize!(Num ("3.14".to_string()) Den Fraction),                   
                   crate::TokenType::Numeric{ 
                       raw: "3.14".to_string(), 
                       base: crate::NumericBase::Decimal, 
                       kind: crate::NumericKind::Fractional
                   });
        assert_eq!(tokenize!(Num ("True".to_string()) Bin Boolean), 
                   crate::TokenType::Numeric{ 
                       raw: "True".to_string(), 
                       base: crate::NumericBase::Binary, 
                       kind: crate::NumericKind::Bool
                   });
        assert_eq!(tokenize!(Num ("0".to_string()) Bin Boolean),
                   crate::TokenType::Numeric{ 
                       raw: "0".to_string(), 
                       base: crate::NumericBase::Binary, 
                       kind: crate::NumericKind::Bool
                   });
    }

    #[test]
    fn type_characters() {
        assert_eq!(tokenize!(Char 'c'), crate::TokenType::Character('c'))
    }
}
