use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_repl_find_documents_should_succeed() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.insertDocument("testCollection", { "name": "John Doe" });
        db.findDocuments("testCollection");
        .exit
        "#,
    )
    .assert()
    .stdout(contains(
        "{
  \"name\": \"John Doe\"
}",
    ));
}

#[test]
fn test_repl_find_documents_should_succeed_filter() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.insertDocument("testCollection", { "name": "John Doe", "age": 30 });
        db.insertDocument("testCollection", { "name": "Jane Doe", "age": 25 });
        db.findDocuments("testCollection", { "age": 30 });
        .exit
        "#,
    )
    .assert()
    .stdout(contains(
        "{
  \"age\": 30,
  \"name\": \"John Doe\"
}",
    ));
}

#[test]
fn test_repl_find_documents_should_succeed_projection() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.insertDocument("testCollection", { "name": "John Doe", "age": 30 });
        db.insertDocument("testCollection", { "name": "Jane Doe", "age": 25 });
        db.findDocuments("testCollection", { "age": 30 }, { "name": 1 });
        .exit
        "#,
    )
    .assert()
    .stdout(contains(
        "{
  \"name\": \"John Doe\"
}",
    ));
}

#[test]
fn test_repl_find_documents_should_succeed_filter_and_projection() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.insertDocument("testCollection", { "name": "John Doe", "age": 30 , "email": "johndoe@example.com"});
        db.insertDocument("testCollection", { "name": "Jane Doe", "age": 25 , "email": "janedoe@example.com"});
        db.findDocuments("testCollection", { "age": 30 }, { "name": 1 });
        .exit
        "#,
    )
    .assert()
    .stdout(contains(
        "{
  \"name\": \"John Doe\"
}",
    ));
}

#[test]
fn test_repl_find_documents_should_succeed_empty() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.findDocuments("testCollection");
        .exit
        "#,
    )
    .assert()
    .stdout(contains(
        "Collection \'testCollection\' created successfully

Exiting.",
    ));
}

#[test]
fn test_repl_find_documents_should_fail() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection");
        db.insertDocument("testCollection", { "name": "John Doe", "age": 30 , "email": "johndoe@example.com"});
        db.insertDocument("testCollection", { "name": "Jane Doe", "age": 25 , "email": "janedoe@example.com"});
        db.findDocuments("nonExistentCollection");
        .exit
        "#,
    )
    .assert()
    .stderr(contains("Error: Error retrieving documents, Collection not found: `Collection \'nonExistentCollection\' not found`"));
}
