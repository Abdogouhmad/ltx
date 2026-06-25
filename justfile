alias t := test
alias b := build
alias d := doc
alias f := fmt
alias fc := fmtck
alias c := check
alias q := cw
alias dia := diagnostics

default:
    just -l

# build with release command
build:
    cargo rel

# command that test the whole workspace project
test:
    cargo test -p ltx_diagnostics --test severity_test

# format the whole project
fmt:
    cargo fmt

# check the format
fmtck:
    cargo fmtck

# check workspace for any error
check:
    cargo ch

# compile and open docs
doc:
    cargo docs

# clippy check
cw:
    cargo qa

# run diagnostics
diagnostics:
    cargo test -p ltx_diagnostics -- --nocapture

# run diagnostics examples
diagnostics-examples:
    echo "errors"
    cargo run -p ltx_diagnostics --example lexer_errors
    echo ""
    echo ""
    cargo run -p ltx_diagnostics --example parser_errors
    echo ""
    echo ""
    cargo run -p ltx_diagnostics --example json_render | jq
