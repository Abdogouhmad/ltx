//! Unified diagnostic errors for the Ltx language.
//!
//! Rather than splitting by compiler phase (lexer/parser/linter), errors are
//! grouped by what they mean *to the user writing LaTeX*. A parser and a
//! lexer can both raise `UnmatchedBrace`; the caller shouldn't need to care
//! which pass caught it.
//!
//! Each variant carries a `url()` pointing to a relevant TeX StackExchange
//! search or reference doc, since these are the errors people already
//! search for constantly when writing LaTeX by hand.
//!
//! Code ranges:
//! - `LTX::E0xx` — syntax / tokenization (braces, delimiters, escapes)
//! - `LTX::E1xx` — structural / semantic (undefined commands, environments,
//!   references, packages)
//! - `LTX::W0xx` — lint warnings (reserved)

use crate::LtxSpan;
use miette::Diagnostic;
use std::borrow::Cow;
use thiserror::Error;

/// All diagnosable errors in the Ltx language, regardless of which pass
/// (lexer, parser, linter) detected them.
#[derive(Debug, Diagnostic, Error, Clone)]
pub enum LtxError {
    // ---------------------------------------------------------------
    // LTX::E0xx — syntax / tokenization
    // ---------------------------------------------------------------
    /// **`LTX::E001`: Unexpected Token**
    ///
    /// A token appeared where the grammar didn't expect one — e.g. a stray
    /// symbol in a position reserved for a command name or argument.
    #[error("unexpected token `{found}`")]
    #[diagnostic(
        code(LTX::E001),
        help(
            "Check for invalid characters, malformed commands, or unsupported syntax near the highlighted position."
        ),
        url("https://tex.stackexchange.com/search?q=unexpected+token"),
        severity(Error)
    )]
    UnexpectedToken {
        /// The textual representation of the offending token.
        found: Cow<'static, str>,
        /// Location of the unexpected token.
        #[label("unexpected token")]
        span: LtxSpan,
    },

    /// **`LTX::E002`: Unexpected End of File**
    ///
    /// The file ended while a construct (environment, argument, math mode)
    /// was still open. Mirrors TeX's classic `\dimen` / `File ended while
    /// scanning` family of errors.
    #[error("unexpected end of file reached while parsing `{found}`")]
    #[diagnostic(
        code(LTX::E002),
        help("Ensure all environments, braces, and command arguments are properly closed."),
        url("https://tex.stackexchange.com/search?q=file+ended+while+scanning"),
        severity(Error)
    )]
    UnexpectedEOF {
        /// Description of what structural component was left open.
        found: Cow<'static, str>,
        /// Location of the premature end-of-file.
        #[label("unexpected end of file")]
        span: LtxSpan,
    },

    /// **`LTX::E003`: Unmatched Brace**
    ///
    /// A `{` or `}` has no matching counterpart. Corresponds to TeX's
    /// `Too many }'s` / `Missing } inserted` errors.
    #[error("unmatched brace detected: `{found}`")]
    #[diagnostic(
        code(LTX::E003),
        help("Verify that every opening brace `{{` has a matching closing brace `}}`."),
        url("https://tex.stackexchange.com/search?q=too+many+%7D%27s"),
        severity(Error)
    )]
    UnmatchedBrace {
        /// The stray brace or surrounding text fragment.
        found: Cow<'static, str>,
        /// Location of the unmatched brace.
        #[label("unmatched brace")]
        span: LtxSpan,
    },

    /// **`LTX::E004`: Invalid Math Delimiter**
    ///
    /// Math-mode delimiters (`$`, `$$`, `\(`, `\)`, `\[`, `\]`) are
    /// malformed, mismatched, or nested illegally. Mirrors TeX's
    /// `Missing $ inserted` error.
    #[error("invalid math delimiter detected: `{found}`")]
    #[diagnostic(
        code(LTX::E004),
        help("Verify correct usage of $$, $, \\(, \\), \\[ and \\]."),
        url("https://tex.stackexchange.com/search?q=missing+%24+inserted"),
        severity(Error)
    )]
    InvalidMathDelimiter {
        /// The malformed delimiter sequence.
        found: Cow<'static, str>,
        /// Location of the broken math delimiter.
        #[label("invalid delimiter")]
        span: LtxSpan,
    },

    /// **`LTX::E005`: Unterminated Argument**
    ///
    /// A macro argument (`\section{...`) was opened but never closed
    /// before the surrounding scope ended.
    #[error("command argument not terminated")]
    #[diagnostic(
        code(LTX::E005),
        help("Add the missing closing brace `}}` to complete the macro argument."),
        url("https://tex.stackexchange.com/search?q=argument+of+has+an+extra"),
        severity(Error)
    )]
    UnterminatedArgument {
        /// Location where the unclosed argument began.
        #[label("unterminated argument")]
        span: LtxSpan,
    },

    /// **`LTX::E006`: Invalid Escape Sequence**
    ///
    /// A backslash (`\`) is followed by characters that don't form a valid
    /// command name or primitive symbol.
    #[error("invalid escape sequence")]
    #[diagnostic(
        code(LTX::E006),
        help("Verify the command name directly following the backslash `\\`."),
        url("https://tex.stackexchange.com/search?q=undefined+control+sequence"),
        severity(Error)
    )]
    InvalidEscapeSequence {
        /// Location of the malformed escape sequence.
        #[label("invalid escape sequence")]
        span: LtxSpan,
    },

    /// **`LTX::E007`: Invalid Unicode**
    ///
    /// The source bytes contain corrupted or non-UTF-8 sequences, often
    /// from files saved in a legacy encoding (e.g. Latin-1).
    #[error("invalid UTF-8 sequence detected")]
    #[diagnostic(
        code(LTX::E007),
        help("Ensure your document is saved with UTF-8 encoding."),
        url("https://tex.stackexchange.com/search?q=inputenc+utf8"),
        severity(Error)
    )]
    InvalidUnicode {
        /// Location of the invalid byte sequence.
        #[label("invalid UTF-8 sequence")]
        span: LtxSpan,
    },

    /// **`LTX::E008`: Illegal Parameter Character Usage**
    ///
    /// A raw `#` appears outside of a macro definition's parameter list,
    /// where it must instead be escaped as `\#`.
    #[error("illegal parameter character usage")]
    #[diagnostic(
        code(LTX::E008),
        help(
            "Verify proper use of `\\#` inside body contexts, or arguments within macro definitions."
        ),
        url("https://tex.stackexchange.com/search?q=you+can%27t+use+macro+parameter+character"),
        severity(Error)
    )]
    IllegalParameterChar {
        /// Location of the stray `#`.
        #[label("illegal parameter character")]
        span: LtxSpan,
    },

    /// **`LTX::E009`: Unterminated Verbatim Block**
    ///
    /// A `\begin{verbatim}` (or similar raw-text environment) has no
    /// matching `\end{verbatim}` before the file ends.
    #[error("verbatim environment was not terminated")]
    #[diagnostic(
        code(LTX::E009),
        help(
            "Close the verbatim block explicitly using an `\\end{{verbatim}}` marker before the file end."
        ),
        url("https://tex.stackexchange.com/search?q=verbatim+environment+not+closed"),
        severity(Error)
    )]
    UnterminatedVerbatim {
        /// Location where the verbatim block was opened.
        #[label("unterminated environment")]
        span: LtxSpan,
    },

    /// **`LTX::E010`: Invalid Character**
    ///
    /// An unsupported low-level or non-printable control byte appears in
    /// the source, often from copy-pasted text with hidden characters.
    #[error("invalid character encountered: `{found}`")]
    #[diagnostic(
        code(LTX::E010),
        help("Remove or replace unsupported tokens or invisible control characters."),
        url("https://tex.stackexchange.com/search?q=invalid+character"),
        severity(Error)
    )]
    InvalidCharacter {
        /// Display form of the offending character.
        found: Cow<'static, str>,
        /// Location of the invalid character.
        #[label("invalid character")]
        span: LtxSpan,
    },

    // ---------------------------------------------------------------
    // LTX::E1xx — structural / semantic
    // ---------------------------------------------------------------
    /// **`LTX::E100`: Undefined Control Sequence**
    ///
    /// A command (`\foo`) was used but never defined by the document class,
    /// a loaded package, or the user. The single most common LaTeX error.
    #[error("undefined control sequence `\\{name}`")]
    #[diagnostic(
        code(LTX::E100),
        help("Check for typos, or load the package that defines `\\{name}`."),
        url("https://tex.stackexchange.com/questions/172985/undefined-control-sequence"),
        severity(Error)
    )]
    UndefinedControlSequence {
        /// The command name, without the leading backslash.
        name: Cow<'static, str>,
        /// Location of the undefined command invocation.
        #[label("undefined command")]
        span: LtxSpan,
    },

    /// **`LTX::E101`: Mismatched Environment**
    ///
    /// A `\begin{env}` was closed by a `\end{other}` with a different name,
    /// or an `\end` appeared with no corresponding `\begin`.
    #[error("mismatched environment: expected `\\end{{{expected}}}`, found `\\end{{{found}}}`")]
    #[diagnostic(
        code(LTX::E101),
        help("Environments must be closed with the same name they were opened with."),
        url("https://tex.stackexchange.com/search?q=environment+ended+by"),
        severity(Error)
    )]
    MismatchedEnvironment {
        /// The name of the environment that was actually opened.
        expected: Cow<'static, str>,
        /// The name given in the mismatched `\end{...}`.
        found: Cow<'static, str>,
        /// Location of the mismatched `\end`.
        #[label("mismatched \\end")]
        span: LtxSpan,
    },

    /// **`LTX::E102`: Unclosed Environment**
    ///
    /// A `\begin{env}` was never matched by an `\end{env}` before the file
    /// or enclosing scope ended.
    #[error("environment `{name}` was never closed")]
    #[diagnostic(
        code(LTX::E102),
        help("Add a matching `\\end{{{name}}}` for this environment."),
        url("https://tex.stackexchange.com/search?q=environment+undefined+or+not+closed"),
        severity(Error)
    )]
    UnclosedEnvironment {
        /// The name of the environment that was left open.
        name: Cow<'static, str>,
        /// Location where the environment was opened.
        #[label("opened here")]
        span: LtxSpan,
    },

    /// **`LTX::E103`: Undefined Environment**
    ///
    /// A `\begin{env}` names an environment that isn't defined by the
    /// document class or any loaded package.
    #[error("undefined environment `{name}`")]
    #[diagnostic(
        code(LTX::E103),
        help("Check for typos, or load the package that defines the `{name}` environment."),
        url("https://tex.stackexchange.com/search?q=environment+undefined"),
        severity(Error)
    )]
    UndefinedEnvironment {
        /// The unrecognized environment name.
        name: Cow<'static, str>,
        /// Location of the `\begin{...}` invocation.
        #[label("undefined environment")]
        span: LtxSpan,
    },

    /// **`LTX::E104`: Undefined Reference**
    ///
    /// A `\ref`, `\cite`, `\eqref`, or similar cross-reference command
    /// points to a label that was never defined via `\label` or a
    /// bibliography entry.
    #[error("undefined reference to `{key}`")]
    #[diagnostic(
        code(LTX::E104),
        help(
            "Define a matching `\\label{{{key}}}` (or bibliography entry), then rerun the compiler."
        ),
        url("https://tex.stackexchange.com/search?q=undefined+references"),
        severity(Warning)
    )]
    UndefinedReference {
        /// The label or citation key that could not be resolved.
        key: Cow<'static, str>,
        /// Location of the `\ref`/`\cite` invocation.
        #[label("unresolved reference")]
        span: LtxSpan,
    },

    /// **`LTX::E105`: Missing Package**
    ///
    /// A command or environment requires a package that was never loaded
    /// with `\usepackage`.
    #[error("`\\{command}` requires the `{package}` package")]
    #[diagnostic(
        code(LTX::E105),
        help("Add `\\usepackage{{{package}}}` to your preamble."),
        url("https://tex.stackexchange.com/search?q=undefined+control+sequence+package"),
        severity(Error)
    )]
    MissingPackage {
        /// The command that requires the package.
        command: Cow<'static, str>,
        /// The package name that should be loaded.
        package: Cow<'static, str>,
        /// Location of the command invocation.
        #[label("requires missing package")]
        span: LtxSpan,
    },

    /// **`LTX::E106`: File Not Found**
    ///
    /// An `\input`, `\include`, or `\includegraphics` command references a
    /// file that doesn't exist on disk or in the configured search paths.
    #[error("file not found: `{path}`")]
    #[diagnostic(
        code(LTX::E106),
        help(
            "Check the file path and extension, and that it's relative to the main document or on the TEXINPUTS path."
        ),
        url("https://tex.stackexchange.com/search?q=file+not+found"),
        severity(Error)
    )]
    FileNotFound {
        /// The path as written in the source (unresolved).
        path: Cow<'static, str>,
        /// Location of the `\input`/`\include`/`\includegraphics` call.
        #[label("cannot locate file")]
        span: LtxSpan,
    },

    /// **`LTX::E107`: Misplaced Alignment Tab**
    ///
    /// An `&` alignment character appears outside of a tabular/array-like
    /// environment where it has no meaning.
    #[error("misplaced alignment tab character `&`")]
    #[diagnostic(
        code(LTX::E107),
        help(
            "`&` is only valid inside alignment environments like `tabular`, `array`, or `align`."
        ),
        url("https://tex.stackexchange.com/search?q=misplaced+alignment+tab+character"),
        severity(Error)
    )]
    MisplacedAlignmentTab {
        /// Location of the stray `&`.
        #[label("misplaced `&`")]
        span: LtxSpan,
    },

    /// **`LTX::E108`: Command Redefined**
    ///
    /// `\newcommand` was used for a command name that's already defined;
    /// TeX requires `\renewcommand` in that case.
    #[error("command `\\{name}` already defined")]
    #[diagnostic(
        code(LTX::E108),
        help("Use `\\renewcommand{{\\{name}}}{{...}}` to redefine an existing command."),
        url("https://tex.stackexchange.com/search?q=command+already+defined"),
        severity(Error)
    )]
    CommandAlreadyDefined {
        /// The command name, without the leading backslash.
        name: Cow<'static, str>,
        /// Location of the conflicting `\newcommand`.
        #[label("already defined")]
        span: LtxSpan,
    },
}

impl LtxError {
    /// Extracts the source span from the error.
    #[must_use]
    #[inline]
    pub const fn span(&self) -> LtxSpan {
        match self {
            Self::UnexpectedToken { span, .. }
            | Self::UnexpectedEOF { span, .. }
            | Self::UnmatchedBrace { span, .. }
            | Self::InvalidMathDelimiter { span, .. }
            | Self::UnterminatedArgument { span, .. }
            | Self::InvalidEscapeSequence { span, .. }
            | Self::InvalidUnicode { span, .. }
            | Self::IllegalParameterChar { span, .. }
            | Self::UnterminatedVerbatim { span, .. }
            | Self::InvalidCharacter { span, .. }
            | Self::UndefinedControlSequence { span, .. }
            | Self::MismatchedEnvironment { span, .. }
            | Self::UnclosedEnvironment { span, .. }
            | Self::UndefinedEnvironment { span, .. }
            | Self::UndefinedReference { span, .. }
            | Self::MissingPackage { span, .. }
            | Self::FileNotFound { span, .. }
            | Self::MisplacedAlignmentTab { span, .. }
            | Self::CommandAlreadyDefined { span, .. } => *span,
        }
    }
}
