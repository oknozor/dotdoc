extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
use parser::vim::VimRcParser;
use parser::DotFileParser;

fn main() {
    let vimrc_parser = VimRcParser::new("/home/okno/.config/nvim/init.vim");
    println!("{:?}", vimrc_parser.into_ast());
}
