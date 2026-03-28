use crate::commands::{find_person_interactive, parse_field};
use crate::db::Database;
use crate::errors::Result;
use crate::models::PersonUpdate;
use std::collections::HashMap;

pub fn run(db: &Database, name: String, fields_input: Vec<String>, use_id: bool) -> Result<()> {
    let person = find_person_interactive(db, &name, use_id)?;

    let mut fields = HashMap::new();
    for field in fields_input {
        let (key, value) = parse_field(&field)?;
        fields.insert(key, value);
    }

    let update = PersonUpdate { name: None, fields };
    db.update_person(person.id, update)?;
    println!("Updated: {} (ID: {})", person.name, person.id);
    Ok(())
}
