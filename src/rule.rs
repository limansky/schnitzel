use globset::{Glob, GlobMatcher};

use crate::{compiler::{Compiler, copy}, route::{Route, no_route}};

pub struct Rule {
    pattern: GlobMatcher,
    route: Box<dyn Route>,
    compiler: Box<dyn Compiler>
}

impl Rule {
    pub fn new(pattern: &str) -> Self {
        let glob = Glob::new(pattern).unwrap_or_else(|e| panic!("Invalid pattern {pattern}: {e}"));
        Rule { pattern: glob.compile_matcher(), route: Box::new(no_route()), compiler: Box::new(copy) }
    }
}
