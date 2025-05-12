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
        db.listCollections();
        .exit
        "#,
    )
    .assert()
    .stdout(contains(
        "Collection 'testCollection'
  0 document(s)",
    ));
}

#[test]
fn test_repl_create_collection_should_succeed_schema() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection", { "schema": { "name": "string" } });
        db.listCollections();
        .exit
        "#,
    )
    .assert()
    .stdout(contains(
        "Collection 'testCollection'
  Required Structure: {\"schema\":{\"name\":\"string\"}}
  0 document(s)",
    ));
}

#[test]
fn test_repl_create_collection_should_succeed_document() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.insertDocument("testCollection", { "name": "John Doe" });
        db.listCollections();
        .exit
        "#,
    )
    .assert()
    .stdout(contains(
        "Collection \'testCollection\'
  1 document(s)",
    ));
}

#[test]
fn test_repl_create_collection_should_succeed_no_collections() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.listCollections();
        .exit
        "#,
    )
    .assert()
    .stdout(contains("No collections found."));
}
