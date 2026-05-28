use anyhow::{Context, Result};
use clap::Parser;
use std::fs::read_to_string;
use std::io::stdout;
use std::path::PathBuf;
use cli_grep_in_rust::find_matches;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    println!(
        "Pattern for Matching:\n\t{:?}\nPath to file:\n\t{:?}",
        args.pattern, args.path
    );

    let content = read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;
    find_matches(&content, &args.pattern, &mut stdout());
    Ok(())
}


#[test]
fn testing_find_matches() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
