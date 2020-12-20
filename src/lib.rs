use std::fmt::Display;

pub enum TokenType {
    BRACKET,
    PUNCTUATION,
    OPERATOR,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            TokenType::BRACKET => "Bracket",
            TokenType::PUNCTUATION => "Punctuation",
            TokenType::OPERATOR => "Operator",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", string)
    }
}

#[allow(dead_code)]
pub struct Token<'a> {
    lexeme: &'a str,
    line: i32,
    column: i32,
    token_type: TokenType,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ lexeme: \"{}\", position: ({}, {}), token_type: {} }}",
            self.lexeme, self.line, self.column, self.token_type
        )
    }
}

#[allow(dead_code)]
pub struct Tokenizer<'a> {
    source: &'a str,

    tokens: Vec<Token<'a>>,

    start: usize,
    current: usize,

    line: i32,
    column: i32,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token<'a>> {
        while !self.has_reached_eof() {
            self.start = self.current;

            self.scan_token();
        }

        self.tokens.push(Token {
            lexeme: "",
            line: self.line,
            column: 0,
            token_type: TokenType::EOF,
        });

        &self.tokens
    }

    fn has_reached_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let character = self.advance(1);

        match character {
            "(" | ")" => self.add_token(TokenType::BRACKET),
            "{" => self.add_token(TokenType::BRACKET),
            "}" => self.add_token(TokenType::BRACKET),
            "<" | ">" => {
                let token_type = if self.match_next('=', false) {
                    TokenType::OPERATOR
                } else {
                    TokenType::BRACKET
                };
                self.add_token(token_type)
            }
            "," | "." | ";" => self.add_token(TokenType::PUNCTUATION),
            "-" => {
                if self.match_next('-', false) {
                    self.read_while(|c| c.ne(&'\n'));
                } else {
                    self.add_token(TokenType::OPERATOR)
                }
            }
            "+" | "*" | "/" | "!" => self.add_token(TokenType::OPERATOR),
            "=" => {
                let token_type = if self.match_next('=', false) {
                    TokenType::OPERATOR
                } else {
                    TokenType::BRACKET
                };
                self.add_token(token_type)
            }
            " " | "\t" => self.column += 1,
            "\r" => {}
            "\n" => {
                self.column = 1;
                self.line += 1
            }
            _ => eprintln!("Unexpected character: {}", character),
        }
    }

    fn advance(&mut self, advance_by: usize) -> &'a str {
        self.current += advance_by;

        &self.source[self.current - advance_by..self.current]
    }

    fn peek(&self) -> char {
        if self.has_reached_eof() {
            return '\0';
        }

        self.source.as_bytes()[self.current] as char
    }

    fn match_next(&mut self, expected: char, lowercase: bool) -> bool {
        if self.has_reached_eof() {
            return false;
        }

        let character = self.source.as_bytes()[self.current] as char;
        if !lowercase && character != expected {
            return false;
        }
        if lowercase && character.to_lowercase().eq(expected.to_lowercase()) {
            return false;
        }

        self.current += 1;
        true
    }

    fn read_while<P>(&mut self, predicate: P)
    where
        P: Fn(char) -> bool,
    {
        while predicate(self.peek()) && !self.has_reached_eof() {
            self.advance(1);
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];

        let token = Token {
            lexeme,
            line: self.line,
            column: self.column,
            token_type,
        };

        self.column += lexeme.len() as i32;
        self.tokens.push(token);
    }
}

pub fn run(source: &str) {
    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}
