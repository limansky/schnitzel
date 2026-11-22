use std::path::PathBuf;

use crate::Result;

#[derive(Debug)]
pub struct Item {
    pub path: PathBuf,
    pub content: String,
}

pub trait Compiler {
    fn compile(&self, item: Item) -> Result<Item>;

    fn then<C: Compiler>(self, next: C) -> Then<Self, C>
    where
        Self: Sized,
    {
        Then(self, next)
    }
}

impl<F> Compiler for F
where
    F: Fn(Item) -> Result<Item>,
{
    fn compile(&self, item: Item) -> Result<Item> {
        self(item)
    }
}

pub struct Then<A, B>(A, B);

impl<A: Compiler, B: Compiler> Compiler for Then<A, B> {
    fn compile(&self, item: Item) -> Result<Item> {
        self.1.compile(self.0.compile(item)?)
    }
}

pub fn copy(item: Item) -> Result<Item> {
    Ok(item)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        Result,
        compiler::{Compiler, Item},
    };

    fn capitalize(mut item: Item) -> Result<Item> {
        item.content = item.content.to_uppercase();
        Ok(item)
    }

    fn reverse(mut item: Item) -> Result<Item> {
        item.content = item.content.chars().rev().collect();
        Ok(item)
    }

    #[test]
    fn then_test() {
        let item = Item {
            path: PathBuf::from("foo.md"),
            content: "Test me now".to_owned(),
        };
        assert_eq!(
            capitalize.then(reverse).compile(item).unwrap().content,
            "WON EM TSET"
        );
    }
}
