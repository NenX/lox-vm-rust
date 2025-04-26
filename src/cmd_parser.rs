use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct CmdParser {
    pub file: Option<PathBuf>,
}
