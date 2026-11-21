use std::path::{Path, PathBuf};

pub trait Route {
    fn route(&self, path: &Path) -> Option<PathBuf>;
}

impl <F> Route for F
    where F: Fn(&Path) -> Option<PathBuf>
{
    fn route(&self, path: &Path) -> Option<PathBuf> {
        self(path)
    }
}

pub fn no_route() -> impl Route {
    |_: &Path| None
}

pub fn set_extension(ext: &str) -> impl Route {
    move |path: &Path| Some(path.with_extension(ext))
}
