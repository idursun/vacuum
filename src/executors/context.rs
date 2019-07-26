use std::path::PathBuf;
pub(crate) type ContextPair<E> = (E, E);

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
            match entry {
                Ok(path) => {
                    ret.push(path);
                }
                _ => (),
            }
        }
        ret
    }
}

impl Context for ContextPair<PathBuf> {
    fn home(&self) -> Self {
        (self.0.home(), PathBuf::from("home"))
    }

    fn config(&self) -> Self {
        (self.0.config(), PathBuf::from("config"))
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let (mut source, mut dest) = self.clone();
        source.push(sub.as_ref());
        dest.push(sub.as_ref());
        (source, dest)
    }

    fn search(&self, pattern: &str) -> Vec<Self> {
        let mut ret = vec![];
        let sources = self.0.search(pattern);
        for source in sources {
            let remaining = source.strip_prefix(self.0.as_path()).unwrap();
            let mut new_destination = self.1.clone();
            new_destination.push(remaining);
            ret.push((source, new_destination))
        }
        ret
    }
}
