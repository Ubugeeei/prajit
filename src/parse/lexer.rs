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
