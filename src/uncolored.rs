// dirty method of satisfying the compiler when colored is disabled
// do not try this at home

pub trait Uncolored {
    fn white(self) -> String;
    fn red(self) -> String;
    fn green(self) -> String;
    fn blue(self) -> String;
    fn magenta(self) -> String;
    fn bold(self) -> String;
}

impl Uncolored for String {
    fn white(self) -> String { return self; }
    fn red(self) -> String { return self; }
    fn green(self) -> String { return self; }
    fn blue(self) -> String { return self; }
    fn magenta(self) -> String { return self; }
    fn bold(self) -> String { return self; }
}

impl Uncolored for &'static str {
    fn white(self) -> String { return self.to_string(); }
    fn red(self) -> String { return self.to_string(); }
    fn green(self) -> String { return self.to_string(); }
    fn blue(self) -> String { return self.to_string(); }
    fn magenta(self) -> String { return self.to_string(); }
    fn bold(self) -> String { return self.to_string(); }
}