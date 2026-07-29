#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

use ltx_cli::cli::{Cli, Command};
use ltx_cli::commands::check::CheckArgs;
use ltx_cli::commands::new::NewArgs;
use ltx_cli::ctx::{AppContext, CliCommand, OutputFormat};
use ltx_cli::error::CliError;
use ltx_cli::exit_code;
use ltx_config::{BibLayout, CompilerEngine, ScaffoldOptions, SrcLayout};
use std::fs;
use std::path::PathBuf;

fn test_ctx() -> AppContext {
    AppContext {
        manifest_path: None,
        format: OutputFormat::Human,
        verbose: 0,
    }
}

#[test]
fn test_exit_code_constants() {
    assert_eq!(exit_code::SUCCESS, 0);
    assert_eq!(exit_code::ERROR, 1);
    assert_eq!(exit_code::FILE_NOT_FOUND, 2);
    assert_eq!(exit_code::INVALID_INPUT, 3);
    assert_eq!(exit_code::DIAGNOSTICS_FOUND, 4);
}

#[test]
fn test_check_command_no_file() {
    let args = CheckArgs { path: None };
    let result = args.execute(&test_ctx());
    assert!(result.is_err());
}

#[test]
fn test_check_command_valid_file() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let file_path = dir.path().join("valid.tex");
    fs::write(
        &file_path,
        "\\documentclass{article}\n\\begin{document}\nHello world!\n\\end{document}\n",
    )
    .expect("write temp file");

    let args = CheckArgs {
        path: Some(file_path),
    };
    let result = args.execute(&test_ctx());
    assert!(result.is_ok());
}

#[test]
fn test_check_command_invalid_tex() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let file_path = dir.path().join("bad.tex");
    fs::write(
        &file_path,
        "\\documentclass{article}\n\\begin{document}\n\\begin{wrong}\n\\end{right}\n\\end{document}\n",
    )
    .expect("write temp file");

    let args = CheckArgs {
        path: Some(file_path),
    };
    let result = args.execute(&test_ctx());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.downcast_ref::<CliError>()
            .is_some_and(|e| matches!(e, CliError::DiagnosticsFound)),
        "expected DiagnosticsFound error"
    );
}

#[test]
fn test_check_command_nonexistent_file() {
    let args = CheckArgs {
        path: Some(PathBuf::from(
            "/tmp/this_file_definitely_does_not_exist_ltx_test.tex",
        )),
    };
    let result = args.execute(&test_ctx());
    assert!(result.is_err());
}

#[test]
fn test_manifest_error_display() {
    let io_err = CliError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "file missing",
    ));
    let msg = io_err.to_string();
    assert!(
        msg.contains("I/O error"),
        "expected 'I/O error' in display, got: {msg}"
    );

    let utf8_err = String::from_utf8(vec![0xFF, 0xFE]).unwrap_err();
    let cli_err = CliError::NotUtf8(utf8_err);
    let msg = cli_err.to_string();
    assert!(
        msg.contains("not valid UTF-8"),
        "expected 'not valid UTF-8' in display, got: {msg}"
    );

    let lexer_err = CliError::LexerErrors;
    let msg = lexer_err.to_string();
    assert!(
        msg.contains("lexer errors"),
        "expected 'lexer errors' in display, got: {msg}"
    );

    let diag_err = CliError::DiagnosticsFound;
    let msg = diag_err.to_string();
    assert!(
        msg.contains("diagnostics"),
        "expected 'diagnostics' in display, got: {msg}"
    );
}

#[test]
fn test_scaffold_options_with_enums() {
    let opts = ScaffoldOptions {
        name: "test_project".to_owned(),
        engine: CompilerEngine::XeLaTeX,
        src: SrcLayout::WithSrcDir,
        bib: BibLayout::WithBibDir,
    };
    assert_eq!(opts.name, "test_project");
    assert_eq!(opts.engine, CompilerEngine::XeLaTeX);
    assert_eq!(opts.src, SrcLayout::WithSrcDir);
    assert_eq!(opts.bib, BibLayout::WithBibDir);

    let flat_opts = ScaffoldOptions {
        name: "flat".to_owned(),
        engine: CompilerEngine::LuaLaTeX,
        src: SrcLayout::Flat,
        bib: BibLayout::Flat,
    };
    assert_eq!(flat_opts.src, SrcLayout::Flat);
    assert_eq!(flat_opts.bib, BibLayout::Flat);
}

#[test]
fn test_new_command_args() {
    let args = NewArgs {
        name: "my_paper".to_owned(),
        engine: CompilerEngine::Tectonic,
        bib: true,
        src: false,
    };
    assert_eq!(args.name, "my_paper");
    assert_eq!(args.engine, CompilerEngine::Tectonic);
    assert!(args.bib);
    assert!(!args.src);

    let args2 = NewArgs {
        name: "thesis".to_owned(),
        engine: CompilerEngine::Tectonic,
        bib: false,
        src: true,
    };
    assert_eq!(args2.name, "thesis");
    assert_eq!(args2.engine, CompilerEngine::Tectonic);
    assert!(!args2.bib);
    assert!(args2.src);
}

#[test]
fn test_cli_parse_new_subcommand() {
    use clap::Parser;
    let cli = Cli::try_parse_from(["ltx", "new", "my_project"]).expect("parse should succeed");
    match cli.command {
        Command::New(args) => {
            assert_eq!(args.name, "my_project");
            assert_eq!(args.engine, CompilerEngine::Tectonic);
        }
        other => panic!("expected Command::New, got {other:?}"),
    }
}

#[test]
fn test_cli_parse_new_with_flags() {
    use clap::Parser;
    let cli = Cli::try_parse_from([
        "ltx", "new", "thesis", "--engine", "xelatex", "--bib", "--src",
    ])
    .expect("parse should succeed");
    match cli.command {
        Command::New(args) => {
            assert_eq!(args.name, "thesis");
            assert_eq!(args.engine, CompilerEngine::XeLaTeX);
            assert!(args.bib);
            assert!(args.src);
        }
        other => panic!("expected Command::New, got {other:?}"),
    }
}
