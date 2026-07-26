#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

use std::fs;
use std::path::Path;

use ltx_utils::{create_dir, create_file, resolve_main_file, write_file};
use pretty_assertions::assert_eq;
use tempfile::tempdir;

#[test]
fn test_create_dir_nested() {
    let dir = tempdir().expect("tempdir");
    let nested = dir.path().join("a").join("b").join("c");

    create_dir(&nested).expect("create_dir should succeed");

    assert!(nested.is_dir());
    assert!(dir.path().join("a").is_dir());
    assert!(dir.path().join("a/b").is_dir());
}

#[test]
fn test_create_dir_idempotent() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("x").join("y");

    create_dir(&path).expect("first call");
    create_dir(&path).expect("second call should not fail");

    assert!(path.is_dir());
}

#[test]
fn test_create_file_with_parents() {
    let dir = tempdir().expect("tempdir");
    let file = dir.path().join("deep").join("nested").join("file.tex");

    create_file(&file).expect("create_file should succeed");

    assert!(file.is_file());
    assert_eq!(file.metadata().expect("metadata").len(), 0);
}

#[test]
fn test_write_file_creates_content() {
    let dir = tempdir().expect("tempdir");
    let file = dir.path().join("output.txt");

    write_file(&file, "hello world").expect("write_file should succeed");

    let content = fs::read_to_string(&file).expect("read back");
    assert_eq!(content, "hello world");
}

#[test]
fn test_resolve_main_file_default() {
    let dir = tempdir().expect("tempdir");
    let main_tex = dir.path().join("main.tex");
    fs::write(&main_tex, r"\documentclass{article}").expect("write main.tex");

    let original = std::env::current_dir().expect("cwd");
    std::env::set_current_dir(dir.path()).expect("chdir");

    let none_path: Option<&String> = None;
    let result = resolve_main_file(none_path);

    std::env::set_current_dir(&original).expect("restore cwd");

    let path = result.expect("should resolve");
    assert_eq!(path, Path::new("main.tex"));
}

#[test]
fn test_resolve_main_file_specific() {
    let dir = tempdir().expect("tempdir");
    let custom = dir.path().join("paper.tex");
    fs::write(&custom, "content").expect("write file");

    let result = resolve_main_file(Some(&custom));
    let path = result.expect("should resolve existing file");
    assert_eq!(path, custom);
}

#[test]
fn test_resolve_main_file_not_found() {
    let dir = tempdir().expect("tempdir");
    let missing = dir.path().join("nope.tex");

    let result = resolve_main_file(Some(&missing));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
}
