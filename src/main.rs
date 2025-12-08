mod cli;
use chrono::NaiveDate;
use clap::Parser;
use cli::{Cli, Commands};
use looming::{model, model::Trackable, storage};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add {
            name,
            r#type,
            deadline,
            url,
            note,
        }) => handle_add(name, r#type, deadline, url, note),
        Some(Commands::Display { name }) => handle_display(name),
        Some(Commands::Remove { name }) => handle_remove(name),
        Some(Commands::Extend { name, days }) => handle_extend(name, days),
        Some(Commands::List) => handle_list(),
        Some(Commands::In { days }) => handle_in(days),
        None => handle_in(14), // Default: show items due in next 14 days
    }
}

fn handle_add(
    name: String,
    item_type: String,
    deadline: NaiveDate,
    url: Option<String>,
    note: Option<String>,
) -> anyhow::Result<()> {
    let mut items = storage::load().unwrap_or_default();
    if items.iter().any(|item| item.name() == name) {
        return Err(anyhow::anyhow!(
            "An item with the name '{}' already exists.",
            name
        ));
    }
    let hard = item_type == "hard";
    let new_item = model::Item::new(deadline, &name, hard, url, note);
    items.push(new_item);
    storage::save(items)
}

fn handle_display(name: String) -> anyhow::Result<()> {
    let items = storage::load()?;
    for item in items {
        if item.name() == name {
            item.display();
            return Ok(());
        }
    }
    Err(anyhow::anyhow!("No item found with the name '{}'.", name))
}

fn handle_extend(name: String, days: i64) -> anyhow::Result<()> {
    let mut items = storage::load()?;
    for item in &mut items {
        if item.name() == name {
            if item.is_hard() {
                return Err(anyhow::anyhow!(
                    "Cannot extend a hard deadline for item '{}'.",
                    name
                ));
            } else {
                let new_date = item.deadline_date() + chrono::Duration::days(days);
                item.change_deadline(new_date);
                println!(
                    "Extended deadline for '{}' by {} days to {}.",
                    name,
                    days,
                    item.deadline_date().format("%-d %b %Y")
                );
            }
        }
    }
    storage::save(items)
}

fn handle_list() -> anyhow::Result<()> {
    let items = storage::load()?;
    let mut printed: usize = 0;
    for item in items {
        item.display();
        printed += 1;
    }
    if printed == 0 {
        println!("No items are currently being tracked.");
    }
    Ok(())
}

fn handle_remove(name: String) -> anyhow::Result<()> {
    let mut items = storage::load().unwrap_or_default();
    items.retain(|item| item.name() != name);
    storage::save(items)
}

fn handle_in(days: i64) -> anyhow::Result<()> {
    let items = storage::load()?;
    let mut printed: usize = 0;
    for item in items {
        if item.days_left() <= days {
            item.display();
            printed += 1;
        }
    }
    if printed == 0 {
        println!("No items due in the next {} days.", days);
    }
    Ok(())
}
