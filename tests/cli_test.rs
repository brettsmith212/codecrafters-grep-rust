use assert_cmd::Command;

#[test]
fn positive_character_groups_returns_0() {
    let mut cmd = Command::cargo_bin("codecrafters-grep").unwrap();
    cmd.args(["-E", "[abc]"])
        .write_stdin("apple")
        .assert()
        .success();
}

#[test]
fn positive_character_groups_returns_1() {
    let mut cmd = Command::cargo_bin("codecrafters-grep").unwrap();
    cmd.args(["-E", "[abc]"])
        .write_stdin("xyz")
        .assert()
        .failure();
}
