#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

use ltx_config::{
    BibLayout, Build, CompileOptions, CompilerEngine, Engine, LtxManifest, Project,
    ScaffoldOptions, SrcLayout, scaffold,
};
use pretty_assertions::assert_eq;
use std::fs;
use tempfile::tempdir;

fn make_opts(name: &str, src: SrcLayout, bib: BibLayout) -> ScaffoldOptions {
    ScaffoldOptions {
        name: name.to_string(),
        engine: CompilerEngine::PdfLaTeX,
        src,
        bib,
    }
}

#[test]
fn test_scaffold_flat_layout() {
    let dir = tempdir().expect("tempdir");
    let project_dir = dir.path().join("myproject");

    scaffold(
        &project_dir,
        &make_opts("myproject", SrcLayout::Flat, BibLayout::Flat),
    )
    .expect("scaffold should succeed");

    assert!(project_dir.join("main.tex").is_file());
    assert!(project_dir.join("references.bib").is_file());
    assert!(project_dir.join("ltx.toml").is_file());
    assert!(project_dir.join(".gitignore").is_file());
}

#[test]
fn test_scaffold_src_layout() {
    let dir = tempdir().expect("tempdir");
    let project_dir = dir.path().join("mypaper");

    scaffold(
        &project_dir,
        &make_opts("mypaper", SrcLayout::WithSrcDir, BibLayout::Flat),
    )
    .expect("scaffold should succeed");

    assert!(project_dir.join("src/main.tex").is_file());
    assert!(project_dir.join("src/sections").is_dir());
    assert!(!project_dir.join("main.tex").exists());
}

#[test]
fn test_scaffold_bib_layout() {
    let dir = tempdir().expect("tempdir");
    let project_dir = dir.path().join("mythesis");

    scaffold(
        &project_dir,
        &make_opts("mythesis", SrcLayout::Flat, BibLayout::WithBibDir),
    )
    .expect("scaffold should succeed");

    assert!(project_dir.join("bib/references.bib").is_file());
    assert!(!project_dir.join("references.bib").exists());
}

#[test]
fn test_scaffold_already_exists() {
    let dir = tempdir().expect("tempdir");
    let project_dir = dir.path().join("existing");

    fs::create_dir_all(&project_dir).expect("create dir");
    fs::write(project_dir.join("dummy.txt"), "x").expect("write file");

    let result = scaffold(
        &project_dir,
        &make_opts("existing", SrcLayout::Flat, BibLayout::Flat),
    );

    assert!(result.is_err());
}

#[test]
fn test_manifest_roundtrip() {
    let mut project = Project::new("test-project");
    project.set_main("src/main.tex");

    let build = Build::new("test-project", CompilerEngine::XeLaTeX);
    let manifest = LtxManifest::new(project).with_build(build);

    let toml_str = manifest.to_toml().expect("to_toml");
    assert!(toml_str.contains("test-project"));
    assert!(toml_str.contains("xelatex"));

    let dir = tempdir().expect("tempdir");
    fs::create_dir_all(dir.path().join("src")).expect("create src");
    fs::write(dir.path().join("src/main.tex"), "% demo\n").expect("write main.tex");

    let toml_path = dir.path().join("ltx.toml");
    manifest.write(&toml_path).expect("write");

    let loaded = LtxManifest::from_file(&toml_path).expect("from_file");
    assert_eq!(loaded.project.name, "test-project");
    assert_eq!(loaded.project.get_main_project(), Some("src/main.tex"));
    let build = loaded.build.expect("build section should exist");
    assert_eq!(build.engine(), CompilerEngine::XeLaTeX);
}

#[test]
fn test_compiler_engine_from_str() {
    assert_eq!(
        "pdflatex".parse::<CompilerEngine>().unwrap(),
        CompilerEngine::PdfLaTeX
    );
    assert_eq!(
        "xelatex".parse::<CompilerEngine>().unwrap(),
        CompilerEngine::XeLaTeX
    );
    assert_eq!(
        "lualatex".parse::<CompilerEngine>().unwrap(),
        CompilerEngine::LuaLaTeX
    );
    assert_eq!(
        "tectonic".parse::<CompilerEngine>().unwrap(),
        CompilerEngine::Tectonic
    );
    assert_eq!(
        "PDFLATEX".parse::<CompilerEngine>().unwrap(),
        CompilerEngine::PdfLaTeX
    );

    let bad = "garbage".parse::<CompilerEngine>();
    assert!(bad.is_err());
}

#[test]
fn test_compiler_engine_display() {
    assert_eq!(CompilerEngine::PdfLaTeX.to_string(), "pdflatex");
    assert_eq!(CompilerEngine::XeLaTeX.to_string(), "xelatex");
    assert_eq!(CompilerEngine::LuaLaTeX.to_string(), "lualatex");
    assert_eq!(CompilerEngine::Tectonic.to_string(), "tectonic");
}

#[test]
fn test_engine_new() {
    let engine = Engine::new(CompilerEngine::LuaLaTeX);
    assert_eq!(engine.compiler(), CompilerEngine::LuaLaTeX);
    assert!(engine.args().is_none());
}

#[test]
fn test_compile_options_defaults() {
    let opts = CompileOptions::default();
    assert!(opts.keep_logs);
    assert!(!opts.keep_intermediates);
    assert!(opts.synctex);
    assert!(!opts.only_cached);
}

#[test]
fn test_build_default_options_when_absent() {
    let build = Build::new("paper", CompilerEngine::Tectonic);
    assert_eq!(build.compile_options(), CompileOptions::default());
}

#[test]
fn test_compile_options_partial_toml() {
    let toml_str = r#"
        [project]
        name = "paper"

        [build]
        name = "paper"
        engine = "tectonic"

        [build.options]
        keep_logs = false
    "#;

    let manifest: LtxManifest = toml::from_str(toml_str).expect("parse toml");
    let opts = manifest.build.expect("build").compile_options();

    assert!(!opts.keep_logs);
    assert!(!opts.keep_intermediates);
    assert!(opts.synctex);
    assert!(!opts.only_cached);
}

#[test]
fn test_compile_options_roundtrip() {
    let mut build = Build::new("paper", CompilerEngine::Tectonic);
    build.set_options(CompileOptions {
        keep_logs: false,
        keep_intermediates: true,
        synctex: false,
        only_cached: true,
    });

    let mut project = Project::new("paper");
    project.set_main("main.tex");
    let manifest = LtxManifest::new(project).with_build(build);
    let toml_str = manifest.to_toml().expect("to_toml");
    assert!(toml_str.contains("keep_logs"));
    assert!(toml_str.contains("only_cached"));

    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("main.tex"), "% demo\n").expect("write main.tex");
    let path = dir.path().join("ltx.toml");
    fs::write(&path, toml_str).expect("write toml");

    let loaded = LtxManifest::from_file(&path).expect("from_file");
    let opts = loaded.build.expect("build").compile_options();
    assert!(!opts.keep_logs);
    assert!(opts.keep_intermediates);
    assert!(!opts.synctex);
    assert!(opts.only_cached);
}

#[test]
fn test_validate_accepts_valid_manifest() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("main.tex"), "% demo\n").expect("write main.tex");

    let toml_str = r#"
        [project]
        name = "paper"
        main = "main.tex"

        [build]
        name = "paper"
        engine = "tectonic"
    "#;

    let manifest = ltx_config::validate_manifest(toml_str, dir.path()).expect("valid manifest");
    assert_eq!(manifest.project.name, "paper");
    assert_eq!(
        manifest.build.expect("build").engine(),
        CompilerEngine::Tectonic
    );
}

#[test]
fn test_validate_rejects_unknown_build_key() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("main.tex"), "% demo\n").expect("write main.tex");

    let toml_str = r#"
        [project]
        name = "paper"
        main = "main.tex"

        [build]
        name = "paper"
        engine = "tectonic"

        [build.option]
        keep_logs = false
    "#;

    let err = ltx_config::validate_manifest(toml_str, dir.path()).expect_err("unknown key");
    assert!(err.to_string().contains("invalid `ltx.toml`"));
    assert!(err.to_string().contains("option"));
}

#[test]
fn test_validate_rejects_missing_main() {
    let toml_str = r#"
        [project]
        name = "paper"

        [build]
        name = "paper"
        engine = "tectonic"
    "#;

    let err =
        ltx_config::validate_manifest(toml_str, std::path::Path::new(".")).expect_err("no main");
    assert!(err.to_string().contains("missing `main`"));
    assert!(err.to_string().contains("[project]"));
}

#[test]
fn test_validate_rejects_missing_build_section() {
    let toml_str = r#"
        [project]
        name = "paper"
        main = "main.tex"
    "#;

    let err =
        ltx_config::validate_manifest(toml_str, std::path::Path::new(".")).expect_err("no build");
    assert!(err.to_string().contains("missing `[build]`"));
}

#[test]
fn test_validate_rejects_missing_build_name() {
    let toml_str = r#"
        [project]
        name = "paper"
        main = "main.tex"

        [build]
        engine = "tectonic"
    "#;

    let err =
        ltx_config::validate_manifest(toml_str, std::path::Path::new(".")).expect_err("no name");
    assert!(err.to_string().contains("missing `name`"));
}

#[test]
fn test_validate_rejects_missing_main_file() {
    let toml_str = r#"
        [project]
        name = "paper"
        main = "does-not-exist.tex"

        [build]
        name = "paper"
        engine = "tectonic"
    "#;

    let err = ltx_config::validate_manifest(toml_str, std::path::Path::new("."))
        .expect_err("file does not exist");
    assert!(err.to_string().contains("main file not found"));
}

#[test]
fn test_from_file_validates() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("ltx.toml");

    fs::write(
        &path,
        r#"
        [project]
        name = "paper"
        main = "main.tex"

        [build]
        name = "paper"
        engine = "tectonic"

        [build.option]
        keep_logs = false
        "#,
    )
    .expect("write toml");

    let err = LtxManifest::from_file(&path).expect_err("typo should be rejected");
    assert!(err.to_string().contains("unknown field `option`"));

    let missing = dir.path().join("missing.toml");
    let err = LtxManifest::from_file(&missing).expect_err("missing file");
    assert!(err.to_string().contains("failed to read"));
}
