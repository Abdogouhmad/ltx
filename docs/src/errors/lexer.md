# Lexer Errors (E000-E099)

The lexer transforms source text into a token stream during the initial compilation phase.
This section enumerates all lexer-related errors, their causes, and recommended resolutions.

## Error Reference Table

| Code | Variant | Diagnostic Message | Remediation |
|:----:|---------|-------------------|-------------|
| E001 | `UnexpectedToken` | Unexpected token encountered | Validate input for invalid characters, malformed commands, or unsupported syntax at the reported position. |
| E002 | `UnexpectedEOF` | Unexpected end of file | Confirm all environments, brace pairs, and command arguments are properly terminated. |
| E003 | `UnmatchedBrace` | Unmatched brace detected | Ensure every opening `{` has a corresponding closing `}` and braces are correctly nested. |
| E004 | `InvalidMathDelimiter` | Invalid math delimiter detected | Validate usage of math delimiters: `$`, `$$`, `\(`, `\)`, `\[`, and `\]`. |
| E005 | `UnterminatedArgument` | Command argument not terminated | Append missing closing brace `}` to the command argument. |
| E006 | `InvalidEscapeSequence` | Invalid escape sequence | Verify the command name following the backslash `\` is valid. |
| E007 | `InvalidUnicode` | Invalid UTF-8 sequence detected | Re-save the source file with UTF-8 encoding. |
| E008 | `IllegalParameterChar` | Illegal parameter character usage | Ensure `#` is used only in macro definitions and follows correct syntax. |
| E009 | `UnterminatedVerbatim` | Verbatim environment not terminated | Close the verbatim environment with appropriate termination markers. |
| E010 | `InvalidCharacter` | Invalid character encountered | Remove or replace unsupported character at the reported position. |

## Error Categories

### Syntax Errors (E001-E006)

These errors occur when the input violates lexical grammar rules.

**E001 - UnexpectedToken**  
The lexer encountered a sequence that does not form a valid token in the current state. Check for:
- Special characters in invalid contexts
- Malformed commands or arguments
- Syntax that does not conform to the language specification

**E002 - UnexpectedEOF**  
The source input terminated prematurely. Verify that:
- All environment blocks are closed
- Every opening brace has a matching closure
- Command argument lists are complete

**E003 - UnmatchedBrace**  
Brace matching failed during tokenization. Inspect:
- Brace pair count (each `{` requires a `}`)
- Brace nesting order
- Arguments enclosed within braces

**E004 - InvalidMathDelimiter**  
Math mode delimiters are incorrectly specified. Valid delimiters:
- Inline: `$...$` or `\(...\)`
- Display: `$$...$$` or `\[...\]`

**E005 - UnterminatedArgument**  
A command argument lacks its closing brace. Each `{` that starts an argument must have a matching `}`.

**E006 - InvalidEscapeSequence**  
The sequence following a backslash does not form a valid command. Ensure the command name consists of valid characters and exists in the context.

### Encoding and Character Errors (E007-E010)

These errors relate to character encoding and invalid character handling.

**E007 - InvalidUnicode**  
The source file contains bytes that do not form valid UTF-8 sequences. Re-encode the file using UTF-8.

**E008 - IllegalParameterChar**  
The `#` character is used in an invalid context. Proper usage:
- Macro parameter references: `#1`, `#2`, etc.
- Macro definitions only

**E009 - UnterminatedVerbatim**  
A verbatim environment was opened but not closed. Ensure the verbatim block terminates correctly.

**E010 - InvalidCharacter**  
The lexer encountered a character not permitted in the current context.
Remove or replace the offending character.

## Diagnostic Example

```bash
🔍 Found 1 issue(s):

LTX::E001

  × unexpected token `@`
   ╭─[main.tex:3:19]
 2 │ % E001: Unexpected Token
 3 │ \newcommand{\foo} @invalid
   ·                   ────┬───
   ·                       ╰── here
 4 │
   ╰────
  help: Check for invalid characters, malformed commands, or unsupported syntax near the highlighted position.
```
