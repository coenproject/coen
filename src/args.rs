use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CoenArgs {
    /// Operation
    #[clap(subcommand)]
    pub operation: OperationType,
}

#[derive(Debug, Subcommand)]
pub enum OperationType {
    /// Create a new project
    New(File),

    /// Build project
    Build(File),
}

#[derive(Debug, Args)]
pub struct File {
    /// Path of the file/project
    pub path: PathBuf,
}
