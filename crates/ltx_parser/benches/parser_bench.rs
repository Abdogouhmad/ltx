#![allow(missing_docs)]

use criterion::{Criterion, criterion_group, criterion_main};
use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, TokenStream};
use ltx_parser::{LtxParser, parse_document};

const SMALL_SOURCE: &str = r"\documentclass{article}
\begin{document}
Hello world!
\end{document}";

const MEDIUM_SOURCE: &str = r"\documentclass{article}
\usepackage{amsmath}
\begin{document}
Hello world! This is a test of the lexer throughput.
\[ \int_{0}^{1} x^2 \, dx = \frac{1}{3} \]
\end{document}";

const FULL_DOCUMENT: &str = r"\documentclass{article}
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{amsmath,amssymb}
\usepackage{graphicx}
\usepackage{hyperref}
\title{Sample Document}
\author{Author Name}
\begin{document}
\maketitle

\section{Introduction}
Hello world! This is a sample document with \textbf{bold} and \textit{italic} text.

\section{Math}
Inline math: $E = mc^2$ and display math:
$$\int_{0}^{1} x^2 \, dx = \frac{1}{3}$$

\begin{itemize}
\item First item
\item Second item
\end{itemize}

\section{Tables}
\begin{tabular}{|l|c|r|}
\hline
Left & Center & Right \\
\hline
A & 1 & 100 \\
B & 2 & 200 \\
\hline
\end{tabular}

\end{document}";

fn setup_parser(source: &str) -> LtxParser<'_> {
    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline("bench.tex", source);
    let stream = TokenStream::new(LtxLexer::new(source, file_id, source_map));
    LtxParser::new(stream)
}

fn bench_parser_full_document(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_parse");
    for (name, source) in [
        ("small", SMALL_SOURCE),
        ("medium", MEDIUM_SOURCE),
        ("full", FULL_DOCUMENT),
    ] {
        group.bench_function(criterion::BenchmarkId::new("document", name), |b| {
            b.iter(|| {
                let mut parser = setup_parser(source);
                let doc = parse_document(&mut parser);
                std::hint::black_box(&doc);
            });
        });
    }
    group.finish();
}

fn bench_parser_stream_vs_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_pipeline");
    let source = FULL_DOCUMENT;

    group.bench_function("stream_creation_only", |b| {
        b.iter(|| {
            let mut source_map = LtxSourceMap::new();
            let file_id = source_map.add_inline("bench.tex", source);
            let stream = TokenStream::new(LtxLexer::new(source, file_id, source_map));
            std::hint::black_box(&stream);
        });
    });

    group.bench_function("full_parse_pipeline", |b| {
        b.iter(|| {
            let mut parser = setup_parser(source);
            let doc = parse_document(&mut parser);
            std::hint::black_box(&doc);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_parser_full_document,
    bench_parser_stream_vs_parse,
);
criterion_main!(benches);
