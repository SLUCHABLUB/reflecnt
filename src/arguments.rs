use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub(crate) struct Arguments {
    pub file: PathBuf,
}
