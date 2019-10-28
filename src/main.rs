extern crate pest;
#[macro_use]
extern crate pest_derive;

mod markdown;
mod parser;
use markdown::ToMarkDown;
use parser::vim::VimRcParser;
use parser::DotFileParser;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let vimrc_parser = VimRcParser::new("/home/okno/.config/nvim/init.vim");
    let ast = vimrc_parser.into_ast();
    let markdown = ast.to_markdown();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(markdown.as_bytes())?;

    Ok(())
}
