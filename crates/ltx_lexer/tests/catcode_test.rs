//! Tests for the `CatCodeState` struct.

use ltx_lexer::{CatCode, CatCodeState};
use pretty_assertions::assert_eq;

#[test]
fn test_default_catcode_state() {
    let state = CatCodeState::default();

    assert_eq!(state.get('\\'), CatCode::Escape);
    assert_eq!(state.get('{'), CatCode::BeginGroup);
    assert_eq!(state.get('}'), CatCode::EndGroup);
    assert_eq!(state.get('$'), CatCode::MathShift);
    assert_eq!(state.get('&'), CatCode::AlignmentTab);
    assert_eq!(state.get('\n'), CatCode::EndOfLine);
    assert_eq!(state.get('#'), CatCode::Parameter);
    assert_eq!(state.get('^'), CatCode::Superscript);
    assert_eq!(state.get('_'), CatCode::Subscript);
    assert_eq!(state.get(' '), CatCode::Space);
    assert_eq!(state.get('%'), CatCode::Comment);
    assert_eq!(state.get('~'), CatCode::Active);

    assert_eq!(state.get('A'), CatCode::Letter);
    assert_eq!(state.get('z'), CatCode::Letter);
    assert_eq!(state.get('@'), CatCode::Other);
    assert_eq!(state.get('!'), CatCode::Other);
}

#[test]
fn test_get_catcodestate() {
    let state = CatCodeState::default();
    let catcode = state.get('\\');
    assert_eq!(catcode, CatCode::Escape);
}

#[test]
fn test_set_catcodestate() {
    let mut state = CatCodeState::default();
    state.set('\\', CatCode::Other);
    assert_eq!(state.get('\\'), CatCode::Other);
}

#[test]
fn test_catcode_from_u8() {
    let catcodevarient = [
        CatCode::Active,
        CatCode::Comment,
        CatCode::EndOfLine,
        CatCode::Escape,
        CatCode::Letter,
        CatCode::MathShift,
        CatCode::Other,
        CatCode::Space,
        CatCode::Subscript,
        CatCode::Superscript,
        CatCode::Parameter,
        CatCode::BeginGroup,
        CatCode::EndGroup,
        CatCode::Invalid,
        CatCode::Ignored,
    ];
    for catcode in catcodevarient {
        let state = CatCode::from_u8(catcode as u8);
        assert_eq!(state, Some(catcode));
    }
}
