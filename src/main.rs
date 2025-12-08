mod cli;
use clap::Parser;
use cli::{Cli, Commands};
use looming::{model, model::Trackable, storage};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            name,
            item_type,
            deadline,
            url,
            note,
        } => {
            let mut items = storage::load().unwrap_or_default();
            let hard = item_type == "hard";
            let new_item = model::Item::new(deadline, &name, hard, url, note);
            items.push(new_item);
            storage::save(items)?;
        }
        Commands::List => {
            let items = storage::load()?;
            for item in items {
                item.display();
            }
        }
        Commands::Upcoming { days } => {
            let items = storage::load()?;
            let today = chrono::Local::now().naive_local().date();
            for item in items {
                let days_left = item.days_left();
                if days_left <= days {
                    println!(
                        "{} - due in {} days on {}",
                        item.name(),
                        days_left,
                        item.deadline_date()
                    );
                }
            }
        }
    }

    Ok(())
}
