use std::path::PathBuf;

use crate::Result;

pub struct Item {
    pub path: PathBuf,
    pub content: String,
}

pub trait Compiler {
    fn compile(&self, item: Item) -> Result<Item>;

    fn then<C: Compiler>(self, next: C) -> Then<Self, C>
        where Self: Sized 
    {
        Then(self, next)
    }
}

impl <F> Compiler for F
    where F: Fn(Item) -> Result<Item>
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
