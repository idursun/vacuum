use crate::application::context::Context;
use std::path::PathBuf;

impl Context for PathBuf {
    type Current = PathBuf;
    fn current(&self) -> Self::Current {
        self.clone()
    }
    fn home(&self) -> Self {
        dirs::home_dir().unwrap()
    }
    fn config(&self) -> Self {
        dirs::config_dir().unwrap()
    }

    fn local(&self) -> Self {
        dirs::data_local_dir().unwrap()
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
