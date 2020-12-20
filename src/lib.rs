pub enum TokenType {
    BRACKET,
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            TokenType::BRACKET => "Bracket",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", string)
    }
}

#[allow(dead_code)]
pub struct Token<'a> {
    lexeme: &'a str,
    line: i32,
    token_type: TokenType,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ lexeme: \"{}\", line: {}, token_type: {} }}",
            self.lexeme, self.line, self.token_type
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
            token_type: TokenType::EOF,
        });

        &self.tokens
    }

    fn has_reached_eof(&self) -> bool {
        println!("current: {}, len(): {}", self.current, self.source.len());
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let character = self.advance(1);

        match character {
            "(" | ")" => self.add_token(TokenType::BRACKET),
            "{" => self.add_token(TokenType::BRACKET),
            "}" => self.add_token(TokenType::BRACKET),
            "<" | ">" => self.add_token(TokenType::BRACKET),
            " " | "\r" | "\t" => {}
            "\n" => self.line += 1,
            _ => eprintln!("Unexpected character: {}", character),
        }
    }

    fn advance(&mut self, advance_by: usize) -> &'a str {
        self.current += advance_by;

        &self.source[self.current - advance_by..self.current]
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];

        let token = Token {
            lexeme,
            line: self.line,
            token_type,
        };

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
