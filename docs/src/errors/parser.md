# Parser Errors (LTX::PARSER::E001-E006)

The parser analyzes the token stream produced by the lexer and constructs an Abstract Syntax Tree (AST) according to the language grammar. This section lists all parser-related errors, their causes, and recommended fixes.

## Error Reference Table

| Code | Variant | Diagnostic Message | Remediation |
|:----:|---------|-------------------|-------------|
| `LTX::PARSER::E001` | `ExpectedToken` | Expected token not found | Check the syntax near the highlighted position — a token is missing or misplaced. |
| `LTX::PARSER::E002` | `UnexpectedEOF` | Unexpected end of file | Close the construct with its matching delimiter before the file end. |
| `LTX::PARSER::E003` | `UnclosedEnvironment` | Environment was not closed | Add a matching `\end{...}` for every `\begin{...}`. |
| `LTX::PARSER::E004` | `MismatchedEnvironment` | Environment closing tag mismatch | Ensure `\end{...}` matches the corresponding `\begin{...}` name. |
| `LTX::PARSER::E005` | `MissingClosingBrace` | Missing closing brace | Add the matching closing brace `}` to terminate the group. |
| `LTX::PARSER::E006` | `UnexpectedEOFWhileParsing` | End of file reached while parsing a structure | Ensure required structures (e.g. `\begin{document}`) are present. |

## Error Categories

### Token Errors (LTX::PARSER::E001-E002)

**LTX::PARSER::E001 - ExpectedToken**

The parser expected a specific token — such as `{`, `}`, or an environment — but found something else or reached the end of the stream.

```latex
% ❌ Incorrect (missing closing brace)
\newcommand{\foo}{Hello

% ✅ Correct
\newcommand{\foo}{Hello}
```

**LTX::PARSER::E002 - UnexpectedEOF**

A construct being parsed (for example inline math) ran to the end of the file without its closing delimiter.

```latex
% ❌ Incorrect
A sentence with $x + y = z$

% ✅ Correct
A sentence with $x + y = z$.
```

### Environment Errors (LTX::PARSER::E003-E004)

**LTX::PARSER::E003 - UnclosedEnvironment**

An environment was opened with `\begin{...}` but never closed with `\end{...}`.

```latex
% ❌ Incorrect
\begin{itemize}
\item First item
\item Second item

% ✅ Correct
\begin{itemize}
\item First item
\item Second item
\end{itemize}
```

**LTX::PARSER::E004 - MismatchedEnvironment**

The environment closing tag doesn't match the opening tag.

```latex
% ❌ Incorrect
\begin{itemize}
\item First item
\end{enumerate}

% ✅ Correct
\begin{itemize}
\item First item
\end{itemize}
```

### Group Errors (LTX::PARSER::E005)

**LTX::PARSER::E005 - MissingClosingBrace**

A braced group `{ ... ` reached the end of the token stream without a matching `}`.

```latex
% ❌ Incorrect
\section{Introduction

% ✅ Correct
\section{Introduction}
```

### Document Structure Errors (LTX::PARSER::E006)

**LTX::PARSER::E006 - UnexpectedEOFWhileParsing**

The top-level document parsing ended before a required structure — such as `\begin{document}` — was found.

```latex
% ❌ Incorrect
\documentclass{article}
Hello world

% ✅ Correct
\documentclass{article}
\begin{document}
Hello world
\end{document}
```

## Diagnostic Example

```bash
🔍 Found 1 issue(s):

LTX::PARSER::E004

  × mismatched environment: expected `\end{itemize}`, found `\end{enumerate}`
    ╭─[main.tex:18:1]
 17 │ \begin{itemize}
 18 │ \end{enumerate}
    · ────────┬───────
    ·         ╰── here
 19 │
    ╰────
  help: Environments must be closed with the same name they were opened with.
```

## Best Practices

1. **Always use matching `\begin` and `\end` pairs** - Keep environments properly nested
2. **Verify command spelling** - Use autocompletion or reference documentation
3. **Load required packages** - Ensure all necessary packages are imported
4. **Close every brace and argument** - Each `{` needs a matching `}`
5. **Structure the document correctly** - Include `\begin{document}` after the preamble

## Related Topics

- [ltx_parser API](../api/parser.md) — the crate that owns these errors
- [Lexer Errors](lexer.md) - Errors during tokenization
- [Config Errors](config.md) - Manifest validation errors
