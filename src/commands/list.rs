use crate::db::Database;
use crate::errors::Result;
use colored::Colorize;

pub fn run(db: &Database, recent: bool, limit: Option<usize>, json: bool) -> Result<()> {
    let people = db.list_people(recent, limit)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&people).unwrap());
    } else {
        if people.is_empty() {
            println!("No people found.");
        } else {
            for person in people {
                println!(
                    "[{}] {} - updated {}",
                    person.id.to_string().dimmed(),
                    person.name.bold(),
                    person.updated_at.format("%Y-%m-%d").to_string().dimmed()
                );
            }
        }
    }

    Ok(())
}
