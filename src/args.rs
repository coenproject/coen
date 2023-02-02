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
    New(NewArgs),

    /// Build project
    Build(BuildArgs),
}

#[derive(Debug, Args)]
pub struct NewArgs {
    /// Path of the file/project
    pub path: PathBuf,

    #[arg(short, long)]
    /// Name of the template
    pub template: Option<String>,

    #[arg(short, long)]
    /// Use reference for template
    pub reference: bool,
}

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Path of the file/project
    pub path: PathBuf,
}
