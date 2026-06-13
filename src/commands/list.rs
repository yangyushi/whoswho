use crate::db::Database;
use crate::errors::Result;
use colored::Colorize;
use serde::Serialize;

pub fn run(db: &Database, recent: bool, limit: Option<usize>, json: bool) -> Result<()> {
    let people = db.list_people(recent, limit)?;
    let note_counts: Vec<usize> = people
        .iter()
        .map(|person| db.count_notes(person.id))
        .collect::<Result<Vec<_>>>()?;

    if json {
        #[derive(Serialize)]
        struct ListedPerson<'a> {
            #[serde(flatten)]
            person: &'a crate::models::Person,
            note_count: usize,
        }

        let listed: Vec<ListedPerson<'_>> = people
            .iter()
            .zip(note_counts.iter())
            .map(|(person, note_count)| ListedPerson {
                person,
                note_count: *note_count,
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&listed).unwrap());
    } else {
        if people.is_empty() {
            println!("No people found.");
        } else {
            for (person, note_count) in people.iter().zip(note_counts.iter()) {
                println!(
                    "[{}] {} - {} - updated {}",
                    person.id.to_string().dimmed(),
                    person.name.bold(),
                    format_note_count(*note_count).dimmed(),
                    person.updated_at.format("%Y-%m-%d").to_string().dimmed()
                );
            }
        }
    }

    Ok(())
}

fn format_note_count(count: usize) -> String {
    match count {
        1 => "1 note".to_string(),
        _ => format!("{} notes", count),
    }
}
