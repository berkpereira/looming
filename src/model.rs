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
    fn display(&self);
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

    fn display(&self) {
        let url_display = match &self.url {
            Some(u) => {
                // OSC 8 hyperlink with bold text: \x1b[1m = bold, \x1b[0m = reset
                format!(" (\x1b[1m\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\\x1b[0m)", u, u)
            }
            None => String::new(),
        };
        println!(
            "{} - due in {} days on {}{}",
            self.name,
            self.days_left(),
            self.deadlines.date.format("%-d %b %Y"),
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
