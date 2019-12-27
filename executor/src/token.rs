use mio::Token;

struct Tokens{
    num:usize,
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
            num: 0
        }
    }

    pub fn next(&mut self) -> Token {
        // self.count().clone()
        let mut count = self.count;
        count = count + 1;

        self.count = count;

        Token(count)
    }
}