# linter-feat.md — `ltx_lint`

New crate. Consumes `ltx_parser` AST + raw source. Filtered by
`ltx_config::LintTable`. Emits via `ltx_diagnostics::LtxDiagnosticSink`.

## Traits (`rule.rs`)

```rust
pub trait AstLintRule: Visitor<'static> {
    fn code(&self) -> &'static str;
    fn slug(&self) -> &'static str;
    fn default_severity(&self) -> LtxSeverity;
    fn finish(&mut self, sink: &mut LtxDiagnosticSink) {}
}

pub trait LineLintRule {
    fn code(&self) -> &'static str;
    fn slug(&self) -> &'static str;
    fn default_severity(&self) -> LtxSeverity;
    fn check_line(&mut self, line_no: u32, line: &str, sink: &mut LtxDiagnosticSink) {}
    fn check_source(&mut self, ctx: &LintContext<'_>, sink: &mut LtxDiagnosticSink) {
        for (n, l) in ctx.lines() { self.check_line(n, l, sink); }
    }
}

pub enum LintRule { Ast(Box<dyn AstLintRule>), Line(Box<dyn LineLintRule>) }
```

## Generic rules (cover W001,W002,W004,W005,W009,W013 — no per-code structs)

```rust
pub struct DefineUseTracker {
    code: &'static str,
    slug: &'static str,
    defined: Vec<(String, LtxSpan)>,
    used: HashSet<String>,
    define_matcher: Box<dyn Fn(&Command<'_>) -> Option<(String, LtxSpan)>>,
    use_matcher: Box<dyn Fn(&Command<'_>) -> Option<String>>,
    message: fn(&str) -> Cow<'static, str>,
}
// impl Visitor<'static> { fn visit_command }
// impl AstLintRule { fn finish -> diff defined vs used }

pub struct TableLookupRule {
    code: &'static str,
    slug: &'static str,
    table: &'static [(&'static str, &'static str)],
    matcher: Box<dyn Fn(&Command<'_>) -> Option<(&'static str, LtxSpan)>>,
    findings: Vec<(LtxSpan, Cow<'static, str>)>,
}
// impl Visitor<'static> { fn visit_command }
// impl AstLintRule { fn finish -> drain findings }

pub struct ConsecutiveLineRule {
    code: &'static str,
    slug: &'static str,
    max: u8,
    predicate: fn(&str) -> bool,
    run: u8,
}
// impl LineLintRule { fn check_line -> count run, emit past `max` }
```

Constructors: `unused_label()`, `unused_macro()`, `unused_package()` →
`DefineUseTracker`; `deprecated_command()`, `deprecated_package()` →
`TableLookupRule`; `multiple_blank_lines()` → `ConsecutiveLineRule`.

Static tables: `PACKAGE_COMMANDS: &[(&str,&[&str])]`,
`DEPRECATED_COMMANDS: &[(&str,&str)]`, `DEPRECATED_PACKAGES: &[(&str,&str)]`.

## Bespoke rules (structural, own struct each)

| Code | Slug | Kind | Note |
|---|---|---|---|
| W003 | duplicate-package-import | Ast | `\usepackage{a,b}` dup names |
| W006 | empty-environment | Ast | body is empty/whitespace-only text |
| W007 | empty-section | Ast | needs sibling lookahead, not single-node |
| W012 | redundant-braces | Ast | ungrouped-context `{single-node}` |
| W014 | missing-caption | Ast | figure/table env, no `\caption` descendant |
| W008 | trailing-whitespace | Line | trailing ws per line |
| W010 | long-line | Line | `max_len: u16`, default 100 |
| W011 | mixed-indentation | Line | leading ws mixes tab+space |
| W015 | todo-comment | Line | **BLOCKED** — needs lexer `Comment` token, stub `#[ignore]` |

## Registry (`registry.rs`, `context.rs`, `error.rs`)

```rust
pub struct LintContext<'src> {
    pub document: &'src Document<'src>,
    pub source_map: Arc<LtxSourceMap>,
    pub raw_source: &'src str,
}
impl<'src> LintContext<'src> {
    pub fn new(...) -> Self;
    pub fn lines(&self) -> impl Iterator<Item = (u32, &'src str)>;
}

pub struct LintRegistry {
    ast_rules: Vec<Box<dyn AstLintRule>>,
    line_rules: Vec<Box<dyn LineLintRule>>,
}
impl LintRegistry {
    pub fn with_defaults() -> Self;
    pub fn filtered(self, table: &LintTable) -> Result<Self, LintError>; // deny > warn > allow
    pub fn run(&mut self, ctx: &LintContext<'_>, sink: &mut LtxDiagnosticSink);
}

pub enum LintError {
    UnknownRule { slug: Cow<'static, str> },
    ConflictingDirective { slug: Cow<'static, str> }, // same slug in deny AND allow -> error, don't guess
}
```

`ALL_RULE_CODES: &[(&str, &str)]` (code, slug) in `lib.rs`, for `ltx code --lint`.

## Build order

1. `rule.rs` traits + `LintRule` enum
2. `DefineUseTracker`, `TableLookupRule`, `ConsecutiveLineRule`
3. `TrailingWhitespace`, `LongLine`, `MixedIndentation`, `EmptyEnvironment`, `RedundantBraces`
4. `DuplicatePackageImport`, `EmptySection`, `MissingCaption`
5. `LintRegistry` + `LintTable` wiring
6. Wire into `ltx_cli` `check` (`no_lint`)
7. `unused_package` off-by-default until `PACKAGE_COMMANDS` is populated
8. `todo_comment` stays stubbed until lexer `Comment` token exists

## Do not

- Do not implement `todo_comment` via raw `%` string search — false-positives
  inside verbatim/math. Leave stubbed.
- Do not enable `unused_package` by default.
- Do not restructure `ltx_parser`/`ltx_lexer` to fit this crate — flag
  blockers instead, don't work around them silently.
