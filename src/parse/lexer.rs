use super::token::Token;

pub struct Lexer {
    input: String,
    position: isize,
    read_position: isize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: -1,
            read_position: 0,
            ch: '\0',
        }
    }

    pub fn next(&mut self) -> Token {
        self.skip_white_space();
        self.read();

        match self.ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.digit(),
            '\0' => Token::EOF,
            _ => Token::Illegal,
        }
    }

    fn digit(&mut self) -> Token {
        let start_pos = self.position as usize;
        while Lexer::is_digit(self.ch) {
            self.read();
        }
        let literal = &self.input[start_pos..(self.position as usize)];
        let num = literal.parse::<f64>().unwrap();

        self.back();

        Token::Number(num)
    }

    fn read(&mut self) {
        if self.read_position >= self.input.len() as isize {
            self.ch = '\0';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position.try_into().unwrap())
                .unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&mut self) -> char {
        if self.read_position >= self.input.len() as isize {
            '\0'
        } else {
            self.input
                .chars()
                .nth(self.read_position.try_into().unwrap())
                .unwrap()
        }
    }

    fn back(&mut self) {
        self.position -= 1;
        self.read_position -= 1;
        self.ch = self
            .input
            .chars()
            .nth(self.position.try_into().unwrap())
            .unwrap()
    }

    fn skip_white_space(&mut self) {
        while self.peek() == ' '
            || self.peek() == '\t'
            || self.peek() == '\n'
            || self.peek() == '\r'
        {
            self.read();
        }
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9' || ch == '.'
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::parse::token::Token;

    #[test]
    fn test_lexer() {
        let input = String::from(
            r#" 1   + 
        23 + 1.5                  * 3 / 0 - 1 "#,
        );

        let mut l = Lexer::new(input);

        let got = l.next();
        assert_eq!(got, Token::Number(1.0));

        let got = l.next();
        assert_eq!(got, Token::Plus);

        let got = l.next();
        assert_eq!(got, Token::Number(23.0));

        let got = l.next();
        assert_eq!(got, Token::Plus);

        let got = l.next();
        assert_eq!(got, Token::Number(1.5));

        let got = l.next();
        assert_eq!(got, Token::Asterisk);

        let got = l.next();
        assert_eq!(got, Token::Number(3.0));

        let got = l.next();
        assert_eq!(got, Token::Slash);

        let got = l.next();
        assert_eq!(got, Token::Number(0.0));

        let got = l.next();
        assert_eq!(got, Token::Minus);

        let got = l.next();
        assert_eq!(got, Token::Number(1.0));

        let got = l.next();
        assert_eq!(got, Token::EOF);
    }
}
