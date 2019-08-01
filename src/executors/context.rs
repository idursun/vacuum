use std::path::PathBuf;

pub trait Context: Sized {
    fn home(&self) -> Self;
    fn config(&self) -> Self;
    fn sub<S: AsRef<str>>(&self, sub: S) -> Self;
    fn search(&self, pattern: &str) -> Vec<Self>;
}

impl Context for PathBuf {
    fn home(&self) -> Self {
        dirs::home_dir().unwrap()
    }

    fn config(&self) -> Self {
        dirs::config_dir().unwrap()
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let mut s = self.clone();
        s.push(sub.as_ref());
        s
    }

    fn search(&self, pattern: &str) -> Vec<Self> {
        let mut ret = vec![];
        let full_pattern = format!(
            "{}{}{}",
            self.to_str().unwrap(),
            std::path::MAIN_SEPARATOR,
            pattern
        );
        for entry in glob::glob(full_pattern.as_ref()).unwrap() {
            if let Ok(path) = entry {
                ret.push(path);
            }
        }
        ret
    }
}
