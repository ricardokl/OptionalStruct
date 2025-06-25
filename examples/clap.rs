use clap::Parser;
use optional_struct::*;

#[optional_struct(OptionalFoo, true, true)]
#[derive(Debug)]
pub struct Foo {
    bar: char,
    baz: bool,
}

#[derive(Parser, Debug, serde::Deserialize, serde::Serialize)]
pub struct OptionalFoo {
    #[clap(long)]
    #[serde(default)]
    pub bar: Option<char>,

    #[clap(long)]
    #[serde(default)]
    pub baz: Option<bool>,
}

fn main() {
    let config_file = r#"{"bar":"a","baz":true}"#;
    let config_args: OptionalFoo = serde_json::from_str(config_file).unwrap();
    let cli_args = OptionalFoo::parse();
    println!("config: {config_args:?} cli: {cli_args:?}");

    // in conflict, cli_args wins
    let merged = config_args.apply(cli_args);
    println!("merged: {merged:?}");

    let args: Foo = merged.try_into().unwrap();
    println!("final {args:?}");
}
