use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_repl_insert_document_should_succeed() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.insertDocument("testCollection", {"field": 123});
        .exit
        "#,
    )
    .assert()
    .stdout(contains("Document has been inserted successfully."));
}

#[test]
fn test_repl_insert_document_should_succeed_multiple_documents() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.insertDocument("testCollection", {"field": 123});
        db.insertDocument("testCollection", {"field": 456});
        .exit
        "#,
    )
    .assert()
    .stdout(contains("Document has been inserted successfully."));
}

#[test]
fn test_repl_insert_document_should_succeed_valid_structure() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection", {"field": "number"});
        db.insertDocument("testCollection", {"field": 123});
        .exit
        "#,
    )
    .assert()
    .stdout(contains("Document has been inserted successfully."));
}

#[test]
fn test_repl_insert_document_should_fail_collection_not_found() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.insertDocument("testCollection", {"field": 123});
        .exit
        "#,
    )
    .assert()
    .stderr(contains("Error: Failed to insert document: Collection not found: `Collection 'testCollection' not found`"));
}

#[test]
fn test_repl_insert_document_should_fail_invalid_structure() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection", {"field": "string"});
        db.insertDocument("testCollection", {"field": 123});
        .exit
        "#,
    )
    .assert()
    .stderr(contains("Error: Failed to insert document: Document invalid: Document does not match the collection's structure"));
}
