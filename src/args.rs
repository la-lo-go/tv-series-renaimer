use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TvSeriesRenaimerArgs {
    /// GPT API key (required, get it from https://platform.openai.com/)
    #[arg(short, long)]
    pub key: String,

    /// Path to the directory that will be scanned
    #[arg(short, long, default_value_t = String::from("./"))]
    pub path: String,

    /// Mode to run the program in (recursive or single)
    #[arg(short, long, default_value_t = String::from("recursive"))]
    pub mode: String,

    /// GPT model to use, must be one with JSON mode enable (see https://platform.openai.com/docs/guides/text-generation/json-mode)
    #[arg(short, long, default_value_t = String::from("gpt-3.5-turbo"))]
    pub gtp_model: String,
}
