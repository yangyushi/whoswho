use crate::errors::{WswError, Result};
use crate::models::{Note, Person, PersonUpdate};
use chrono::Local;
use rusqlite::{params, Connection, OptionalExtension};
use std::collections::HashMap;
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS people (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                fields TEXT NOT NULL DEFAULT '{}',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                person_id INTEGER NOT NULL,
                content TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_people_name ON people(name)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_notes_person_id ON notes(person_id)",
            [],
        )?;

        Ok(())
    }

    pub fn add_person(&self, name: &str, fields: HashMap<String, String>) -> Result<Person> {
        let fields_json = serde_json::to_string(&fields)?;
        let now = Local::now();

        self.conn.execute(
            "INSERT INTO people (name, fields, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
            params![name, fields_json, now],
        )?;

        let id = self.conn.last_insert_rowid();

        Ok(Person {
            id,
            name: name.to_string(),
            fields,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_person_by_id(&self, id: i64) -> Result<Option<Person>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, fields, created_at, updated_at FROM people WHERE id = ?1"
        )?;

        let person = stmt
            .query_row([id], |row| {
                let fields_json: String = row.get(2)?;
                let fields: HashMap<String, String> = serde_json::from_str(&fields_json)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                        2,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    ))?;

                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    fields,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .optional()?;

        Ok(person)
    }

    pub fn find_people_by_name(&self, name: &str) -> Result<Vec<Person>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, fields, created_at, updated_at FROM people WHERE name LIKE ?1 ORDER BY name"
        )?;

        let pattern = format!("%{}%", name);
        let rows = stmt.query_map([pattern], |row| {
            let fields_json: String = row.get(2)?;
            let fields: HashMap<String, String> = serde_json::from_str(&fields_json)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                    2,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                ))?;

            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                fields,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        let mut people = Vec::new();
        for row in rows {
            people.push(row?);
        }
        Ok(people)
    }

    pub fn update_person(&self, id: i64, update: PersonUpdate) -> Result<()> {
        if let Some(person) = self.get_person_by_id(id)? {
            let mut fields = person.fields;

            if let Some(new_name) = update.name {
                let mut stmt = self.conn.prepare("UPDATE people SET name = ?1, updated_at = ?2 WHERE id = ?3")?;
                stmt.execute(params![new_name, Local::now(), id])?;
            }

            for (key, value) in update.fields {
                fields.insert(key, value);
            }

            let fields_json = serde_json::to_string(&fields)?;
            let mut stmt = self.conn.prepare("UPDATE people SET fields = ?1, updated_at = ?2 WHERE id = ?3")?;
            stmt.execute(params![fields_json, Local::now(), id])?;

            Ok(())
        } else {
            Err(WswError::NotFound(format!("ID {}", id)))
        }
    }

    pub fn delete_person(&self, id: i64) -> Result<()> {
        let rows = self.conn.execute("DELETE FROM people WHERE id = ?1", [id])?;
        if rows == 0 {
            Err(WswError::NotFound(format!("ID {}", id)))
        } else {
            Ok(())
        }
    }

    pub fn delete_field(&self, id: i64, field_name: &str) -> Result<()> {
        if let Some(person) = self.get_person_by_id(id)? {
            let mut fields = person.fields;
            fields.remove(field_name);

            let fields_json = serde_json::to_string(&fields)?;
            self.conn.execute(
                "UPDATE people SET fields = ?1, updated_at = ?2 WHERE id = ?3",
                params![fields_json, Local::now(), id],
            )?;
            Ok(())
        } else {
            Err(WswError::NotFound(format!("ID {}", id)))
        }
    }

    pub fn add_note(&self, person_id: i64, content: &str) -> Result<Note> {
        let now = Local::now();
        self.conn.execute(
            "INSERT INTO notes (person_id, content, created_at) VALUES (?1, ?2, ?3)",
            params![person_id, content, now],
        )?;

        let id = self.conn.last_insert_rowid();

        Ok(Note {
            id,
            person_id,
            content: content.to_string(),
            created_at: now,
        })
    }

    pub fn get_notes(&self, person_id: i64, limit: usize) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, person_id, content, created_at FROM notes WHERE person_id = ?1 ORDER BY created_at DESC LIMIT ?2"
        )?;

        let rows = stmt.query_map(params![person_id, limit as i64], |row| {
            Ok(Note {
                id: row.get(0)?,
                person_id: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;

        let mut notes = Vec::new();
        for row in rows {
            notes.push(row?);
        }
        Ok(notes)
    }

    pub fn list_people(&self, recent: bool, limit: Option<usize>) -> Result<Vec<Person>> {
        let order_by = if recent { "updated_at DESC" } else { "name" };
        let limit_clause = limit.map(|l| format!("LIMIT {}", l)).unwrap_or_default();

        let sql = format!(
            "SELECT id, name, fields, created_at, updated_at FROM people ORDER BY {} {}",
            order_by, limit_clause
        );

        let mut stmt = self.conn.prepare(&sql)?;

        let rows = stmt.query_map([], |row| {
            let fields_json: String = row.get(2)?;
            let fields: HashMap<String, String> = serde_json::from_str(&fields_json)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                    2,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                ))?;

            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                fields,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        let mut people = Vec::new();
        for row in rows {
            people.push(row?);
        }
        Ok(people)
    }

    pub fn search(&self, query: &str, field: Option<&str>) -> Result<Vec<Person>> {
        if let Some(field_name) = field {
            let mut stmt = self.conn.prepare(
                "SELECT id, name, fields, created_at, updated_at FROM people WHERE json_extract(fields, ?1) LIKE ?2 ORDER BY name"
            )?;

            let json_path = format!("$.{}", field_name);
            let pattern = format!("%{}%", query);

            let rows = stmt.query_map(params![json_path, pattern], |row| {
                let fields_json: String = row.get(2)?;
                let fields: HashMap<String, String> = serde_json::from_str(&fields_json)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                        2,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    ))?;

                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    fields,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?;

            let mut people = Vec::new();
            for row in rows {
                people.push(row?);
            }
            Ok(people)
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT id, name, fields, created_at, updated_at FROM people WHERE name LIKE ?1 OR fields LIKE ?2 ORDER BY name"
            )?;

            let pattern = format!("%{}%", query);

            let rows = stmt.query_map(params![pattern, pattern], |row| {
                let fields_json: String = row.get(2)?;
                let fields: HashMap<String, String> = serde_json::from_str(&fields_json)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                        2,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    ))?;

                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    fields,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?;

            let mut people = Vec::new();
            for row in rows {
                people.push(row?);
            }
            Ok(people)
        }
    }
}
