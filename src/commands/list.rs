use crate::db::Database;
use crate::errors::Result;
use colored::Colorize;

pub fn run(db: &Database, recent: bool, limit: Option<usize>, json: bool) -> Result<()> {
    let people = db.list_people_with_note_counts(recent, limit)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&people).unwrap());
    } else {
        if people.is_empty() {
            println!("No people found.");
        } else {
            let id_width = people
                .iter()
                .map(|listed| visible_width(&listed.person.id.to_string()))
                .max()
                .unwrap_or(0);
            let name_width = people
                .iter()
                .map(|listed| visible_width(&listed.person.name))
                .max()
                .unwrap_or(0);
            let note_width = people
                .iter()
                .map(|listed| visible_width(&format_note_count(listed.note_count)))
                .max()
                .unwrap_or(0);

            for listed in people.iter() {
                let person = &listed.person;
                let id = pad_left(&person.id.to_string(), id_width);
                let name = pad_right(&person.name, name_width);
                let note_count = pad_right(&format_note_count(listed.note_count), note_width);

                println!(
                    "[{}] {} - {} - updated {}",
                    id.dimmed(),
                    name.bold(),
                    note_count.dimmed(),
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

fn pad_left(value: &str, width: usize) -> String {
    let padding = width.saturating_sub(visible_width(value));
    format!("{}{}", " ".repeat(padding), value)
}

fn pad_right(value: &str, width: usize) -> String {
    let padding = width.saturating_sub(visible_width(value));
    format!("{}{}", value, " ".repeat(padding))
}

fn visible_width(value: &str) -> usize {
    value.chars().map(char_width).sum()
}

fn char_width(ch: char) -> usize {
    if ch.is_control() {
        0
    } else if is_wide(ch) {
        2
    } else {
        1
    }
}

fn is_wide(ch: char) -> bool {
    matches!(
        ch,
        '\u{1100}'..='\u{115F}'
            | '\u{2329}'
            | '\u{232A}'
            | '\u{2E80}'..='\u{A4CF}'
            | '\u{AC00}'..='\u{D7A3}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{FE10}'..='\u{FE19}'
            | '\u{FE30}'..='\u{FE6F}'
            | '\u{FF00}'..='\u{FF60}'
            | '\u{FFE0}'..='\u{FFE6}'
            | '\u{1F300}'..='\u{1FAFF}'
            | '\u{20000}'..='\u{3FFFD}'
    )
}
