use std::path::{Path, PathBuf};

pub trait Route {
    fn route(&self, path: &Path) -> Option<PathBuf>;
}

impl<F> Route for F
where
    F: Fn(&Path) -> Option<PathBuf>,
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::route::{Route, no_route, set_extension};

    #[test]
    fn set_extension_test() {
        assert_eq!(
            set_extension("html").route(&PathBuf::from("docs.md")),
            Some(PathBuf::from("docs.html"))
        );
        assert_eq!(
            set_extension("xml").route(&PathBuf::from("about/cv")),
            Some(PathBuf::from("about/cv.xml"))
        );
    }

    #[test]
    fn no_route_test() {
        assert_eq!(no_route().route(&PathBuf::from("docs,md")), None);
    }
}
