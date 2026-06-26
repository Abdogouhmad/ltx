# Parser Errors (E100-E199)

The parser analyzes the token stream produced by the lexer and constructs an Abstract Syntax Tree (AST) according to the language grammar. This section lists all parser-related errors, their causes, and recommended fixes.

## Error Reference Table

| Code | Variant | Diagnostic Message | Remediation |
|:----:|---------|-------------------|-------------|
| E100 | `MissingDocumentClass` | Missing document class declaration | Add `\documentclass{...}` at the start of the document. |
| E101 | `DuplicateDocumentClass` | Multiple document classes detected | Keep only one document class declaration per document. |
| E102 | `UnknownCommand` | Command not recognized | Verify command spelling or install the required package. |
| E103 | `UndefinedEnvironment` | Environment does not exist | Verify environment name and ensure required packages are loaded. |
| E104 | `UnclosedEnvironment` | Environment was not closed | Add the matching `\end{...}` statement for every `\begin{...}`. |
| E105 | `MismatchedEndEnv` | Environment closing tag mismatch | Ensure `\end{...}` matches the corresponding `\begin{...}` name. |
| E106 | `MissingRequiredArgument` | Required argument missing | Provide all mandatory arguments for the command. |
| E107 | `TooManyArguments` | Too many arguments supplied | Remove unnecessary arguments or verify command syntax. |
| E108 | `UnexpectedArgument` | Unexpected argument encountered | Verify command syntax and argument expectations. |
| E109 | `InvalidOptionalArgument` | Optional argument is malformed | Verify bracket structure and ensure values are valid. |
| E110 | `UnexpectedEndEnvironment` | Environment closed before opening | Check for missing `\begin{...}` statements. |
| E111 | `InvalidCommandContext` | Command used in an invalid context | Move the command to a valid location or context. |
| E112 | `InvalidMacroDefinition` | Macro definition syntax invalid | Verify macro definition format with correct delimiters. |
| E113 | `CircularMacroExpansion` | Recursive macro expansion detected | Remove recursive macro references that cause infinite loops. |
| E114 | `RecursiveInputDetected` | Recursive file inclusion detected | Break the inclusion cycle by restructuring file imports. |

## Error Categories

### Document Structure Errors (E100-E101)

These errors relate to the overall document structure and organization.

**E100 - MissingDocumentClass**

The document lacks the required `\documentclass` declaration. Every valid document must begin with a document class specification.

```latex
% ❌ Incorrect
\begin{document}
Hello world
\end{document}

% ✅ Correct
\documentclass{article}
\begin{document}
Hello world
\end{document}
```

**E101 - DuplicateDocumentClass**

Multiple `\documentclass` declarations were found. A document should contain exactly one document class declaration.

```latex
% ❌ Incorrect
\documentclass{article}
\documentclass{report}
\begin{document}
Content
\end{document}

% ✅ Correct
\documentclass{article}
\begin{document}
Content
\end{document}
```

### Command and Environment Errors (E102-E105)

These errors occur when the parser encounters unknown or improperly used commands and environments.

**E102 - UnknownCommand**

The parser encountered a command that is not defined in the current context. This often occurs due to:
- Misspelled command names
- Missing package imports
- Commands used in the wrong context

```latex
% ❌ Incorrect (assuming color package not loaded)
\textcolor{red}{Text}

% ✅ Correct
\usepackage{color}
\textcolor{red}{Text}

% Or fix spelling
\textbf{Bold text}  % not \textbold
```

**E103 - UndefinedEnvironment**

An environment name was used that hasn't been defined. Verify:
- Correct spelling of environment name
- Required packages are imported
- Environment is available in the current context

```latex
% ❌ Incorrect (assuming todonotes not loaded)
\begin{todo}
Fix this section
\end{todo}

% ✅ Correct
\usepackage{todonotes}
\begin{todo}
Fix this section
\end{todo}
```

**E104 - UnclosedEnvironment**

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

**E105 - MismatchedEndEnv**

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

### Argument Errors (E106-E109)

These errors involve command arguments that are missing, excessive, or malformed.

**E106 - MissingRequiredArgument**

A command was called without all required arguments.

```latex
% ❌ Incorrect (assuming \section requires an argument)
\section

% ✅ Correct
\section{Introduction}
```

**E107 - TooManyArguments**

More arguments were provided than the command accepts.

```latex
% ❌ Incorrect (if \textit only takes one argument)
\textit{Text}{Extra}

% ✅ Correct
\textit{Text}
```

**E108 - UnexpectedArgument**

An argument was provided where none was expected.

```latex
% ❌ Incorrect (if command doesn't take arguments)
\newline{Extra}

% ✅ Correct
\newline
```

**E109 - InvalidOptionalArgument**

An optional argument (within `[...]`) is malformed or contains invalid values.

```latex
% ❌ Incorrect
\usepackage[invalid-option]{geometry}

% ✅ Correct
\usepackage[margin=1in]{geometry}
```

### Context Errors (E110-E111)

These errors involve environments or commands used in incorrect contexts.

**E110 - UnexpectedEndEnvironment**

An `\end{...}` was encountered without a corresponding `\begin{...}`.

```tex
% ❌ Incorrect
\end{itemize}

% ✅ Correct
\begin{itemize}
\item Content
\end{itemize}
```

**E111 - InvalidCommandContext**

A command is used in a location where it's not allowed.

```latex
% ❌ Incorrect (assuming \section can't be used in math mode)
\[
\section{Title}
\]

% ✅ Correct
\section{Title}
\[
x^2 + y^2 = z^2
\]
```

### Macro and Inclusion Errors (E112-E114)

These errors relate to macro definitions and file inclusion.

**E112 - InvalidMacroDefinition**

The macro definition syntax is incorrect.

```latex
% ❌ Incorrect
\newcommand{\hello}(World)

% ✅ Correct
\newcommand{\hello}{World}
\newcommand{\greet}[1]{Hello #1}
```

**E113 - CircularMacroExpansion**

A macro recursively references itself, which would cause infinite expansion.

```latex
% ❌ Incorrect
\newcommand{\loop}{\loop}  % Infinite recursion

% ✅ Correct
\newcommand{\hello}{Hello \world}
\newcommand{\world}{World}
```

**E114 - RecursiveInputDetected**

A file tries to include itself directly or indirectly.

```latex
% file: main.tex
% ❌ Incorrect
\input{main}  % Self-inclusion

% file: a.tex
% ❌ Incorrect (if both files include each other)
\input{b}

% file: b.tex
\input{a}
```

## Diagnostic Example

```bash
🔍 Found 1 issue(s):

LTX::E105

  × environment closing tag mismatch: expected `flushleft`, found `flushright`
    ╭─[main.tex:18:1]
 17 │ Text formatting...
 18 │ \end{flushright}
    · ────────┬───────
    ·         ╰── here
 19 │
    ╰────
  help: Ensure the environment names match.
```

## Best Practices

1. **Always use matching `\begin` and `\end` pairs** - Keep environments properly nested
2. **Verify command spelling** - Use autocompletion or reference documentation
3. **Load required packages** - Ensure all necessary packages are imported
4. **Check argument counts** - Count required arguments before using commands
5. **Avoid circular references** - Design file structure without cycles
6. **Use an editor with syntax highlighting** - Catch errors early

## Related Topics

- [Lexer Errors](lexer.md) - Errors during tokenization

