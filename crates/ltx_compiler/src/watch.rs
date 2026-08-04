// ltx_compiler/src/watch.rs
use notify::{EventKind, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, DebouncedEvent, new_debouncer};
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
    time::Duration,
};

use crate::build::build;
use crate::config::CompilerConfig;
use crate::error::CompilerError as WatchError;

/// Configuration for a watch session. Holds a pre-parsed config so we never
/// re-read `ltx.toml` on every compile — only when it actually changes.
pub struct WatchConfig {
    root: PathBuf,
    manifest_path: PathBuf,
    config: CompilerConfig,
    /// Paths `notify` actually watches, with their watch mode. Kept as narrow
    /// as the project layout allows — the `src/` root for structured projects
    /// (`ltx new --src`) or the main file alone for single-structure ones
    /// (`ltx new`). The compiler's own `target/` output is never watched, so
    /// a build can never feed back into a rebuild loop.
    targets: Vec<(PathBuf, RecursiveMode)>,
    debounce: Duration,
}

impl WatchConfig {
    /// Builds a watch session from an already-parsed config.
    ///
    /// The watched paths are derived from `[project].main`: a single-file
    /// project (`ltx new`) watches just `main.tex`, while a structured
    /// project (`ltx new --src`) watches `src/` recursively. The manifest is
    /// always watched so config edits reload correctly.
    #[must_use]
    pub fn new(root: PathBuf, manifest_path: PathBuf, config: CompilerConfig) -> Self {
        let targets = Self::watch_targets(&root, &manifest_path, config.main_file());
        Self {
            root,
            manifest_path,
            config,
            targets,
            debounce: Duration::from_millis(300),
        }
    }

    /// Computes the narrow set of paths to watch. The manifest is always
    /// watched; a main file under `src/` pulls in the whole `src/` tree,
    /// while a flat project watches the main file itself.
    fn watch_targets(
        root: &Path,
        manifest_path: &Path,
        main_file: &str,
    ) -> Vec<(PathBuf, RecursiveMode)> {
        let mut targets = Vec::with_capacity(2);
        targets.push((manifest_path.to_path_buf(), RecursiveMode::NonRecursive));

        if main_file.starts_with("src/") || main_file == "src" {
            targets.push((root.join("src"), RecursiveMode::Recursive));
        } else {
            targets.push((root.join(main_file), RecursiveMode::NonRecursive));
        }
        targets
    }

    /// Watches the configured paths and rebuilds the project whenever a
    /// relevant file (`.tex`, `.sty`, `.cls`, `.bib`) or the manifest changes.
    ///
    /// Compiles once on startup, then blocks until the watch channel closes.
    ///
    /// # Errors
    ///
    /// Returns an error if the watcher cannot be initialized or if the watch
    /// channel disconnects before the session ends.
    pub fn run_watch(&mut self) -> Result<(), WatchError> {
        let (tx, rx) = mpsc::channel::<DebounceEventResult>();
        let mut debouncer = new_debouncer(self.debounce, None, tx)?;

        // Only watch source targets, never `target/` output, so the build's
        // own writes cannot feed back into a rebuild loop.
        for (path, mode) in &self.targets {
            debouncer.watch(path, *mode)?;
        }

        self.compile_once();

        for result in rx {
            match result {
                Ok(events) => {
                    // Drop pure-read events. The engine opens/reads the source
                    // during every compile, which notify reports as Access
                    // (open/close) events — without this filter a build would
                    // retrigger itself forever. Only real writes (Modify,
                    // Create, Remove) rebuild.
                    let meaningful: Vec<&DebouncedEvent> =
                        events.iter().filter(|e| Self::is_meaningful(e)).collect();

                    if meaningful.is_empty() {
                        continue;
                    }

                    let touched_manifest = meaningful
                        .iter()
                        .any(|e| e.paths.iter().any(|p| p == &self.manifest_path));

                    if touched_manifest {
                        if let Err(e) = self.reload_manifest() {
                            eprintln!("failed to reload {}: {e:?}", self.manifest_path.display());
                            continue;
                        }
                        self.rebuild(&meaningful);
                    } else if meaningful.iter().any(|e| Self::is_relevant(e)) {
                        self.rebuild(&meaningful);
                    }
                }
                Err(errors) => {
                    for e in errors {
                        eprintln!("watch error: {e}");
                    }
                }
            }
        }

        Err(WatchError::ChannelClosed)
    }

    fn reload_manifest(&mut self) -> miette::Result<()> {
        let manifest = ltx_config::LtxManifest::from_file(&self.manifest_path)?;
        self.config = CompilerConfig::from_manifest(&manifest)?;
        Ok(())
    }

    /// Logs the paths that triggered the rebuild, so a self-triggered loop
    /// (e.g. compiler output landing inside a watched dir) is easy to spot,
    /// then compiles.
    fn rebuild(&self, events: &[&DebouncedEvent]) {
        let changed: Vec<String> = events
            .iter()
            .flat_map(|e| e.paths.iter())
            .map(|p| p.display().to_string())
            .collect();
        eprintln!("watch: {} -> rebuilding", changed.join(", "));
        self.compile_once();
    }

    #[must_use]
    #[inline]
    const fn is_meaningful(event: &DebouncedEvent) -> bool {
        matches!(
            event.event.kind,
            EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
        )
    }

    #[must_use]
    #[inline]
    fn is_relevant(event: &DebouncedEvent) -> bool {
        event.paths.iter().any(|p| Self::is_relevant_path(p))
    }

    #[must_use]
    fn is_relevant_path(path: &Path) -> bool {
        // Never rebuild from the compiler's own output churn.
        if path.components().any(|c| c.as_os_str() == "target") {
            return false;
        }
        matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("tex" | "sty" | "cls" | "bib")
        )
    }

    fn compile_once(&self) {
        eprintln!("compiling {}...", self.config.main_file());

        if let Err(err) = build(&self.config, &self.root) {
            eprintln!("{err:?}");
        }
    }
}
