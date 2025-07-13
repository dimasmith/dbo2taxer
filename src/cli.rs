use std::path::PathBuf;

use clap::Parser;
use dbo2taxer::filter::Quarter;

/// Convert DBO account statement CSVs into a CSV for a Taxer system.
#[derive(Debug, Parser)]
pub struct Cli {
    /// Input file - a csv acccount statement in DBO format.
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Output file with taxer csv.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Include only payments occured during the specified quarter.
    #[arg(short, long, value_enum)]
    pub quarter: Option<Quarter>,

    /// Include only payments occured in a specified year.
    #[arg(short, long)]
    pub year: Option<i32>,
}
