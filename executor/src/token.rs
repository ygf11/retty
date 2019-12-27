use mio::Token;

pub struct Tokens {
    num: usize,
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
            num: usize::min_value(),
        }
    }

    pub fn next(&mut self) -> Token {

        let mut count = self.num;
        // avoid over flow
        if count == usize::max_value() {
            count = usize::min_value();
        }

        let token = Token(count);

        count = count + 1;

        self.num = count;

        token
    }
}