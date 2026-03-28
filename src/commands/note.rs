use crate::commands::find_person_interactive;
use crate::db::Database;
use crate::errors::Result;

pub fn run(db: &Database, name: String, content: String, use_id: bool) -> Result<()> {
    let person = find_person_interactive(db, &name, use_id)?;
    let note = db.add_note(person.id, &content)?;
    println!("Added note to {} (ID: {})", person.name, note.id);
    Ok(())
}
