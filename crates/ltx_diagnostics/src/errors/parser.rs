//! A systematic error for Ltx parser
//!
//! Parser supports the error codes `LTX::E100 -- LTX::E199`

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Errors encountered during the syntactic parsing phase of the Ltx compiler.
///
/// This enum categorizes structural failures, malformed commands, mismatched block
/// environments, and invalid macro layouts. Each variant maps directly to an explicit
/// `LTX::E1xx` code.
#[derive(Debug, Diagnostic, Error, Clone)]
pub enum ParserError {
    /// **`LTX::E100`: Missing Document Class**
    ///
    /// Triggered when a document does not begin with an explicit document configuration line.
    #[error("missing document class declaration")]
    #[diagnostic(
        code(LTX::E100),
        help("Add `\\documentclass{{...}}` at the start of the document."),
        severity(Error)
    )]
    MissingDocumentClass {
        /// The precise source location bounds where a document class definition was expected.
        #[label("expected \\documentclass here")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E101`: Duplicate Document Class**
    ///
    /// Triggered when the compiler detects more than one root document class configuration declaration.
    #[error("multiple document classes detected")]
    #[diagnostic(
        code(LTX::E101),
        help("Keep only one document class declaration."),
        severity(Error)
    )]
    DuplicateDocumentClass {
        /// The textual display value representing the rogue class variant.
        found: String,
        /// The location of the duplicate document class declaration.
        #[label("duplicate declaration")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E102`: Unknown Command**
    ///
    /// Triggered when an escaped identifier sequence is not recognized as a native directive or registered macro.
    #[error("command not recognized: `{found}`")]
    #[diagnostic(
        code(LTX::E102),
        help("Verify spelling or install the required package."),
        severity(Error)
    )]
    UnknownCommand {
        /// The raw string token representation of the rogue macro identifier.
        found: String,
        /// The structural location mapping directly to the unknown macro name.
        #[label("unknown command")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E103`: Undefined Environment**
    ///
    /// Triggered when an environment name passed to a structural scoping marker does not exist.
    #[error("environment does not exist: `{found}`")]
    #[diagnostic(
        code(LTX::E103),
        help("Verify the environment name and package requirements."),
        severity(Error)
    )]
    UndefinedEnvironment {
        /// The target configuration keyword name of the faulty block layer definition.
        found: String,
        /// The location segment identifying the invalid environment label.
        #[label("undefined environment")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E104`: Unclosed Environment**
    ///
    /// Triggered when a structural code scoping block environment is never closed.
    #[error("environment was not closed: `{found}`")]
    #[diagnostic(
        code(LTX::E104),
        help("Add the matching `\\end{{...}}` statement."),
        severity(Error)
    )]
    UnclosedEnvironment {
        /// The identifier key name representing the structural block component left open.
        found: String,
        /// The location bounds where the specific environment segment block opened.
        #[label("environment opened here")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E105`: Mismatched End Environment**
    ///
    /// Triggered when the structural namespace tag of a closing tag differs from its corresponding opening block marker.
    #[error("environment closing tag mismatch: expected `{expected}`, found `{found}`")]
    #[diagnostic(
        code(LTX::E105),
        help("Ensure the environment names match."),
        severity(Error)
    )]
    MismatchedEndEnv {
        /// The exact environment label token name expected by the compiler.
        expected: String,
        /// The unexpected structural name tag that was typed instead.
        found: String,
        /// The precise source index location string tracking the broken closing block marker.
        #[label("mismatched closing tag")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E106`: Missing Required Argument**
    ///
    /// Triggered when a structured layout sequence macro or environment is missing mandatory argument blocks.
    #[error("required argument missing")]
    #[diagnostic(
        code(LTX::E106),
        help("Provide all mandatory arguments."),
        severity(Error)
    )]
    MissingRequiredArgument {
        /// The location bounds identifying the position where the missing item sequence belongs.
        #[label("missing required argument")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E107`: Too Many Arguments**
    ///
    /// Triggered when extra text payloads are supplied to an entity that accepts a strict, smaller count parameters.
    #[error("too many arguments supplied")]
    #[diagnostic(
        code(LTX::E107),
        help("Remove unnecessary arguments."),
        severity(Error)
    )]
    TooManyArguments {
        /// The location mapping directly across the excess parameters.
        #[label("extra argument(s) ignored")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E108`: Unexpected Argument**
    ///
    /// Triggered when an argument block sequence format is added to commands that accept no options or inputs.
    #[error("unexpected argument encountered")]
    #[diagnostic(
        code(LTX::E108),
        help("Verify command syntax."),
        severity(Error)
    )]
    UnexpectedArgument {
        /// The position mapping to the rogue item sequence.
        #[label("unexpected argument")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E109`: Invalid Optional Argument**
    ///
    /// Triggered when optional parameters formatted inside syntax delimiters (`[...]`) contain bad formatting layout.
    #[error("optional argument is malformed")]
    #[diagnostic(
        code(LTX::E109),
        help("Verify bracket structure and values."),
        severity(Error)
    )]
    InvalidOptionalArgument {
        /// The tracking token index location tracing the faulty optional data syntax bracket.
        #[label("malformed optional configuration")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E110`: Unexpected End Environment**
    ///
    /// Triggered when an termination command macro occurs isolated without having a valid, open scope block preceding it.
    #[error("environment closed before opening: `{found}`")]
    #[diagnostic(
        code(LTX::E110),
        help("Check for missing `\\begin{{...}}` statements."),
        severity(Error)
    )]
    UnexpectedEndEnvironment {
        /// The rogue environment string token being turned off illegally.
        found: String,
        /// The precise code window track point pointing to the orphan block statement.
        #[label("stray environment close")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E111`: Invalid Command Context**
    ///
    /// Triggered when a structurally correct layout directive token is called inside a forbidden scope region.
    #[error("command used in an invalid context")]
    #[diagnostic(
        code(LTX::E111),
        help("Move the command to a valid location."),
        severity(Error)
    )]
    InvalidCommandContext {
        /// The tracking location coordinate pointing out the illegal usage context position.
        #[label("invalid context")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E112`: Invalid Macro Definition**
    ///
    /// Triggered when the structural code payload declaring or designing a custom macro has layout errors.
    #[error("macro definition syntax invalid")]
    #[diagnostic(
        code(LTX::E112),
        help("Verify macro definition format."),
        severity(Error)
    )]
    InvalidMacroDefinition {
        /// The range monitoring the faulty macro construction segment.
        #[label("invalid macro syntax layout")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E113`: Circular Macro Expansion**
    ///
    /// Triggered when custom macro parameters resolve directly or transitively back into themselves, causing loops.
    #[error("recursive macro expansion detected: `{found}`")]
    #[diagnostic(
        code(LTX::E113),
        help("Remove recursive macro references."),
        severity(Error)
    )]
    CircularMacroExpansion {
        /// The identifier token string tracing the root component causing the loop.
        found: String,
        /// The coordinate bounding the bad execution link.
        #[label("circular loop link point")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },

    /// **`LTX::E114`: Recursive Input Detected**
    ///
    /// Triggered when a document template import structure loops by loading itself or parents directly.
    #[error("recursive file inclusion detected: `{found}`")]
    #[diagnostic(
        code(LTX::E114),
        help("Break the inclusion cycle."),
        severity(Error)
    )]
    RecursiveInputDetected {
        /// The target filename loop string reference payload track.
        found: String,
        /// The range trace pointing out the cyclical target include directive statement line.
        #[label("recursive inclusion")]
        span: SourceSpan,
        /// The complete source file wrapper context.
        #[source_code]
        src: miette::NamedSource<String>,
    },
}

impl ParserError {
    /// Mutates or rebuilds the internal variant payload to attach source code context.
    #[must_use]
    pub fn with_source(self, span: SourceSpan, src: miette::NamedSource<String>) -> Self {
        match self {
            Self::MissingDocumentClass { .. } => Self::MissingDocumentClass { span, src },
            Self::DuplicateDocumentClass { found, .. } => Self::DuplicateDocumentClass { found, span, src },
            Self::UnknownCommand { found, .. } => Self::UnknownCommand { found, span, src },
            Self::UndefinedEnvironment { found, .. } => Self::UndefinedEnvironment { found, span, src },
            Self::UnclosedEnvironment { found, .. } => Self::UnclosedEnvironment { found, span, src },
            Self::MismatchedEndEnv { expected, found, .. } => Self::MismatchedEndEnv { expected, found, span, src },
            Self::MissingRequiredArgument { .. } => Self::MissingRequiredArgument { span, src },
            Self::TooManyArguments { .. } => Self::TooManyArguments { span, src },
            Self::UnexpectedArgument { .. } => Self::UnexpectedArgument { span, src },
            Self::InvalidOptionalArgument { .. } => Self::InvalidOptionalArgument { span, src },
            Self::UnexpectedEndEnvironment { found, .. } => Self::UnexpectedEndEnvironment { found, span, src },
            Self::InvalidCommandContext { .. } => Self::InvalidCommandContext { span, src },
            Self::InvalidMacroDefinition { .. } => Self::InvalidMacroDefinition { span, src },
            Self::CircularMacroExpansion { found, .. } => Self::CircularMacroExpansion { found, span, src },
            Self::RecursiveInputDetected { found, .. } => Self::RecursiveInputDetected { found, span, src },
        }
    }
}
