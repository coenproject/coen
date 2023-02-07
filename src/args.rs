use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CoenArgs {
    /// Path of the root file
    pub root: PathBuf,

    #[arg(short, long)]
    /// Path of the target
    pub target: Option<PathBuf>,

    #[arg(short, long)]
    /// Silence logs
    pub silent: bool,
}
