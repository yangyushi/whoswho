use crate::db::Database;
use crate::errors::Result;
use colored::Colorize;
use serde::Serialize;

pub fn run(db: &Database, query: String, field: Option<String>, json: bool) -> Result<()> {
    let people = db.search(&query, field.as_deref())?;
    let include_note_matches = should_show_note_matches(field.as_deref());

    #[derive(Serialize)]
    struct SearchResult {
        #[serde(flatten)]
        person: crate::models::Person,
        matched_notes: Vec<crate::models::Note>,
    }

    let mut results = Vec::new();
    for person in people {
        let matched_notes = if include_note_matches {
            db.search_notes_for_person(person.id, &query)?
        } else {
            Vec::new()
        };
        results.push(SearchResult {
            person,
            matched_notes,
        });
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
    } else {
        if results.is_empty() {
            println!("No matches found.");
        } else {
            println!("Found {} match(es):", results.len());
            for result in results {
                println!(
                    "[{}] {}",
                    result.person.id.to_string().dimmed(),
                    result.person.name.bold()
                );
                for note in result.matched_notes {
                    println!(
                        "  note [{}] {}",
                        note.created_at
                            .format("%Y-%m-%d %H:%M")
                            .to_string()
                            .dimmed(),
                        note.content
                    );
                }
            }
        }
    }

    Ok(())
}

fn should_show_note_matches(field: Option<&str>) -> bool {
    field
        .map(|field| field.eq_ignore_ascii_case("note") || field.eq_ignore_ascii_case("notes"))
        .unwrap_or(true)
}
