use crate::errors::{Result, WswError};
use crate::models::{ListedPerson, Note, Person, PersonUpdate};
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
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, fields, created_at, updated_at FROM people WHERE id = ?1")?;

        let person = stmt
            .query_row([id], |row| {
                let fields_json: String = row.get(2)?;
                let fields: HashMap<String, String> =
                    serde_json::from_str(&fields_json).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;

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
            let fields: HashMap<String, String> =
                serde_json::from_str(&fields_json).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        2,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;

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
                let mut stmt = self
                    .conn
                    .prepare("UPDATE people SET name = ?1, updated_at = ?2 WHERE id = ?3")?;
                stmt.execute(params![new_name, Local::now(), id])?;
            }

            for (key, value) in update.fields {
                fields.insert(key, value);
            }

            let fields_json = serde_json::to_string(&fields)?;
            let mut stmt = self
                .conn
                .prepare("UPDATE people SET fields = ?1, updated_at = ?2 WHERE id = ?3")?;
            stmt.execute(params![fields_json, Local::now(), id])?;

            Ok(())
        } else {
            Err(WswError::NotFound(format!("ID {}", id)))
        }
    }

    pub fn delete_person(&self, id: i64) -> Result<()> {
        let rows = self
            .conn
            .execute("DELETE FROM people WHERE id = ?1", [id])?;
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
        self.conn.execute(
            "UPDATE people SET updated_at = ?1 WHERE id = ?2",
            params![now, person_id],
        )?;

        let id = self.conn.last_insert_rowid();

        Ok(Note {
            id,
            person_id,
            content: content.to_string(),
            created_at: now,
        })
    }

    pub fn get_all_notes(&self, person_id: i64) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, person_id, content, created_at FROM notes WHERE person_id = ?1 ORDER BY created_at DESC"
        )?;

        let rows = stmt.query_map([person_id], |row| {
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

    pub fn list_people_with_note_counts(&self, limit: Option<usize>) -> Result<Vec<ListedPerson>> {
        let limit_clause = limit.map(|l| format!("LIMIT {}", l)).unwrap_or_default();

        let sql = format!(
            "SELECT
                p.id,
                p.name,
                p.fields,
                p.created_at,
                p.updated_at,
                COUNT(n.id) AS note_count
             FROM (
                SELECT id, name, fields, created_at, updated_at
                FROM people
                ORDER BY updated_at DESC {}
             ) p
             LEFT JOIN notes n ON n.person_id = p.id
             GROUP BY p.id, p.name, p.fields, p.created_at, p.updated_at
             ORDER BY p.updated_at DESC",
            limit_clause
        );

        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| {
            let fields_json: String = row.get(2)?;
            let fields: HashMap<String, String> =
                serde_json::from_str(&fields_json).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        2,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
            let note_count: i64 = row.get(5)?;

            Ok(ListedPerson {
                person: Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    fields,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                },
                note_count: note_count as usize,
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
            if field_name.eq_ignore_ascii_case("note") || field_name.eq_ignore_ascii_case("notes") {
                let mut stmt = self.conn.prepare(
                    "SELECT id, name, fields, created_at, updated_at FROM people WHERE id IN (
                        SELECT person_id FROM notes WHERE content LIKE ?1
                    ) ORDER BY name",
                )?;

                let pattern = format!("%{}%", query);

                let rows = stmt.query_map([pattern], |row| {
                    let fields_json: String = row.get(2)?;
                    let fields: HashMap<String, String> = serde_json::from_str(&fields_json)
                        .map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                2,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

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
                return Ok(people);
            }

            let mut stmt = self.conn.prepare(
                "SELECT id, name, fields, created_at, updated_at FROM people WHERE json_extract(fields, ?1) LIKE ?2 ORDER BY name"
            )?;

            let json_path = format!("$.{}", field_name);
            let pattern = format!("%{}%", query);

            let rows = stmt.query_map(params![json_path, pattern], |row| {
                let fields_json: String = row.get(2)?;
                let fields: HashMap<String, String> =
                    serde_json::from_str(&fields_json).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;

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
                "SELECT id, name, fields, created_at, updated_at FROM people WHERE
                    name LIKE ?1
                    OR fields LIKE ?2
                    OR id IN (SELECT person_id FROM notes WHERE content LIKE ?3)
                 ORDER BY name",
            )?;

            let pattern = format!("%{}%", query);

            let rows = stmt.query_map(params![pattern, pattern, pattern], |row| {
                let fields_json: String = row.get(2)?;
                let fields: HashMap<String, String> =
                    serde_json::from_str(&fields_json).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;

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

    pub fn search_notes_for_person(&self, person_id: i64, query: &str) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, person_id, content, created_at FROM notes
             WHERE person_id = ?1 AND content LIKE ?2
             ORDER BY created_at DESC",
        )?;

        let pattern = format!("%{}%", query);
        let rows = stmt.query_map(params![person_id, pattern], |row| {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (Database, NamedTempFile) {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::open(temp_file.path()).unwrap();
        (db, temp_file)
    }

    #[test]
    fn test_add_person() {
        let (db, _temp) = create_test_db();
        let mut fields = HashMap::new();
        fields.insert("email".to_string(), "test@example.com".to_string());
        fields.insert("role".to_string(), "Developer".to_string());

        let person = db.add_person("Test User", fields.clone()).unwrap();
        assert_eq!(person.name, "Test User");
        assert_eq!(
            person.fields.get("email"),
            Some(&"test@example.com".to_string())
        );
        assert_eq!(person.fields.get("role"), Some(&"Developer".to_string()));
        assert!(person.id > 0);
    }

    #[test]
    fn test_get_person_by_id() {
        let (db, _temp) = create_test_db();
        let person = db.add_person("Test User", HashMap::new()).unwrap();

        let retrieved = db.get_person_by_id(person.id).unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.name, "Test User");
        assert_eq!(retrieved.id, person.id);
    }

    #[test]
    fn test_get_person_by_id_not_found() {
        let (db, _temp) = create_test_db();
        let result = db.get_person_by_id(999).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_find_people_by_name() {
        let (db, _temp) = create_test_db();
        db.add_person("Alice Smith", HashMap::new()).unwrap();
        db.add_person("Alice Jones", HashMap::new()).unwrap();
        db.add_person("Bob Brown", HashMap::new()).unwrap();

        let results = db.find_people_by_name("Alice").unwrap();
        assert_eq!(results.len(), 2);

        let results = db.find_people_by_name("Bob").unwrap();
        assert_eq!(results.len(), 1);

        let results = db.find_people_by_name("Charlie").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_update_person() {
        let (db, _temp) = create_test_db();
        let person = db.add_person("Test User", HashMap::new()).unwrap();

        let mut new_fields = HashMap::new();
        new_fields.insert("email".to_string(), "updated@example.com".to_string());

        let update = PersonUpdate {
            name: Some("Updated Name".to_string()),
            fields: new_fields,
        };

        db.update_person(person.id, update).unwrap();

        let updated = db.get_person_by_id(person.id).unwrap().unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(
            updated.fields.get("email"),
            Some(&"updated@example.com".to_string())
        );
    }

    #[test]
    fn test_delete_person() {
        let (db, _temp) = create_test_db();
        let person = db.add_person("Test User", HashMap::new()).unwrap();

        db.delete_person(person.id).unwrap();
        let result = db.get_person_by_id(person.id).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_person_not_found() {
        let (db, _temp) = create_test_db();
        let result = db.delete_person(999);
        assert!(matches!(result, Err(WswError::NotFound(_))));
    }

    #[test]
    fn test_delete_field() {
        let (db, _temp) = create_test_db();
        let mut fields = HashMap::new();
        fields.insert("email".to_string(), "test@example.com".to_string());
        fields.insert("phone".to_string(), "555-1234".to_string());

        let person = db.add_person("Test User", fields).unwrap();
        db.delete_field(person.id, "email").unwrap();

        let updated = db.get_person_by_id(person.id).unwrap().unwrap();
        assert!(updated.fields.get("email").is_none());
        assert!(updated.fields.get("phone").is_some());
    }

    #[test]
    fn test_add_note() {
        let (db, _temp) = create_test_db();
        let person = db.add_person("Test User", HashMap::new()).unwrap();

        let note = db.add_note(person.id, "Test note content").unwrap();
        assert_eq!(note.content, "Test note content");
        assert_eq!(note.person_id, person.id);
        assert!(note.id > 0);
    }

    #[test]
    fn test_get_notes() {
        let (db, _temp) = create_test_db();
        let person = db.add_person("Test User", HashMap::new()).unwrap();

        db.add_note(person.id, "First note").unwrap();
        db.add_note(person.id, "Second note").unwrap();
        db.add_note(person.id, "Third note").unwrap();

        let notes = db.get_notes(person.id, 10).unwrap();
        assert_eq!(notes.len(), 3);

        // Test limit
        let notes = db.get_notes(person.id, 2).unwrap();
        assert_eq!(notes.len(), 2);
    }

    #[test]
    fn test_list_people_with_note_counts_sorts_by_updated_at() {
        let (db, _temp) = create_test_db();
        let charlie = db.add_person("Charlie", HashMap::new()).unwrap();
        let alice = db.add_person("Alice", HashMap::new()).unwrap();
        let bob = db.add_person("Bob", HashMap::new()).unwrap();

        db.add_note(alice.id, "Most recent").unwrap();

        let people = db.list_people_with_note_counts(None).unwrap();
        assert_eq!(people.len(), 3);
        assert_eq!(people[0].person.name, "Alice");
        assert_eq!(people[0].person.id, alice.id);
        assert!(people.iter().any(|listed| listed.person.id == bob.id));
        assert!(people.iter().any(|listed| listed.person.id == charlie.id));
    }

    #[test]
    fn test_list_people_with_note_counts_limit() {
        let (db, _temp) = create_test_db();
        db.add_person("Alice", HashMap::new()).unwrap();
        db.add_person("Bob", HashMap::new()).unwrap();
        db.add_person("Charlie", HashMap::new()).unwrap();

        let people = db.list_people_with_note_counts(Some(2)).unwrap();
        assert_eq!(people.len(), 2);
    }

    #[test]
    fn test_list_people_with_note_counts() {
        let (db, _temp) = create_test_db();
        let alice = db.add_person("Alice", HashMap::new()).unwrap();
        let bob = db.add_person("Bob", HashMap::new()).unwrap();

        db.add_note(alice.id, "First").unwrap();
        db.add_note(alice.id, "Second").unwrap();

        let people = db.list_people_with_note_counts(None).unwrap();

        assert_eq!(people.len(), 2);
        assert_eq!(people[0].person.name, "Alice");
        assert_eq!(people[0].note_count, 2);
        assert_eq!(people[1].person.name, "Bob");
        assert_eq!(people[1].person.id, bob.id);
        assert_eq!(people[1].note_count, 0);
    }

    #[test]
    fn test_search_by_name() {
        let (db, _temp) = create_test_db();
        db.add_person("Alice Smith", HashMap::new()).unwrap();
        db.add_person("Bob Jones", HashMap::new()).unwrap();

        let results = db.search("Smith", None).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Alice Smith");
    }

    #[test]
    fn test_search_by_field() {
        let (db, _temp) = create_test_db();
        let mut fields1 = HashMap::new();
        fields1.insert("role".to_string(), "Developer".to_string());
        db.add_person("Alice", fields1).unwrap();

        let mut fields2 = HashMap::new();
        fields2.insert("role".to_string(), "Manager".to_string());
        db.add_person("Bob", fields2).unwrap();

        let results = db.search("Developer", Some("role")).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Alice");
    }
}
