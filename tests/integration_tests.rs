use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

fn wsw_cmd() -> Command {
    Command::cargo_bin("wsw").unwrap()
}

fn temp_db_arg() -> (String, NamedTempFile) {
    let temp_file = NamedTempFile::new().unwrap();
    (temp_file.path().to_str().unwrap().to_string(), temp_file)
}

#[test]
fn test_cli_no_args_shows_help() {
    let mut cmd = wsw_cmd();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"));
}

#[test]
fn test_cli_help_flag() {
    let mut cmd = wsw_cmd();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Store and retrieve personal information",
    ));
}

#[test]
fn test_cli_version_flag() {
    let mut cmd = wsw_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_add_person() {
    let (db_path, _temp) = temp_db_arg();
    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "add",
        "Test User",
        "email=test@example.com",
        "role=Developer",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Added: Test User"))
        .stdout(predicate::str::contains("ID:"));
}

#[test]
fn test_add_and_get_person() {
    let (db_path, _temp) = temp_db_arg();

    // Add person
    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "add",
        "Alice Smith",
        "email=alice@example.com",
        "role=Engineer",
    ]);
    cmd.assert().success();

    // Get person
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "get", "Alice"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Alice Smith"))
        .stdout(predicate::str::contains("alice@example.com"))
        .stdout(predicate::str::contains("Engineer"));
}

#[test]
fn test_get_person_shorthand() {
    let (db_path, _temp) = temp_db_arg();

    // Add person
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Bob Jones"]);
    cmd.assert().success();

    // Get using shorthand (no subcommand)
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "Bob"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Bob Jones"));
}

#[test]
fn test_set_person() {
    let (db_path, _temp) = temp_db_arg();

    // Add person
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Charlie", "email=old@example.com"]);
    cmd.assert().success();

    // Update person
    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "set",
        "Charlie",
        "email=new@example.com",
        "github=charlie",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Updated: Charlie"));

    // Verify update
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "get", "Charlie"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("new@example.com"))
        .stdout(predicate::str::contains("github"));
}

#[test]
fn test_list_people() {
    let (db_path, _temp) = temp_db_arg();

    // Add people
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Alice"]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Bob"]);
    cmd.assert().success();

    // List people
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn test_list_shows_note_counts() {
    let (db_path, _temp) = temp_db_arg();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Alice"]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "note", "Alice", "First note"]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "note", "Alice", "Second note"]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("2 notes"));
}

#[test]
fn test_list_json_output() {
    let (db_path, _temp) = temp_db_arg();

    // Add person
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "JSON Test", "field=value"]);
    cmd.assert().success();

    // List with JSON output
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "list", "--json"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"name\":"))
        .stdout(predicate::str::contains("\"id\":"))
        .stdout(predicate::str::contains("JSON Test"));
}

#[test]
fn test_search() {
    let (db_path, _temp) = temp_db_arg();

    // Add people
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Developer One", "role=Developer"]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Manager One", "role=Manager"]);
    cmd.assert().success();

    // Search
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "search", "Developer"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Developer One"))
        .stdout(predicate::str::contains("Found 1 match"));
}

#[test]
fn test_search_by_field() {
    let (db_path, _temp) = temp_db_arg();

    // Add people
    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "add",
        "Alice",
        "role=Engineer",
        "team=Platform",
    ]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "add",
        "Bob",
        "role=Manager",
        "team=Growth",
    ]);
    cmd.assert().success();

    // Search by field
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "search", "-f", "role", "Engineer"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Found 1 match"));
}

#[test]
fn test_search_finds_note_content() {
    let (db_path, _temp) = temp_db_arg();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Alice"]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "note",
        "Alice",
        "Discuss vector database rollout",
    ]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "search", "vector"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Discuss vector database rollout"));
}

#[test]
fn test_search_by_note_field() {
    let (db_path, _temp) = temp_db_arg();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Alice", "role=Engineer"]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Bob", "role=vector"]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "note",
        "Alice",
        "Discuss vector database rollout",
    ]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "search", "-f", "notes", "vector"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Discuss vector database rollout"))
        .stdout(predicate::str::contains("Bob").not());
}

#[test]
fn test_note_and_log() {
    let (db_path, _temp) = temp_db_arg();

    // Add person
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Note Test"]);
    cmd.assert().success();

    // Add note
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "note", "Note Test", "This is a test note"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Added note"));

    // View log
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "log", "Note Test"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is a test note"));
}

#[test]
fn test_get_includes_notes() {
    let (db_path, _temp) = temp_db_arg();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Note Detail Test"]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "note",
        "Note Detail Test",
        "Visible from get",
    ]);
    cmd.assert().success();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "get", "Note Detail Test"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Notes:"))
        .stdout(predicate::str::contains("Visible from get"));
}

#[test]
fn test_rm_person_with_yes_flag() {
    let (db_path, _temp) = temp_db_arg();

    // Add person
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "To Delete"]);
    cmd.assert().success();

    // Remove with -y flag
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "rm", "To Delete", "-y"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Removed"));

    // Verify deletion
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "get", "To Delete"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Error:")));
}

#[test]
fn test_rm_field() {
    let (db_path, _temp) = temp_db_arg();

    // Add person with fields
    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "add",
        "Field Test",
        "email=test@example.com",
        "phone=555-1234",
    ]);
    cmd.assert().success();

    // Remove field with -y flag
    let mut cmd = wsw_cmd();
    cmd.args([
        "--db",
        &db_path,
        "rm",
        "Field Test",
        "--field",
        "email",
        "-y",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Removed field"));

    // Verify field removal
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "get", "Field Test"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("phone"))
        .stdout(predicate::str::contains("555-1234"));
}

#[test]
fn test_get_by_id() {
    let (db_path, _temp) = temp_db_arg();

    // Add person
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "ID Test"]);
    cmd.assert().success();

    // Get by ID (ID should be 1 for first person)
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "get", "--id", "1"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ID Test"));
}

#[test]
fn test_not_found_error() {
    let (db_path, _temp) = temp_db_arg();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "get", "NonExistentPerson"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Error:")));
}

#[test]
fn test_list_limit() {
    let (db_path, _temp) = temp_db_arg();

    // Add multiple people
    for i in 1..=5 {
        let mut cmd = wsw_cmd();
        cmd.args(["--db", &db_path, "add", &format!("Person {}", i)]);
        cmd.assert().success();
    }

    // List with limit
    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "list", "--limit", "3"]);
    cmd.assert().success();
}

#[test]
fn test_invalid_field_format() {
    let (db_path, _temp) = temp_db_arg();

    let mut cmd = wsw_cmd();
    cmd.args(["--db", &db_path, "add", "Test", "invalidfield"]);
    cmd.assert().failure().stderr(
        predicate::str::contains("Invalid field format").or(predicate::str::contains("Error:")),
    );
}
