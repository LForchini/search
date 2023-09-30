use std::process::Command;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short, default_value = "firefox")]
    browser: String,

    #[arg(long, short, default_value = "duckduckgo.com")]
    search_engine: String,

    #[arg(trailing_var_arg = true)]
    search_term: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let url = format!(
        "https://{}/?q={}",
        args.search_engine,
        args.search_term.join(" ")
    );
    let _ = Command::new(args.browser)
        .arg("--new-window")
        .arg(url)
        .output()?;

    Ok(())
}
