use crate::application::context::Context;
use std::path::PathBuf;

#[derive(Clone)]
pub struct TargetDirectoryContext {
    pub target: PathBuf,
}

impl TargetDirectoryContext {
    pub fn new(target_dir: PathBuf) -> Self {
        Self { target: target_dir }
    }
}

impl Context for TargetDirectoryContext {
    type Current = PathBuf;

    fn current(&self) -> Self::Current {
        self.target.clone()
    }

    fn home(&self) -> Self {
        Self {
            target: self.target.sub("home"),
        }
    }

    fn config(&self) -> Self {
        Self {
            target: self.target.sub("config"),
        }
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let sub = sub.as_ref();
        let target = self.target.sub(sub);

        Self { target }
    }

    fn search(&self, pattern: &str) -> Vec<Self> {
        let mut ret = Vec::new();
        let sources = self.target.search(pattern);
        for source in sources {
            let remaining = source.strip_prefix(self.target.as_path()).unwrap();
            let target = self.target.sub(remaining.to_str().unwrap());
            ret.push(Self { target })
        }
        ret
    }
}
