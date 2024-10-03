use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    Eof,

    BoolVal,
    CharVal,
    IntVal,
    FloatVal,
    StringVal,
    Identifier,

    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,

    Assign,

    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessThanEqual,
    GreaterThanEqual,

    LogicalOr,
    LogicalNot,
    LogicalAnd,

    BitwiseOr,
    BitwiseNot,
    BitwiseAnd,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Colon,
    SemiColon,

    Let,
    Const,
    Enum,
    Struct,
    Fun,
    Namespace,

    If,
    Elif,
    Else,
    Switch,
    Case,
    Default,
    For,
    While,
    Do,
    Break,
    Continue,
    Return,

    Error
}

#[derive(Clone, Debug)]
pub enum TokenValue {
    Bool(bool),
    Char(char),
    Int(i64),
    Float(f64),
    String(String)
}

#[derive(Clone, Debug)]
pub struct Token {
    pub(crate) kind: TokenType,
    pub(crate) value: TokenValue,
    line: u16
}

impl Token {
    pub fn new(kind: TokenType, value: TokenValue, line: u16) -> Self {
        Self {
            kind, value, line
        }
    }
}

pub struct Lexer {
    path: String,
    code: String,
    index: usize,
    line: u16
}

lazy_static! {
    static ref KEYWORD_MAP: HashMap<&'static str, TokenType> = {
        let mut keyword_map = HashMap::new();
        keyword_map.insert("let", TokenType::Let);
        keyword_map.insert("const", TokenType::Const);
        keyword_map.insert("enum", TokenType::Enum);
        keyword_map.insert("struct", TokenType::Struct);
        keyword_map.insert("fun", TokenType::Fun);

        keyword_map.insert("if", TokenType::If);
        keyword_map.insert("elif", TokenType::Elif);
        keyword_map.insert("else", TokenType::Else);
        keyword_map.insert("switch", TokenType::Switch);
        keyword_map.insert("case", TokenType::Case);
        keyword_map.insert("default", TokenType::Default);
        keyword_map.insert("for", TokenType::For);
        keyword_map.insert("while", TokenType::While);
        keyword_map.insert("do", TokenType::Do);
        keyword_map.insert("break", TokenType::Break);
        keyword_map.insert("continue", TokenType::Continue);
        keyword_map.insert("return", TokenType::Return);

        keyword_map
    };

    static ref OPERATOR_MAP: HashMap<&'static str, TokenType> = {
        let mut operator_map = HashMap::new();
        operator_map.insert("+", TokenType::Plus);
        operator_map.insert("-", TokenType::Minus);
        operator_map.insert("*", TokenType::Multiply);
        operator_map.insert("/", TokenType::Divide);
        operator_map.insert("%", TokenType::Modulus);

        operator_map.insert("=", TokenType::Assign);

        operator_map.insert("||", TokenType::LogicalOr);
        operator_map.insert("!", TokenType::LogicalNot);
        operator_map.insert("&&", TokenType::LogicalAnd);

        operator_map.insert("<", TokenType::LessThan);
        operator_map.insert(">", TokenType::GreaterThan);
        operator_map.insert("==", TokenType::Equal);
        operator_map.insert("!=", TokenType::NotEqual);
        operator_map.insert("<=", TokenType::LessThanEqual);
        operator_map.insert(">=", TokenType::GreaterThanEqual);

        operator_map.insert("|", TokenType::BitwiseOr);
        operator_map.insert("~", TokenType::BitwiseNot);
        operator_map.insert("&", TokenType::BitwiseAnd);
        operator_map.insert("^", TokenType::BitwiseXor);
        operator_map.insert("<<", TokenType::BitwiseShiftLeft);
        operator_map.insert(">>", TokenType::BitwiseShiftRight);

        operator_map
    };

    static ref DELIMITER_MAP: HashMap<&'static str, TokenType> = {
        let mut delimiter_map = HashMap::new();
        delimiter_map.insert("(", TokenType::LeftParen);
        delimiter_map.insert(")", TokenType::RightParen);
        delimiter_map.insert("[", TokenType::LeftBracket);
        delimiter_map.insert("]", TokenType::RightBracket);
        delimiter_map.insert("{", TokenType::LeftBrace);
        delimiter_map.insert("}", TokenType::RightBrace);
        delimiter_map.insert(".", TokenType::Dot);
        delimiter_map.insert(",", TokenType::Comma);
        delimiter_map.insert(":", TokenType::Colon);
        delimiter_map.insert(";", TokenType::SemiColon);

        delimiter_map
    };

    static ref OPERATORS: &'static str = "+-*/%=<>|&^~";
    static ref DELIMITERS: &'static str = "()[]{}.,:;";
}

impl Lexer {
    pub fn new(path: String, code: String) -> Self {
        Self {
            path, code, index: 0, line: 1
        }
    }

    fn advance(&mut self) {
        if self.index < self.code.len() {
            if self.code.chars().nth(self.index).unwrap() == '\n' {
                self.line += 1;
            }

            self.index += 1;
        }
    }

    fn get_identifier(&mut self) -> Token {
        let mut start = self.index;

        while self.code.chars().nth(self.index).unwrap().is_alphabetic() {
            self.advance();
        }

        let identifier = &self.code[start..self.index];
        let kind = KEYWORD_MAP.get(identifier).cloned().unwrap_or(TokenType::Identifier);
        Token::new(kind, TokenValue::String(identifier.to_string()), self.line)
    }

    fn get_number(&mut self) -> Token {
        let mut start = self.index;

        while self.code.chars().nth(self.index).unwrap().is_digit(10) {
            self.advance();
        }

        if self.code.chars().nth(self.index).unwrap() == '.' {
            self.advance();

            while self.code.chars().nth(self.index).unwrap().is_digit(10) {
                self.advance();
            }

            let number = &self.code[start..self.index];
            return Token::new(TokenType::FloatVal, TokenValue::Float(number.parse::<f64>().unwrap()), self.line);
        }

        let number = &self.code[start..self.index];
        Token::new(TokenType::IntVal, TokenValue::Int(number.parse::<i64>().unwrap()), self.line)
    }

    fn get_string(&mut self) -> Token {
        self.advance(); // Skip opening quote

        let mut start = self.index;

        while self.code.chars().nth(self.index).unwrap() != '"' {
            self.advance();
        }

        let string = &self.code[start..self.index];
        Token::new(TokenType::StringVal, TokenValue::String(string.to_string()), self.line)
    }

    fn get_operator(&mut self) -> Token {
        let mut start = self.index;

        while OPERATORS.contains(self.code.chars().nth(self.index).unwrap()) {
            self.advance();
        }

        let operator = &self.code[start..self.index];

        let kind = match operator {
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "*" => TokenType::Multiply,
            "/" => TokenType::Divide,
            "%" => TokenType::Modulus,
            "=" => TokenType::Assign,
            "==" => TokenType::Equal,
            "!=" => TokenType::NotEqual,
            "<" => TokenType::LessThan,
            ">" => TokenType::GreaterThan,
            "<=" => TokenType::LessThanEqual,
            ">=" => TokenType::GreaterThanEqual,
            "||" => TokenType::LogicalOr,
            "!" => TokenType::LogicalNot,
            "&&" => TokenType::LogicalAnd,
            "|" => TokenType::BitwiseOr,
            "~" => TokenType::BitwiseNot,
            "&" => TokenType::BitwiseAnd,
            "^" => TokenType::BitwiseXor,
            "<<" => TokenType::BitwiseShiftLeft,
            ">>" => TokenType::BitwiseShiftRight,
            _ => unreachable!()
        };

        Token::new(kind, TokenValue::String(operator.to_string()), self.line)
    }

    fn get_delimiter(&mut self) -> Token {
        let mut start = self.index;

        while DELIMITERS.contains(self.code.chars().nth(self.index).unwrap()) {
            self.advance();
        }

        let delimiter = &self.code[start..self.index];

        let kind = match delimiter {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "[" => TokenType::LeftBracket,
            "]" => TokenType::RightBracket,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "." => TokenType::Dot,
            "," => TokenType::Comma,
            ":" => TokenType::Colon,
            ";" => TokenType::SemiColon,
            _ => unreachable!()
        };

        Token::new(kind, TokenValue::String(delimiter.to_string()), self.line)
    }

    pub fn next_token(&mut self) -> Token {
        if self.index >= self.code.len() {
            return Token::new(TokenType::Eof, TokenValue::String("".to_string()), self.line);
        }

        let current_char = self.code.chars().nth(self.index).unwrap();

        if current_char.is_whitespace() {
            self.advance();
            return self.next_token();
        }

        if current_char.is_alphabetic() {
            return self.get_identifier();
        }

        if current_char.is_digit(10) {
            return self.get_number();
        }

        if current_char == '"' {
            return self.get_string();
        }

        if OPERATORS.contains(current_char) {
            return self.get_operator();
        }

        if DELIMITERS.contains(current_char) {
            return self.get_delimiter();
        }

        Token::new(TokenType::Error, TokenValue::String(format!("Unexpected character: {}", current_char)), self.line)
    }
}