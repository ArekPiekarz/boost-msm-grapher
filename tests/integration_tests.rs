#![allow(non_snake_case)]

#[test]
fn shouldFail_whenNoFilePathIsProvided()
{
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().assert().failure()
        .stderr("Error: \"Please provide a path to a file to analyze.\"\n");
}

#[test]
fn shouldFail_whenTooManyArgumentsAreProvided()
{
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().args(&["file1", "file2"]).assert().failure()
        .stderr("Error: \"Too many arguments passed to program, expected only one with a file path, got 2\"\n");
}

#[test]
fn shouldFail_whenFileCannotBeRead()
{
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg("wrong_file").assert().failure()
        .stderr("Error: \"Failed to read file: \\\"wrong_file\\\", error: No such file or directory (os error 2)\"\n");
}

#[test]
fn shouldFail_whenFileDoesNotHaveTransitionTable()
{
    let file = tempfile::NamedTempFile::new().unwrap();
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Transition table was not found.\"\n");
}
