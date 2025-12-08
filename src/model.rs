use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    deadlines: Deadline, // will make this a vector in future
    url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Deadline {
    date: NaiveDate,
    hard: bool,
    note: Option<String>,
}

impl Item {
    pub fn new(
        date: NaiveDate,
        name: &str,
        hard: bool,
        url: Option<String>,
        note: Option<String>,
    ) -> Self {
        let deadlines = Deadline { date, hard, note };
        Item {
            name: name.to_string(),
            deadlines,
            url,
        }
    }

    // pub fn show_date(&self) {
    //     println!("Deadline is {}", self.deadline);
    // }
}

pub trait Trackable {
    fn name(&self) -> &str;
    fn deadline_date(&self) -> NaiveDate;
    fn is_hard(&self) -> bool;
    fn note(&self) -> Option<&str>;
    fn display(&self);
    fn display_with_bullet(&self, show_bullet: bool);
    fn days_left(&self) -> i64;
    fn change_deadline(&mut self, new_date: NaiveDate);
    fn next_deadline(&self) -> (); // placeholder until we have multiple deadlines per item
}

impl Trackable for Item {
    fn name(&self) -> &str {
        &self.name
    }

    fn deadline_date(&self) -> NaiveDate {
        self.deadlines.date
    }

    fn is_hard(&self) -> bool {
        self.deadlines.hard
    }

    fn note(&self) -> Option<&str> {
        self.deadlines.note.as_deref()
    }

    fn display(&self) {
        self.display_with_bullet(true);
    }

    fn display_with_bullet(&self, show_bullet: bool) {
        let url_display = match &self.url {
            Some(u) => {
                // OSC 8 hyperlink with bold cyan text
                format!(" (\x1b[1;36m\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\\x1b[0m)", u, u)
            }
            None => String::new(),
        };

        let days_left = self.days_left();
        // Color based on urgency: red (<=3), orange (4-14), green (>14)
        let days_color = if days_left <= 3 {
            "\x1b[1;31m" // bold red
        } else if days_left <= 14 {
            "\x1b[1;33m" // bold orange (yellow)
        } else {
            "\x1b[1;32m" // bold green
        };

        let bullet = if show_bullet {
            format!("{}※\x1b[0m ", days_color) // bullet matches urgency color
        } else {
            String::new()
        };

        println!(
            "{}\x1b[1m{}\x1b[0m - due in {}{} days\x1b[0m on \x1b[36m{}\x1b[0m{}",
            bullet,
            self.name,           // bold name
            days_color,
            days_left,
            self.deadlines.date.format("%-d %b %Y"),  // cyan date
            url_display
        );
    }

    fn days_left(&self) -> i64 {
        let today = chrono::Local::now().naive_local().date();
        (self.deadlines.date - today).num_days()
    }

    fn change_deadline(&mut self, new_date: NaiveDate) {
        self.deadlines.date = new_date;
    }

    fn next_deadline(&self) -> () {}
}
