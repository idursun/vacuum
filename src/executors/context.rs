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
        vec![PathBuf::from("A")]
    }
}

impl Context for ContextPair<PathBuf> {
    fn home(&self) -> Self {
        (self.0.home(), PathBuf::new())
    }

    fn config(&self) -> Self {
        (self.0.config(), PathBuf::new())
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let (mut source, mut dest) = self.clone();
        source.push(sub.as_ref());
        dest.push(sub.as_ref());
        (source, dest)
    }

    fn search(&self, pattern: &str) -> Vec<Self> {
        let sources = self.0.search(pattern);
        let mut ret = vec![];
        for source in sources {
            let mut ndest = self.1.clone();
            ndest.push("A");
            ret.push((source, ndest))
        }
        ret
    }
}
