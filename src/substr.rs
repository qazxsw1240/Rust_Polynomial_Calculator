pub trait SubStr {
    fn substr(&self, start: usize, len: usize) -> String;
}

impl SubStr for String {
    fn substr(&self, start: usize, len: usize) -> String {
        self.chars().skip(start).take(len).collect::<String>()
    }
}
