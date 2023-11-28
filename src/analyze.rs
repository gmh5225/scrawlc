use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "scrawl.pest"]
pub struct Analyzer;
