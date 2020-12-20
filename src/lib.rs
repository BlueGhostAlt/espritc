use std::fmt::Display;

use colored::Colorize;

pub enum ErrorKind {
    UnknownCharacter,
}

pub enum TokenKind {
    Bracket,
    Punctuation,
    Operator,
    Eof,
}

pub struct Error<'a, 'b> {
    lexeme: &'a str,
    line: usize,
    column: usize,
    context: &'a str,
    filename: &'b str,
    kind: ErrorKind,
}

#[allow(dead_code)]
pub struct Token<'a> {
    lexeme: &'a str,
    line: usize,
    column: usize,
    kind: TokenKind,
}

#[allow(dead_code)]
pub struct Tokenizer<'a, 'b> {
    source: &'a str,
    filename: &'b str,

    tokens: Vec<Token<'a>>,

    start: usize,
    current: usize,

    line: usize,
    column: usize,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            ErrorKind::UnknownCharacter => "unknown character",
        };

        write!(f, "{}", string)
    }
}

impl Display for Error<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}\n{} {}:{}:{}\n {}\n{} {} {}\n {}{}{}\n\n{}{}\n\n{}{}",
            "error[E0001]".bright_red(),
            ": ".bright_white(),
            format!("{}", self.kind).bright_white(),
            format!(
                "{:>line_length$}",
                "-->",
                line_length = self.line.to_string().len() + 3
            )
            .bright_cyan(),
            self.filename,
            self.line,
            self.column,
            format!(
                "{:>line_length$}",
                "|",
                line_length = self.line.to_string().len() + 1
            )
            .bright_cyan(),
            self.line.to_string().bright_cyan(),
            format!(
                "{:>line_length$}",
                "|",
                line_length = self.line.to_string().len()
            )
            .bright_cyan(),
            self.context,
            format!(
                "{:>line_length$}",
                "|",
                line_length = self.line.to_string().len() + 1
            )
            .bright_cyan(),
            format!("{:>column$}", " ", column = self.column),
            format!("{:lexeme_length$}", "^", lexeme_length = self.lexeme.len()).bright_red(),
            "error".bright_red(),
            ": aborting due to 1 previous error".bright_white(),
            "error".bright_red(),
            format!(": could not tokenize `{}`", self.filename).bright_white()
        )
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            TokenKind::Bracket => "Bracket",
            TokenKind::Punctuation => "Punctuation",
            TokenKind::Operator => "Operator",
            TokenKind::Eof => "EOF",
        };

        write!(f, "{}", string)
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ lexeme: \"{}\", position: ({}, {}), token_type: {} }}",
            self.lexeme, self.line, self.column, self.kind
        )
    }
}

impl<'a, 'b> Error<'a, 'b> {
    pub fn new(
        lexeme: &'a str,
        line: usize,
        column: usize,
        context: &'a str,
        filename: &'b str,
        kind: ErrorKind,
    ) -> Error<'a, 'b> {
        Error {
            lexeme,
            line,
            column,
            context,
            filename,
            kind,
        }
    }
}

impl<'a, 'b> Tokenizer<'a, 'b> {
    pub fn new(source: &'a str, filename: &'b str) -> Tokenizer<'a, 'b> {
        Tokenizer {
            source,
            filename,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn scan_tokens(&'a mut self) -> Result<&'a [Token<'a>], Error<'a, 'b>> {
        while !self.has_reached_eof() {
            self.start = self.current;

            self.scan_token()?
        }

        self.tokens.push(Token {
            lexeme: "",
            line: self.line,
            column: 0,
            kind: TokenKind::Eof,
        });

        Ok(&self.tokens)
    }

    fn has_reached_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), Error<'a, 'b>> {
        let character = self.advance(1);

        let result = match character {
            "(" | ")" => self.add_token(TokenKind::Bracket),
            "{" => self.add_token(TokenKind::Bracket),
            "}" => self.add_token(TokenKind::Bracket),
            "<" | ">" => {
                let token_type = if self.match_next('=', false) {
                    TokenKind::Operator
                } else {
                    TokenKind::Bracket
                };
                self.add_token(token_type)
            }
            "," | "." | ";" => self.add_token(TokenKind::Punctuation),
            "-" => {
                if self.match_next('-', false) {
                    self.read_while(|c| c.ne(&'\n'));
                } else {
                    self.add_token(TokenKind::Operator)
                }
            }

            "+" | "*" | "/" | "!" => self.add_token(TokenKind::Operator),
            "=" => {
                let token_type = if self.match_next('=', false) {
                    TokenKind::Operator
                } else {
                    TokenKind::Bracket
                };
                self.add_token(token_type)
            }
            " " | "\t" => self.column += 1,

            "\r" => {}
            "\n" => {
                self.column = 1;
                self.line += 1;
            }
            _ => Err(self.boo(character))?,
        };

        Ok(result)
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

    fn add_token(&mut self, kind: TokenKind) {
        let lexeme = &self.source[self.start..self.current];

        let token = Token {
            lexeme,
            line: self.line,
            column: self.column,
            kind,
        };

        self.column += lexeme.len();
        self.tokens.push(token);
    }

    fn boo(&self, lexeme: &'a str) -> Error<'a, 'b> {
        let line = self.source.lines().nth(self.line - 1).unwrap();

        Error::new(
            lexeme,
            self.line,
            self.column,
            line,
            self.filename,
            ErrorKind::UnknownCharacter,
        )
    }
}

pub fn run<'a>(tokenizer: &'a mut Tokenizer<'a, 'a>) -> Result<&'a [Token<'a>], Error<'a, 'a>> {
    tokenizer.scan_tokens()
}
