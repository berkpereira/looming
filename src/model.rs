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
    fn display(&self);
    fn days_left(&self) -> i64;
    fn next_deadline(&self) -> (); // placeholder until we have multiple deadlines per item
}

impl Trackable for Item {
    fn name(&self) -> &str {
        &self.name
    }

    fn deadline_date(&self) -> NaiveDate {
        self.deadlines.date
    }

    fn display(&self) {
        println!(
            "{} - due on {}{}",
            self.name,
            self.deadlines.date,
            match &self.url {
                Some(u) => format!(" (URL: {})", u),
                None => String::new(),
            }
        );
    }

    fn days_left(&self) -> i64 {
        let today = chrono::Local::now().naive_local().date();
        (self.deadlines.date - today).num_days()
    }

    fn next_deadline(&self) -> () {}
}
