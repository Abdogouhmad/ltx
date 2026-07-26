#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

use ltx_config::{
    BibLayout, CompilerEngine, Engine, LtxManifest, Project, ScaffoldOptions, SrcLayout, scaffold,
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
    assert!(project_dir.join("config.toml").is_file());
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

    let engine = Engine::new(CompilerEngine::XeLaTeX);
    let manifest = LtxManifest::new(project, engine);

    let toml_str = manifest.to_toml().expect("to_toml");
    assert!(toml_str.contains("test-project"));
    assert!(toml_str.contains("xelatex"));

    let dir = tempdir().expect("tempdir");
    let toml_path = dir.path().join("config.toml");
    manifest.write(&toml_path).expect("write");

    let loaded = LtxManifest::from_file(&toml_path).expect("from_file");
    assert_eq!(loaded.project.name, "test-project");
    assert_eq!(loaded.project.get_main_project(), Some("src/main.tex"));
    assert_eq!(loaded.engine.compiler(), CompilerEngine::XeLaTeX);
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
