use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_repl_create_collection() {
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());

    let mut cmd = Command::cargo_bin("nosqlite-cli").unwrap();
    cmd.arg(&path);

    cmd.write_stdin(
        r#"
        db.createCollection("testCollection")
        .exit
        "#,
    )
    .assert()
    .stdout(contains("Collection 'testCollection' created successfully"));
}
