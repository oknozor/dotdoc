pub(crate) mod vim;

use pest::iterators::Pair;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub trait DotFileParser<'a> {
    type PairItem;
    type Ast;

    fn new(path: &str) -> Self;

    fn parse_file(&'a self) -> Self::PairItem;

    fn into_ast(&self) -> Self::Ast;
}

fn get_file_content(path: &str) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
