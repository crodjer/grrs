use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::main_binary()?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

#[test]
fn blank_pattern_matches_all() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    let content = "A test\nActual content\nMore content\nAnother test";
    writeln!(file, "{}", content)?;

    let mut cmd = Command::main_binary()?;
    cmd.arg("").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(content));
    Ok(())
}
