#[derive(Debug, PartialEq, Clone)]
enum Token {
    Darja,          // 'darja' keyword (class)
    Tareeqa,        // 'tareeqa' keyword (method)
    SubhKoBataDy,         // 'subhkobatady' keyword (public)
    SubhKoChupaDy,           // 'subhKochupady' keyword (private)
    Hamesha,        // 'hamesha' keyword (static)
    Naya,           // 'naya' keyword (new)
    Agar,           // 'agar' keyword (if)
    Warna,          // 'warna' keyword (else)
    Jabtak,         // 'jabtak' keyword (while)
    Wapas,          // 'wapas' keyword (return)
    Var,            // 'var' keyword (var declaration)
    Null,           // 'null'
    Sahi,           // 'true'
    Ghalat,         // 'false'
    Barabar,        // 'barabar' (equals `=`)
    Chota,          // 'chota' (less than `<`)
    Bara,           // 'bara' (greater than `>`)
    Aur,            // 'aur' (and `&&`)
    Ya,             // 'ya' (or `||`)
    Super,          // 'super' (for method overriding)
    Identifier(String), // variable names
    Number(i64),    // numbers
    StringLiteral(String), // string literals
    LParen,         // '('
    RParen,         // ')'
    LBrace,         // '{'
    RBrace,         // '}'
    Comma,          // ','
    Semicolon,      // ';'
    Dot,            // '.'
    EOF,            // End of file
}

impl Lexer {
    // Update the `next_token()` function to recognize new keywords for OOP concepts
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            Some('(') => { self.next_char(); Token::LParen }
            Some(')') => { self.next_char(); Token::RParen }
            Some('{') => { self.next_char(); Token::LBrace }
            Some('}') => { self.next_char(); Token::RBrace }
            Some(',') => { self.next_char(); Token::Comma }
            Some(';') => { self.next_char(); Token::Semicolon }
            Some('.') => { self.next_char(); Token::Dot }
            Some(c) if c.is_digit(10) => self.read_number(),
            Some('"') => self.read_string_literal(),
            Some(_) => self.read_identifier_or_keyword(),
            None => Token::EOF,
        }
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start_position = self.position;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                self.next_char();
            } else {
                break;
            }
        }

        let ident = &self.input[start_position..self.position];
        match ident {
            "darja" => Token::Darja,
            "tareeqa" => Token::Tareeqa,
            "subhkobatady" => Token::SubhKoBataDy,
            "subhkochupady" => Token::SubhKoChupaDy,
            "hamesha" => Token::Hamesha,
            "naya" => Token::Naya,
            "agar" => Token::Agar,
            "warna" => Token::Warna,
            "jabtak" => Token::Jabtak,
            "wapas" => Token::Wapas,
            "var" => Token::Var,
            "null" => Token::Null,
            "sahi" => Token::Sahi,
            "ghalat" => Token::Ghalat,
            "barabar" => Token::Barabar,
            "chota" => Token::Chota,
            "bara" => Token::Bara,
            "aur" => Token::Aur,
            "ya" => Token::Ya,
            "super" => Token::Super,
            _ => Token::Identifier(ident.to_string()),
        }
    }
}
