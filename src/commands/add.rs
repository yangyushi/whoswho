use crate::commands::parse_field;
use crate::db::Database;
use crate::errors::Result;
use crate::models::Person;
use std::collections::HashMap;

pub fn run(db: &Database, name: String, fields_input: Vec<String>) -> Result<()> {
    let mut fields = HashMap::new();
    for field in fields_input {
        let (key, value) = parse_field(&field)?;
        fields.insert(key, value);
    }

    let person: Person = db.add_person(&name, fields)?;
    println!("Added: {} (ID: {})", person.name, person.id);
    Ok(())
}
