use crate::commands::find_person_interactive;
use crate::db::Database;
use crate::errors::Result;
use std::io::{self, Write};

pub fn run(
    db: &Database,
    name: String,
    field: Option<String>,
    use_id: bool,
    yes: bool,
) -> Result<()> {
    let person = find_person_interactive(db, &name, use_id)?;

    if let Some(field_name) = field {
        if !yes {
            print!("Remove field '{}' from {}? [y/N] ", field_name, person.name);
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Cancelled.");
                return Ok(());
            }
        }

        db.delete_field(person.id, &field_name)?;
        println!("Removed field '{}' from {}", field_name, person.name);
    } else {
        if !yes {
            print!("Remove {} entirely? [y/N] ", person.name);
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Cancelled.");
                return Ok(());
            }
        }

        db.delete_person(person.id)?;
        println!("Removed {}", person.name);
    }

    Ok(())
}
