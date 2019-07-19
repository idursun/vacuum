use std::path::PathBuf;

pub trait Context: Sized {
    fn home(&self) -> Self;
    fn config(&self) -> Self;
    fn sub<S: AsRef<str>>(&self, sub: S) -> Self;
    fn search(&self, pattern: &String) -> Vec<Self>;
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

    fn search(&self, pattern: &String) -> Vec<Self> {
        vec![PathBuf::from("A")]
    }
}

pub trait Ops {
    fn copy<S: AsRef<str>>(&self, file_name: S);
    fn copy_glob<S: AsRef<str>>(&self, pattern: S);
    fn execute<S: AsRef<str>>(&self, command: S);
}

pub(crate) type ContextPair<E> = (E, E);

impl Ops for ContextPair<PathBuf> {
    fn copy<S: AsRef<str>>(&self, file_name: S) {
        println!(
            "copying {} from {} to {}",
            file_name.as_ref(),
            self.0.display(),
            self.1.display()
        );
    }

    fn copy_glob<S: AsRef<str>>(&self, pattern: S) {
        println!(
            "copying files matching {} from {}",
            pattern.as_ref(),
            self.0.display()
        );
    }

    fn execute<S: AsRef<str>>(&self, command: S) {
        println!("executing '{}' in {}", command.as_ref(), self.0.display());
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

    fn search(&self, pattern: &String) -> Vec<Self> {
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
