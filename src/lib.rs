use crate::lexer::Token;
use logos::Logos;

mod lexer;

pub fn lex(input: &str) -> Vec<Token> {
    Token::lexer(input).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_five_plus_five_whitespace() {
        let input = "5 + 5";
        let tokens = lex(input);
        assert_eq!(
            tokens,
            vec![Token::Number(5.0), Token::Plus, Token::Number(5.0)]
        );
    }

    #[test]
    fn lex_five_plus_five_no_whitespace() {
        let input = "5+5";
        let tokens = lex(input);
        assert_eq!(
            tokens,
            vec![Token::Number(5.0), Token::Plus, Token::Number(5.0)]
        );
    }
}
