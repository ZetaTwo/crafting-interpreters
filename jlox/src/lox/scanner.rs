
pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(line: String) -> Scanner {
        Scanner { source: line }
    }

    pub fn scan_tokens(&self) -> std::str::Split<char> {
        self.source.split(' ')
    }
}
