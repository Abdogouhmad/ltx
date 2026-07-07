//! Combined examples demonstrating Parser errors from `LTX::E100` to `LTX::E114`.
use ltx_diagnostics::errors::ParserError;
mod common;
use common::ExampleRunner;

/// A single source containing unique snippet violations for each parser error.
const MULTI_ERROR_SOURCE: &str = r"% E101: Duplicate Document Class (Assume class already defined earlier)
\documentclass{book}

% E102: Unknown Command
\unknowncmd{hello}

% E103: Undefined Environment
\begin{fakeenv}
\end{fakeenv}

% E104: Unclosed Environment
\begin{center}
Missing end statement.

% E105: Mismatched End Environment
\begin{flushleft}
Text formatting...
\end{flushright}

% E106: Missing Required Argument
\label{}

% E107: Too Many Arguments
\section{Title}{ExtraArg1}

% E108: Unexpected Argument
\clearpage{UnexpectedArg}

% E109: Invalid Optional Argument
\documentclass[invalid=true=malformed]{article}

% E110: Unexpected End Environment
\end{quote}

% E111: Invalid Command Context
This is standard text \usepackage{amsmath} outside the preamble.

% E112: Invalid Macro Definition
\newcommand{\my_bad_macro}[invalid_param]{text}

% E113: Circular Macro Expansion
\newcommand{\looping}{\looping}

% E114: Recursive Input Detected
\input{parser_error_examples.rs}
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instantiate your single runner instance with the unified source text
    let runner = ExampleRunner::new(MULTI_ERROR_SOURCE.to_string());

    // -----------------------------------------------------------------
    // LTX::E100 - Missing Document Class
    // -----------------------------------------------------------------
    // For E100, we intentionally use an empty file to trigger a missing declaration error
    let empty_runner = ExampleRunner::new("% Just comments here\nHello World");
    empty_runner.trigger_and_render("Hello World", |s| ParserError::MissingDocumentClass {
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E101 - Duplicate Document Class
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\documentclass{book}", |s| {
        ParserError::DuplicateDocumentClass {
            found: std::borrow::Cow::Borrowed("book"),
            span: s,
        }
    })?;

    // -----------------------------------------------------------------
    // LTX::E102 - Unknown Command
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\unknowncmd", |s| ParserError::UnknownCommand {
        found: std::borrow::Cow::Borrowed("\\unknowncmd"),
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E103 - Undefined Environment
    // -----------------------------------------------------------------
    runner.trigger_and_render("fakeenv", |s| ParserError::UndefinedEnvironment {
        found: std::borrow::Cow::Borrowed("fakeenv"),
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E104 - Unclosed Environment
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\begin{center}", |s| ParserError::UnclosedEnvironment {
        found: std::borrow::Cow::Borrowed("center"),
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E105 - Mismatched End Environment
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\end{flushright}", |s| ParserError::MismatchedEndEnv {
        expected: std::borrow::Cow::Borrowed("flushleft"),
        found: std::borrow::Cow::Borrowed("flushright"),
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E106 - Missing Required Argument
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\label{}", |s| ParserError::MissingRequiredArgument {
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E107 - Too Many Arguments
    // -----------------------------------------------------------------
    runner.trigger_and_render("{ExtraArg1}", |s| ParserError::TooManyArguments { span: s })?;

    // -----------------------------------------------------------------
    // LTX::E108 - Unexpected Argument
    // -----------------------------------------------------------------
    runner.trigger_and_render("{UnexpectedArg}", |s| ParserError::UnexpectedArgument {
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E109 - Invalid Optional Argument
    // -----------------------------------------------------------------
    runner.trigger_and_render("[invalid=true=malformed]", |s| {
        ParserError::InvalidOptionalArgument { span: s }
    })?;

    // -----------------------------------------------------------------
    // LTX::E110 - Unexpected End Environment
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\end{quote}", |s| ParserError::UnexpectedEndEnvironment {
        found: std::borrow::Cow::Borrowed("quote"),
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E111 - Invalid Command Context
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\usepackage{amsmath}", |s| {
        ParserError::InvalidCommandContext { span: s }
    })?;

    // -----------------------------------------------------------------
    // LTX::E112 - Invalid Macro Definition
    // -----------------------------------------------------------------
    runner.trigger_and_render("[invalid_param]", |s| ParserError::InvalidMacroDefinition {
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E113 - Circular Macro Expansion
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\looping}", |s| ParserError::CircularMacroExpansion {
        found: std::borrow::Cow::Borrowed("looping"),
        span: s,
    })?;

    // -----------------------------------------------------------------
    // LTX::E114 - Recursive Input Detected
    // -----------------------------------------------------------------
    runner.trigger_and_render("parser_error_examples.rs", |s| {
        ParserError::RecursiveInputDetected {
            found: std::borrow::Cow::Borrowed("parser_error_examples.rs"),
            span: s,
        }
    })?;

    Ok(())
}
