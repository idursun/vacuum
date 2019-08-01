pub trait Ops {
    fn copy<S: AsRef<str>>(&self, file_name: S);
    fn copy_glob<S: AsRef<str>>(&self, pattern: S);
    fn execute<S: AsRef<str>>(&self, command: S);
}
