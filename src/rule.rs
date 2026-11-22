use std::{
    fs,
    path::{Path, PathBuf},
};

use globset::{Glob, GlobMatcher};

use crate::{
    Result,
    compiler::{Compiler, Item, copy},
    route::{Route, no_route},
};

pub struct Rule {
    pattern: GlobMatcher,
    route: Box<dyn Route>,
    compiler: Box<dyn Compiler>,
}

impl Rule {
    pub fn new(pattern: &str) -> Self {
        let glob = Glob::new(pattern).unwrap_or_else(|e| panic!("Invalid pattern {pattern}: {e}"));
        Rule {
            pattern: glob.compile_matcher(),
            route: Box::new(no_route()),
            compiler: Box::new(copy),
        }
    }
}

#[derive(Default)]
pub struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn matches(mut self, pattern: &str, wrap: impl FnOnce(Rule) -> Rule) -> Self {
        self.rules.push(wrap(Rule::new(pattern)));
        self
    }

    pub fn run(&self, src: &Path, dst: &Path) -> Result<()> {
        for file in walk(src)? {
            let normalized = file.strip_prefix(src).unwrap(); // Shall not fail, cause src is always a prefix

            if let Some(rule) = self.rules.iter().find(|r| r.pattern.is_match(normalized))
                && let Some(out) = rule.route.route(normalized)
            {
                let item = Item {
                    path: normalized.to_path_buf(),
                    content: fs::read_to_string(file)?,
                };
                let compiled = rule.compiler.compile(item)?;
                let out = dst.join(out);
                if let Some(parent) = out.parent()
                    && !fs::exists(parent)?
                {
                    fs::create_dir(parent)?;
                }

                fs::write(out, compiled.content)?;
            }
        }

        Ok(())
    }
}

fn walk(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let file_path = entry?.path();
        if file_path.is_dir() {
            files.append(&mut walk(&file_path)?);
        } else {
            files.push(file_path);
        }
    }
    Ok(files)
}
