//! Tests for the `CatCodeState` struct.

use ltx_lexer::{LtxCatCode, LtxCatCodeState};
use pretty_assertions::assert_eq;

#[test]
fn test_default_catcode_state() {
    let state = LtxCatCodeState::default();

    assert_eq!(state.get('\\'), LtxCatCode::Escape);
    assert_eq!(state.get('{'), LtxCatCode::BeginGroup);
    assert_eq!(state.get('}'), LtxCatCode::EndGroup);
    assert_eq!(state.get('$'), LtxCatCode::MathShift);
    assert_eq!(state.get('&'), LtxCatCode::AlignmentTab);
    assert_eq!(state.get('\n'), LtxCatCode::EndOfLine);
    assert_eq!(state.get('#'), LtxCatCode::Parameter);
    assert_eq!(state.get('^'), LtxCatCode::Superscript);
    assert_eq!(state.get('_'), LtxCatCode::Subscript);
    assert_eq!(state.get(' '), LtxCatCode::Space);
    assert_eq!(state.get('%'), LtxCatCode::Comment);
    assert_eq!(state.get('~'), LtxCatCode::Active);

    assert_eq!(state.get('A'), LtxCatCode::Letter);
    assert_eq!(state.get('z'), LtxCatCode::Letter);
    assert_eq!(state.get('@'), LtxCatCode::Other);
    assert_eq!(state.get('!'), LtxCatCode::Other);
}

#[test]
fn test_get_catcodestate() {
    let state = LtxCatCodeState::default();
    let catcode = state.get('\\');
    assert_eq!(catcode, LtxCatCode::Escape);
}

#[test]
fn test_set_catcodestate() {
    let mut state = LtxCatCodeState::default();
    state.set('\\', LtxCatCode::Other);
    assert_eq!(state.get('\\'), LtxCatCode::Other);
}

#[test]
fn test_catcode_from_u8() {
    let catcodevarient = [
        LtxCatCode::Active,
        LtxCatCode::Comment,
        LtxCatCode::EndOfLine,
        LtxCatCode::Escape,
        LtxCatCode::Letter,
        LtxCatCode::MathShift,
        LtxCatCode::Other,
        LtxCatCode::Space,
        LtxCatCode::Subscript,
        LtxCatCode::Superscript,
        LtxCatCode::Parameter,
        LtxCatCode::BeginGroup,
        LtxCatCode::EndGroup,
        LtxCatCode::Invalid,
        LtxCatCode::Ignored,
    ];
    for catcode in catcodevarient {
        let state = LtxCatCode::from_u8(catcode as u8);
        assert_eq!(state, Some(catcode));
    }
}
