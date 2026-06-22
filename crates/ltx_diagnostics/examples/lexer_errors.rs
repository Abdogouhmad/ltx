//! Combined examples demonstrating Lexer errors from `LTX::E001` to `LTX::E010`.
use ltx_diagnostics::errors::LexerError;

mod common;
use common::ExampleRunner;

/// A single source containing unique snippet violations for each lexer error.
const MULTI_ERROR_SOURCE: &str = r"\documentclass{article}
% E001: Unexpected Token
\newcommand{\foo} @invalid

% E002: Unexpected End of File
\begin{document}

% E003: Unmatched Brace
Hello } world!

% E004: Invalid Math Delimiter
An inline equation \( x+y ] is broken.

% E005: Unterminated Argument
\textbf{Bold text missing close brace

% E006: Invalid Escape Sequence
This is a bad macro: \1234invalid

% E007: Invalid Unicode
Simulated sequence here

% E008: Illegal Parameter Character Usage
Using raw # parameter character in regular text.

% E009: Unterminated Verbatim Block
\begin{verbatim}
Some raw code text without an end...

% E010: Invalid Character
Stray control sequence raw: \x07
\end{document}";

fn main() {
    // Instantiate your single runner instance with the unified source text
    let runner = ExampleRunner::new(MULTI_ERROR_SOURCE.to_string());
    // -----------------------------------------------------------------
    // LTX::E001 - Unexpected Token
    // -----------------------------------------------------------------
    runner.trigger_and_render("@invalid", |s| LexerError::UnexpectedToken {
        found: "@".to_string(),
        span: s,
    });

    // -----------------------------------------------------------------
    // LTX::E002 - Unexpected End of File
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\begin{document}", |s| LexerError::UnexpectedEOF {
        found: "document environment".to_string(),
        span: s,
    });

    // -----------------------------------------------------------------
    // LTX::E003 - Unmatched Brace
    // -----------------------------------------------------------------
    runner.trigger_and_render(" }", |s| LexerError::UnmatchedBrace {
        found: "}".to_string(),
        span: s,
    });

    // -----------------------------------------------------------------
    // LTX::E004 - Invalid Math Delimiter
    // -----------------------------------------------------------------
    runner.trigger_and_render(" ]", |s| LexerError::InvalidMathDelimiter {
        found: "\\]".to_string(),
        span: s,
    });

    // -----------------------------------------------------------------
    // LTX::E005 - Unterminated Argument
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\textbf{Bold text missing close brace", |s| {
        LexerError::UnterminatedArgument { span: s }
    });

    // -----------------------------------------------------------------
    // LTX::E006 - Invalid Escape Sequence
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\1234invalid", |s| LexerError::InvalidEscapeSequence {
        span: s,
    });

    // -----------------------------------------------------------------
    // LTX::E007 - Invalid Unicode
    // -----------------------------------------------------------------
    runner.trigger_and_render("sequence", |s| LexerError::InvalidUnicode { span: s });

    // -----------------------------------------------------------------
    // LTX::E008 - Illegal Parameter Character Usage
    // -----------------------------------------------------------------
    runner.trigger_and_render("#", |s| LexerError::IllegalParameterChar { span: s });

    // -----------------------------------------------------------------
    // LTX::E009 - Unterminated Verbatim Block
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\begin{verbatim}", |s| LexerError::UnterminatedVerbatim {
        span: s,
    });

    // -----------------------------------------------------------------
    // LTX::E010 - Invalid Character
    // -----------------------------------------------------------------
    runner.trigger_and_render("\\x07", |s| LexerError::InvalidCharacter {
        found: "\\x07".to_string(),
        span: s,
    });
}
