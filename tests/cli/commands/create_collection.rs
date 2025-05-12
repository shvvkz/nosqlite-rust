use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_repl_create_collection_should_succeed() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        .exit
        "#,
    )
    .assert()
    .stdout(contains("Collection 'testCollection' created successfully"));
}

#[test]
fn test_repl_create_collection_should_fail_syntax_error() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection";
        .exit
        "#,
    )
    .assert()
    .stderr(contains("Error: Syntax error: missing closing ')'"));
}

#[test]
fn test_repl_create_collection_should_fail_json_error() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection",);
        .exit
        "#,
    )
    .assert()
    .stderr(contains("Error: Invalid JSON schema"));
}

#[test]
fn test_repl_create_collection_should_fail_already_exists() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.createCollection("testCollection");
        .exit
        "#,
    )
    .assert()
    .stderr(contains(
        "Error: Failed to create collection: Collection already exists: `testCollection`",
    ));
}
