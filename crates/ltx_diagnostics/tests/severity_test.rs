//! This a test to match whether ltx SEVERITy matches the mattie severity
use ltx_diagnostics::LtxSeverity;
use miette::Severity;
use pretty_assertions::assert_eq;


#[test]
fn match_error() {
    let ltx_sev = LtxSeverity::Error;
    let miette_sev = ltx_sev.to_miette();
    assert_eq!(miette_sev, Severity::Error);
}

#[test]
fn match_warning() {
    let ltx_sev = LtxSeverity::Warning;
    let miette_sev = ltx_sev.to_miette();
    assert_eq!(miette_sev, Severity::Warning);
}

#[test]
fn match_hint() {
    let ltx_sev = LtxSeverity::Hint;
    let miette_sev = ltx_sev.to_miette();
    assert_eq!(miette_sev, Severity::Advice);
}


#[test]
fn matched_all() {
    // ltx severity
    let ltx_sev_error = LtxSeverity::Error;
    let ltx_sev_warning = LtxSeverity::Warning;
    let ltx_sev_hint = LtxSeverity::Hint;
    // miette severity
    let miette_sev_err = Severity::Error;
    let miette_sev_warning = Severity::Warning;
    let miette_sev_hint = Severity::Advice;
    assert_eq!(miette_sev_err as u8 , ltx_sev_error as u8);
    assert_eq!(miette_sev_warning as u8 , ltx_sev_warning as u8);
    assert_eq!(miette_sev_hint as u8 , ltx_sev_hint as u8);
}
