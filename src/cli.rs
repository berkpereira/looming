use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use looming::model::Trackable;
use looming::{model, storage};

fn parse_date(s: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new deadline
    Add {
        /// Name of the item
        #[arg(short, long)]
        name: String,

        /// Deadline type (hard or soft)
        #[arg(short = 't', long, value_parser = ["hard", "soft"], default_value_t = String::from("hard"))]
        item_type: String,

        /// Deadline YYYY-MM-DD
        #[arg(short, long, value_parser = parse_date)]
        deadline: NaiveDate,

        /// Optional URL for the item
        #[arg(short, long)]
        url: Option<String>,

        /// Optional note for deadline
        #[arg(long)]
        note: Option<String>,
    },

    /// List all tracked items
    List,

    /// Show items due soon
    Upcoming {
        /// Number of days to look ahead
        #[arg(short, long, default_value_t = 14)]
        days: i64,
    },
}
