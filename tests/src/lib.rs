#[cfg(test)]
#[macro_use]
extern crate idiom_core;

pub use idiom_core::*;

#[cfg(test)]
mod tests {

    #[test]
    fn macros() {
        assert_eq!(tokenize!(EOF), crate::TokenType::EOF)
    }

}
