use crate::commands::{display_person, find_person_interactive};
use crate::db::Database;
use crate::errors::Result;

pub fn run(db: &Database, name: String, use_id: bool, json: bool) -> Result<()> {
    let person = find_person_interactive(db, &name, use_id)?;
    display_person(&person, json);
    Ok(())
}
