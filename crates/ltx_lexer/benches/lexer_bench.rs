#![allow(missing_docs)]

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, TokenStream};

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

const LARGE_SOURCE: &str = r"\documentclass{article}
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{amsmath,amssymb}
\usepackage{graphicx}
\usepackage{hyperref}
\title{A Large Document}
\author{Author Name}
\begin{document}
\maketitle

\section{Introduction}
This document contains a substantial amount of text to stress-test
the lexer. The quick brown fox jumps over the lazy dog. Pack my box
with five dozen liquor jugs. How vexingly quick daft zebras jump!

\section{Math}
Inline math: $E = mc^2$ and $a^2 + b^2 = c^2$.
Display math:
$$\int_{0}^{1} x^2 \, dx = \frac{1}{3}$$
$$\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6}$$

\section{Lists}
\begin{itemize}
  \item First item with \textbf{bold} and \textit{italic} text.
  \item Second item with \texttt{monospace} and \emph{emphasis}.
  \item Third item with a \href{https://example.com}{link}.
\end{itemize}

\section{Tables}
\begin{tabular}{|l|c|r|}
  \hline
  Left & Center & Right \\
  \hline
  A    & 1      & 100   \\
  B    & 2      & 200   \\
  C    & 3      & 300   \\
  \hline
\end{tabular}

\section{Equations}
\begin{equation}
  \label{eq:euler}
  e^{i\pi} + 1 = 0
\end{equation}

As shown in Equation~\ref{eq:Euler}, Euler's identity is beautiful.

\end{document}";

fn bench_lexer_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer_tokenize");
    for (name, source) in [
        ("small", SMALL_SOURCE),
        ("medium", MEDIUM_SOURCE),
        ("large", LARGE_SOURCE),
    ] {
        group.bench_with_input(BenchmarkId::new("tokens", name), source, |b, src| {
            b.iter(|| {
                let mut source_map = LtxSourceMap::new();
                let file_id = source_map.add_inline("bench.tex", src);
                let lexer = LtxLexer::new(src, file_id, source_map);
                lexer.count()
            });
        });
    }
    group.finish();
}

fn bench_stream_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokenstream_creation");
    for (name, source) in [
        ("small", SMALL_SOURCE),
        ("medium", MEDIUM_SOURCE),
        ("large", LARGE_SOURCE),
    ] {
        group.bench_with_input(BenchmarkId::new("new", name), source, |b, src| {
            b.iter(|| {
                let mut source_map = LtxSourceMap::new();
                let file_id = source_map.add_inline("bench.tex", src);
                let lexer = LtxLexer::new(src, file_id, source_map);
                TokenStream::new(lexer)
            });
        });
    }
    group.finish();
}

fn bench_lexer_iter_vs_next(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer_dispatch");
    let source = LARGE_SOURCE;

    group.bench_function("iterator_count", |b| {
        b.iter(|| {
            let mut source_map = LtxSourceMap::new();
            let file_id = source_map.add_inline("bench.tex", source);
            let lexer = LtxLexer::new(source, file_id, source_map);
            lexer.count()
        });
    });

    group.bench_function("next_token_loop", |b| {
        b.iter(|| {
            let mut source_map = LtxSourceMap::new();
            let file_id = source_map.add_inline("bench.tex", source);
            let mut lexer = LtxLexer::new(source, file_id, source_map);
            let mut count = 0usize;
            while lexer.next_token().is_some() {
                count += 1;
            }
            count
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_lexer_throughput,
    bench_stream_creation,
    bench_lexer_iter_vs_next,
);
criterion_main!(benches);
