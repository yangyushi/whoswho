use crate::commands::find_person_interactive;
use crate::db::Database;
use crate::errors::Result;
use colored::Colorize;

pub fn run(db: &Database, name: String, use_id: bool, limit: usize) -> Result<()> {
    let person = find_person_interactive(db, &name, use_id)?;
    let notes = db.get_notes(person.id, limit)?;

    println!("Notes for {}:", person.name.bold());

    if notes.is_empty() {
        println!("  (no notes yet)");
    } else {
        for note in notes {
            println!(
                "  [{}] {}",
                note.created_at
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
                    .dimmed(),
                note.content
            );
        }
    }

    Ok(())
}
