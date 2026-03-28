use crate::db::Database;
use crate::errors::Result;
use colored::Colorize;

pub fn run(db: &Database, query: String, field: Option<String>, json: bool) -> Result<()> {
    let people = db.search(&query, field.as_deref())?;

    if json {
        println!("{}", serde_json::to_string_pretty(&people).unwrap());
    } else {
        if people.is_empty() {
            println!("No matches found.");
        } else {
            println!("Found {} match(es):", people.len());
            for person in people {
                println!(
                    "[{}] {}",
                    person.id.to_string().dimmed(),
                    person.name.bold()
                );
            }
        }
    }

    Ok(())
}
