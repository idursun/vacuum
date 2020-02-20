pub trait Context: Sized {
    type Current;
    fn current(&self) -> Self::Current;
    fn home(&self) -> Self;
    fn config(&self) -> Self;
    fn sub<S: AsRef<str>>(&self, sub: S) -> Self;
    fn search(&self, pattern: &str) -> Vec<Self>;
}
