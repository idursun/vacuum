use crate::application::context::Context;
use std::path::PathBuf;

#[derive(Clone)]
pub struct RestoreContext {
    pub source: PathBuf,
    pub target: PathBuf,
}

impl RestoreContext {
    pub fn new(source_dir: PathBuf) -> Self {
        Self {
            source: source_dir,
            target: PathBuf::default(),
        }
    }
}

impl Context for RestoreContext {
    type Current = (PathBuf, PathBuf);

    fn current(&self) -> Self::Current {
        let s = self.source.clone();
        let t = self.target.clone();
        (s, t)
    }

    fn home(&self) -> Self {
        Self {
            source: self.source.sub("home"),
            target: self.target.home(),
        }
    }

    fn config(&self) -> Self {
        Self {
            source: self.source.sub("config"),
            target: self.target.config(),
        }
    }

    fn local(&self) -> Self {
        Self {
            source: self.source.sub("local"),
            target: self.target.local(),
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
