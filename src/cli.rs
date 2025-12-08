use chrono::NaiveDate;
use clap::{Parser, Subcommand};

fn parse_date(s: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(s, "%d-%m-%Y")
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// TODO add a command to modify item
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new deadline
    Add {
        /// Name of the item
        #[arg(short, long)]
        name: String,

        /// Deadline type
        #[arg(short = 't', long, value_parser = ["hard", "soft"], default_value_t = String::from("hard"))]
        r#type: String,

        /// Deadline DD-MM-YYYY
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

    /// Display an item by name
    Display {
        /// Name of the item to display
        name: String,
    },

    /// Remove an item by name
    Remove {
        /// Name of the item to remove
        name: String,
    },

    /// Extend a soft deadline by N days
    Extend {
        /// Name of the soft item to extend
        #[arg(short, long)]
        name: String,

        /// Number of days to extend by
        #[arg(short, long, default_value_t = 7)]
        days: i64,
    },

    /// Show items due in the next N days
    In {
        /// Number of days to look ahead (default: 14)
        #[arg(default_value_t = 14)]
        days: i64,
    },
}
