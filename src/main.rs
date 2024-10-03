mod lexer;


fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod tests {
    #![allow(unused, warnings)]
    use crate::lexer::{Lexer, Token, TokenType, TokenValue};
    use super::*;

    impl PartialEq for TokenType {
        fn eq(&self, other: &Self) -> bool {
            self.clone() == *other
        }
    }

    impl PartialEq for TokenValue {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (TokenValue::Bool(a), TokenValue::Bool(b)) => a == b,
                (TokenValue::Char(a), TokenValue::Char(b)) => a == b,
                (TokenValue::Int(a), TokenValue::Int(b)) => a == b,
                (TokenValue::Float(a), TokenValue::Float(b)) => a == b,
                (TokenValue::String(a), TokenValue::String(b)) => a == b,
                _ => false, // Different variants are not equal
            }
        }
    }


    impl PartialEq for Token {
        fn eq(&self, other: &Self) -> bool {
            self.kind == other.kind && self.value == other.value
        }
    }


    #[test]
    fn test_lexer_identifiers() {
        let code = "let x = 10;
                   const y = 'a';
                   enum Color { Red, Green, Blue };
                   struct Point { x: i32, y: i32 };
                   fun add(a: i32, b: i32) -> i32 { a + b };
                   namespace MyNamespace { ... };";

        let mut lexer = Lexer::new("test.txt".to_string(), code.to_string());

        assert_eq!(lexer.next_token(), Token::new(TokenType::Let, TokenValue::String("let".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("x".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Assign, TokenValue::String("=".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::IntVal, TokenValue::Int(10), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::SemiColon, TokenValue::String(";".to_string()), 1));

        assert_eq!(lexer.next_token(), Token::new(TokenType::Const, TokenValue::String("const".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("y".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Assign, TokenValue::String("=".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::CharVal, TokenValue::Char('a'), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::SemiColon, TokenValue::String(";".to_string()), 2));

        assert_eq!(lexer.next_token(), Token::new(TokenType::Enum, TokenValue::String("enum".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("Color".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LeftBrace, TokenValue::String("{".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("Red".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Comma, TokenValue::String(",".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("Green".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Comma, TokenValue::String(",".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("Blue".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RightBrace, TokenValue::String("}".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::SemiColon, TokenValue::String(";".to_string()), 3));

        assert_eq!(lexer.next_token(), Token::new(TokenType::Struct, TokenValue::String("struct".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("Point".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LeftBrace, TokenValue::String("{".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("x".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Colon, TokenValue::String(":".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("i32".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Comma, TokenValue::String(",".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("y".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Colon, TokenValue::String(":".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("i32".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RightBrace, TokenValue::String("}".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::SemiColon, TokenValue::String(";".to_string()), 4));

        assert_eq!(lexer.next_token(), Token::new(TokenType::Fun, TokenValue::String("fun".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("add".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LeftParen, TokenValue::String("(".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("a".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Colon, TokenValue::String(":".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("i32".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Comma, TokenValue::String(",".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("b".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Colon, TokenValue::String(":".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("i32".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RightParen, TokenValue::String(")".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("i32".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LeftBrace, TokenValue::String("{".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("a".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Plus, TokenValue::String("+".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("b".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RightBrace, TokenValue::String("}".to_string()), 5));
        assert_eq!(lexer.next_token(), Token::new(TokenType::SemiColon, TokenValue::String(";".to_string()), 5));

        assert_eq!(lexer.next_token(), Token::new(TokenType::Namespace, TokenValue::String("namespace".to_string()), 6));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Identifier, TokenValue::String("MyNamespace".to_string()), 6));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LeftBrace, TokenValue::String("{".to_string()), 6));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Error, TokenValue::String("...".to_string()), 6));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RightBrace, TokenValue::String("}".to_string()), 6));
        assert_eq!(lexer.next_token(), Token::new(TokenType::SemiColon, TokenValue::String(";".to_string()), 6));
    }

    #[test]
    fn test_lexer_numbers() {
        let code = "10
                   1234567890
                   3.14159
                   0.001
                   1.0";

        let mut lexer = Lexer::new("test.txt".to_string(), code.to_string());

        assert_eq!(lexer.next_token(), Token::new(TokenType::IntVal, TokenValue::Int(10), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::IntVal, TokenValue::Int(1234567890), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::FloatVal, TokenValue::Float(3.14159), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::FloatVal, TokenValue::Float(0.001), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::FloatVal, TokenValue::Float(1.0), 5));
    }

    #[test]
    fn test_lexer_strings() {
        let code = "\"Hello, world!\"
                   \"This is a string with spaces.\"";

        let mut lexer = Lexer::new("test.txt".to_string(), code.to_string());

        assert_eq!(lexer.next_token(), Token::new(TokenType::StringVal, TokenValue::String("Hello, world!".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::StringVal, TokenValue::String("This is a string with spaces.".to_string()), 2));
    }

    #[test]
    fn test_lexer_operators() {
        let code = "+ - * / % =
                   == != < > <= >=
                   || ! &&
                   | ~ & ^ << >>";

        let mut lexer = Lexer::new("test.txt".to_string(), code.to_string());

        assert_eq!(lexer.next_token(), Token::new(TokenType::Plus, TokenValue::String("+".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Minus, TokenValue::String("-".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Multiply, TokenValue::String("*".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Divide, TokenValue::String("/".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Modulus, TokenValue::String("%".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Assign, TokenValue::String("=".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Equal, TokenValue::String("==".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::NotEqual, TokenValue::String("!=".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LessThan, TokenValue::String("<".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::GreaterThan, TokenValue::String(">".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LessThanEqual, TokenValue::String("<=".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::GreaterThanEqual, TokenValue::String(">=".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LogicalOr, TokenValue::String("||".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LogicalNot, TokenValue::String("!".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LogicalAnd, TokenValue::String("&&".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::BitwiseOr, TokenValue::String("|".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::BitwiseNot, TokenValue::String("~".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::BitwiseAnd, TokenValue::String("&".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::BitwiseXor, TokenValue::String("^".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::BitwiseShiftLeft, TokenValue::String("<<".to_string()), 4));
        assert_eq!(lexer.next_token(), Token::new(TokenType::BitwiseShiftRight, TokenValue::String(">>".to_string()), 4));
    }

    #[test]
    fn test_lexer_delimiters() {
        let code = "( ) [ ] { } . , : ;";

        let mut lexer = Lexer::new("test.txt".to_string(), code.to_string());

        assert_eq!(lexer.next_token(), Token::new(TokenType::LeftParen, TokenValue::String("(".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RightParen, TokenValue::String(")".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LeftBracket, TokenValue::String("[".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RightBracket, TokenValue::String("]".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LeftBrace, TokenValue::String("{".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RightBrace, TokenValue::String("}".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Dot, TokenValue::String(".".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Comma, TokenValue::String(",".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Colon, TokenValue::String(":".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::SemiColon, TokenValue::String(";".to_string()), 1));
    }

    #[test]
    fn test_lexer_keywords() {
        let code = "let const enum struct fun
                   if elif else switch case default
                   for while do break continue return
                   namespace";

        let mut lexer = Lexer::new("test.txt".to_string(), code.to_string());

        assert_eq!(lexer.next_token(), Token::new(TokenType::Let, TokenValue::String("let".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Const, TokenValue::String("const".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Enum, TokenValue::String("enum".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Struct, TokenValue::String("struct".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Fun, TokenValue::String("fun".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::If, TokenValue::String("if".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Elif, TokenValue::String("elif".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Else, TokenValue::String("else".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Switch, TokenValue::String("switch".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Case, TokenValue::String("case".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Default, TokenValue::String("default".to_string()), 2));
        assert_eq!(lexer.next_token(), Token::new(TokenType::For, TokenValue::String("for".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::While, TokenValue::String("while".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Do, TokenValue::String("do".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Break, TokenValue::String("break".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Continue, TokenValue::String("continue".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Return, TokenValue::String("return".to_string()), 3));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Namespace, TokenValue::String("namespace".to_string()), 4));
    }

    #[test]
    fn test_lexer_eof() {
        let code = "";

        let mut lexer = Lexer::new("test.txt".to_string(), code.to_string());

        assert_eq!(lexer.next_token(), Token::new(TokenType::Eof, TokenValue::String("".to_string()), 1));
    }

    #[test]
    fn test_lexer_error() {
        let code = "1 + 2 * $";

        let mut lexer = Lexer::new("test.txt".to_string(), code.to_string());

        assert_eq!(lexer.next_token(), Token::new(TokenType::IntVal, TokenValue::Int(1), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Plus, TokenValue::String("+".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::IntVal, TokenValue::Int(2), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Multiply, TokenValue::String("*".to_string()), 1));
        assert_eq!(lexer.next_token(), Token::new(TokenType::Error, TokenValue::String("$".to_string()), 1));
    }
}