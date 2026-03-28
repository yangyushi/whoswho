pub mod add;
pub mod get;
pub mod list;
pub mod log;
pub mod note;
pub mod rm;
pub mod search;
pub mod set;

use crate::db::Database;
use crate::errors::Result;
use crate::models::Person;

fn find_person_interactive(db: &Database, name: &str, use_id: bool) -> Result<Person> {
    if use_id {
        let id: i64 = name.parse().map_err(|_| {
            crate::errors::WswError::Other(format!("'{}' is not a valid ID", name))
        })?;
        db.get_person_by_id(id)?.ok_or_else(|| {
            crate::errors::WswError::NotFound(format!("ID {}", id))
        })
    } else {
        let people = db.find_people_by_name(name)?;
        match people.len() {
            0 => Err(crate::errors::WswError::NotFound(name.to_string())),
            1 => Ok(people.into_iter().next().unwrap()),
            _ => {
                eprintln!("Multiple people found matching '{}':", name);
                for person in &people {
                    eprintln!("  [{}] {} ({})", person.id, person.name, person.updated_at.format("%Y-%m-%d"));
                }
                eprintln!("\nUse --id to specify which one to use.");
                Err(crate::errors::WswError::MultipleMatches(name.to_string()))
            }
        }
    }
}

fn parse_field(field: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = field.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(crate::errors::WswError::InvalidFieldFormat(field.to_string()));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn display_person(person: &Person, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(person).unwrap());
    } else {
        // Calculate max key length for alignment
        let mut all_keys: Vec<&str> = vec!["Name"];
        for key in person.fields.keys() {
            all_keys.push(key);
        }
        let max_len = all_keys.iter().map(|k| k.len()).max().unwrap_or(4);

        // Print with alignment
        println!("{:width$} {}", "Name:", person.name, width = max_len + 1);
        for (key, value) in &person.fields {
            println!("{:width$} {}", format!("{}:", key), value, width = max_len + 1);
        }
        println!("(updated on {})", person.updated_at.format("%Y-%m-%d %H:%M"));
    }
}
