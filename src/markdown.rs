use pulldown_cmark::{Options, Parser, html};

use crate::{Result, compiler::Item};

pub fn markdown(mut item: Item) -> Result<Item> {
    let parser = Parser::new_ext(&item.content, Options::all());
    let mut out = String::new();
    html::push_html(&mut out, parser);
    item.content = out;
    Ok(item)
}
