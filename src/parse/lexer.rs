pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
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
}
