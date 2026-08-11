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
        no_lint: false,
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
        no_lint: false,
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
        no_lint: false,
    };
    let result = args.execute(&test_ctx());
    assert!(result.is_err());
}

#[test]
fn test_check_command_recursive_checks_every_tex_file_by_default() {
    let dir = tempfile::tempdir().expect("create temp dir");
    fs::create_dir(dir.path().join("src")).expect("create src dir");
    fs::write(
        dir.path().join("main.tex"),
        "\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}\n",
    )
    .expect("write main.tex");
    fs::write(
        dir.path().join("src/chapter.tex"),
        "\\documentclass{article}\n\\begin{document}\nHi\n\\end{document}\n",
    )
    .expect("write src/chapter.tex");
    fs::write(
        dir.path().join("src/bad.tex"),
        "\\begin{minipage}\n\\end{minipage}\n\\end{document}\n",
    )
    .expect("write src/bad.tex");
    fs::write(
        dir.path().join("ltx.toml"),
        "[project]\nname = \"demo\"\nmain = \"main.tex\"\n\n[build]\nname = \"demo\"\nengine = \"tectonic\"\n",
    )
    .expect("write ltx.toml");

    let ctx = AppContext {
        manifest_path: Some(dir.path().join("ltx.toml")),
        format: OutputFormat::Human,
        verbose: 0,
    };
    let args = CheckArgs {
        path: None,
        no_lint: false,
    };
    let result = args.execute(&ctx);
    let err = result.expect_err("default check should surface the broken file");
    assert!(
        err.downcast_ref::<CliError>()
            .is_some_and(|e| matches!(e, CliError::DiagnosticsFound)),
        "expected DiagnosticsFound error"
    );
}

#[test]
fn test_check_command_recursive_passes_with_warnings_only() {
    let dir = tempfile::tempdir().expect("create temp dir");
    fs::write(
        dir.path().join("main.tex"),
        "\\documentclass{article}\n\\begin{document}\n\\label{fig:x}\n\\end{document}\n",
    )
    .expect("write main.tex");
    fs::write(
        dir.path().join("ltx.toml"),
        "[project]\nname = \"demo\"\nmain = \"main.tex\"\n\n[build]\nname = \"demo\"\nengine = \"tectonic\"\n",
    )
    .expect("write ltx.toml");

    let ctx = AppContext {
        manifest_path: Some(dir.path().join("ltx.toml")),
        format: OutputFormat::Human,
        verbose: 0,
    };
    let args = CheckArgs {
        path: None,
        no_lint: false,
    };
    assert!(
        args.execute(&ctx).is_ok(),
        "warnings alone should still pass the check"
    );
}

#[test]
fn test_check_command_recursive_with_no_tex_files_fails() {
    let dir = tempfile::tempdir().expect("create temp dir");
    fs::write(
        dir.path().join("ltx.toml"),
        "[project]\nname = \"demo\"\nmain = \"main.tex\"\n\n[build]\nname = \"demo\"\nengine = \"tectonic\"\n",
    )
    .expect("write ltx.toml");
    let ctx = AppContext {
        manifest_path: Some(dir.path().join("ltx.toml")),
        format: OutputFormat::Human,
        verbose: 0,
    };
    let args = CheckArgs {
        path: None,
        no_lint: false,
    };
    assert!(
        args.execute(&ctx).is_err(),
        "a project with no `.tex` files should fail the check"
    );
}

#[test]
fn test_check_command_path_checks_a_single_file() {
    let dir = tempfile::tempdir().expect("create temp dir");
    fs::write(
        dir.path().join("main.tex"),
        "\\begin{minipage}\n\\end{minipage}\n\\end{document}\n",
    )
    .expect("write main.tex (bad on purpose)");
    fs::write(
        dir.path().join("good.tex"),
        "\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}\n",
    )
    .expect("write good.tex");
    fs::write(
        dir.path().join("ltx.toml"),
        "[project]\nname = \"demo\"\nmain = \"main.tex\"\n\n[build]\nname = \"demo\"\nengine = \"tectonic\"\n",
    )
    .expect("write ltx.toml");

    let ctx = AppContext {
        manifest_path: Some(dir.path().join("ltx.toml")),
        format: OutputFormat::Human,
        verbose: 0,
    };
    let args = CheckArgs {
        path: Some(dir.path().join("good.tex")),
        no_lint: false,
    };
    assert!(
        args.execute(&ctx).is_ok(),
        "-p should check only the given file, ignoring the broken main.tex"
    );
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

#[test]
fn test_cli_parse_watch_subcommand() {
    use clap::Parser;
    let cli = Cli::try_parse_from(["ltx", "watch"]).expect("parse should succeed");
    match cli.command {
        Command::Watch(_) => {}
        other => panic!("expected Command::Watch, got {other:?}"),
    }
}
