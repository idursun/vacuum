use crate::application::context::Context;
use std::path::PathBuf;

#[derive(Clone)]
pub struct StoreContext {
    pub source: PathBuf,
    pub target: PathBuf,
}

impl StoreContext {
    pub fn new(target_dir: PathBuf) -> Self {
        Self {
            source: PathBuf::default(),
            target: target_dir,
        }
    }
}

impl Context for StoreContext {
    type Current = (PathBuf, PathBuf);

    fn current(&self) -> Self::Current {
        let s = self.source.clone();
        let t = self.target.clone();
        (s, t)
    }

    fn home(&self) -> Self {
        Self {
            source: self.source.home(),
            target: self.target.sub("home"),
        }
    }

    fn config(&self) -> Self {
        Self {
            source: self.source.config(),
            target: self.target.sub("config"),
        }
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let sub = sub.as_ref();
        let source = self.source.sub(sub);
        let target = self.target.sub(sub);

        Self { source, target }
    }

    fn search(&self, pattern: &str) -> Vec<Self> {
        let mut ret = Vec::new();
        let sources = self.source.search(pattern);
        for source in sources {
            let remaining = source.strip_prefix(self.source.as_path()).unwrap();
            let target = self.target.sub(remaining.to_str().unwrap());
            ret.push(Self { source, target })
        }
        ret
    }
}
