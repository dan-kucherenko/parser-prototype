use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let parsed_data = Grammar::parse(Rule::field, "-271.23")?;
    dbg!(parsed_data);

    Ok(())
}