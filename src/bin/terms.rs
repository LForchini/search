use clap::Parser;
use thiserror::Error;
use wikipedia::Wikipedia;

#[derive(Debug, Parser)]
struct Args {
    #[arg()]
    term: Vec<String>,
}

fn main() -> Result<(), TermsError> {
    let args = Args::parse();

    let term = args.term.join(" ");

    let wiki: Wikipedia<wikipedia::http::default::Client> = Wikipedia::default();
    let page = wiki.page_from_title(term);

    let content = page.get_content()?;
    let content = content.split('\n').next().ok_or(TermsError::ParseError)?;

    if content.ends_with("may refer to:") {
        return Err(TermsError::Ambiguous);
    }

    println!("{}", content);

    Ok(())
}

#[derive(Debug, Error)]
enum TermsError {
    #[error("Could not find article")]
    NotFound,

    #[error("Could not parse article")]
    ParseError,

    #[error("Could not disambiguate")]
    Ambiguous,
}

impl From<wikipedia::Error> for TermsError {
    fn from(_: wikipedia::Error) -> Self {
        Self::NotFound
    }
}
