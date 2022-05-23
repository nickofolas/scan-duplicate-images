use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Parser {
    #[structopt(long = "source")]
    pub source_dir: String,

    #[structopt(long = "min-similarity", default_value = "100")]
    pub match_threshold: u8,

    #[structopt(long = "delete-files")]
    pub delete_files: bool,
}
