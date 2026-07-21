//! benches for catcode
//! Benchmarks for `CatCodeState` performance

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use ltx_lexer::{LtxCatCode, LtxCatCodeState};

fn bench_catcode_get(c: &mut Criterion) {
    let state = LtxCatCodeState::default_tex();
    let chars: Vec<char> = "Hello \\LaTeX $E=mc^2$ {group} %comment".chars().collect();

    c.bench_function("catcode_get", |b| {
        b.iter(|| {
            let mut sum = 0u8;
            for &ch in &chars {
                let cat = state.get(ch);
                sum += cat as u8;
            }
            black_box(sum);
        });
    });
}

fn bench_catcode_set(c: &mut Criterion) {
    let chars: Vec<char> = "Hello \\LaTeX $E=mc^2$ {group} %comment".chars().collect();

    c.bench_function("catcode_set", |b| {
        b.iter(|| {
            let mut state = LtxCatCodeState::default_tex();
            for &ch in &chars {
                state.set(ch, LtxCatCode::Letter);
                black_box(&state);
            }
        });
    });
}

fn bench_catcode_is_letter(c: &mut Criterion) {
    let state = LtxCatCodeState::default_tex();
    let chars: Vec<char> = "Hello \\LaTeX $E=mc^2$ {group} %comment".chars().collect();

    c.bench_function("catcode_is_letter", |b| {
        b.iter(|| {
            let mut count = 0;
            for &ch in &chars {
                if state.is_letter(ch) {
                    count += 1;
                }
            }
            black_box(count);
        });
    });
}

fn bench_catcode_reset(c: &mut Criterion) {
    c.bench_function("catcode_reset_to_other", |b| {
        b.iter(|| {
            let mut state = LtxCatCodeState::default_tex();
            state.reset_to_other();
            black_box(state);
        });
    });
}

fn bench_catcode_default_tex(c: &mut Criterion) {
    c.bench_function("catcode_default_tex", |b| {
        b.iter(|| {
            let state = LtxCatCodeState::default_tex();
            black_box(state);
        });
    });
}

fn bench_catcode_get_vs_hashmap(c: &mut Criterion) {
    use std::collections::HashMap;

    let chars: Vec<char> = "Hello \\LaTeX $E=mc^2$ {group} %comment".chars().collect();

    // Array-based (your implementation)
    let array_state = LtxCatCodeState::default_tex();

    // HashMap-based (for comparison)
    let mut hashmap_state = HashMap::new();
    // Initialize with same values
    let ranging = 32..=126;
    for chara in ranging {
        if let Ok(byte) = u8::try_from(chara) {
            hashmap_state.insert(byte as char, LtxCatCode::Other);
        }
    }
    hashmap_state.insert('\\', LtxCatCode::Escape);
    hashmap_state.insert('{', LtxCatCode::GroupStart);
    hashmap_state.insert('}', LtxCatCode::GroupEnd);
    hashmap_state.insert('$', LtxCatCode::MathShift);
    hashmap_state.insert('&', LtxCatCode::AlignmentTab);
    hashmap_state.insert('\n', LtxCatCode::EndOfLine);
    hashmap_state.insert('#', LtxCatCode::Parameter);
    hashmap_state.insert('^', LtxCatCode::Superscript);
    hashmap_state.insert('_', LtxCatCode::Subscript);
    hashmap_state.insert(' ', LtxCatCode::WhiteSpace);
    hashmap_state.insert('%', LtxCatCode::Comment);
    hashmap_state.insert('~', LtxCatCode::Active);
    for c in ('A'..='Z').chain('a'..='z') {
        hashmap_state.insert(c, LtxCatCode::Letter);
    }

    // Benchmark array version
    c.bench_function("catcode_get_array", |b| {
        b.iter(|| {
            let mut sum = 0u8;
            for &ch in &chars {
                let cat = array_state.get(ch);
                sum += cat as u8;
            }
            black_box(sum);
        });
    });

    // Benchmark HashMap version
    c.bench_function("catcode_get_hashmap", |b| {
        b.iter(|| {
            let mut sum = 0u8;
            for &ch in &chars {
                let cat = *hashmap_state.get(&ch).unwrap_or(&LtxCatCode::Other);
                sum += cat as u8;
            }
            black_box(sum);
        });
    });
}

fn bench_catcode_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_operations");
    let chars: Vec<char> = (0..1000)
        .map(|i| u8::try_from(i % 95 + 32).unwrap_or(32) as char)
        .collect();
    group.bench_function("get_1000_chars", |b| {
        let state = LtxCatCodeState::default_tex();
        b.iter(|| {
            let mut sum = 0u8;
            for &ch in &chars {
                let cat = state.get(ch);
                sum += cat as u8;
            }
            black_box(sum);
        });
    });

    // Set 1000 characters
    group.bench_function("set_1000_chars", |b| {
        b.iter(|| {
            let mut state = LtxCatCodeState::default_tex();
            for (i, &ch) in chars.iter().enumerate() {
                state.set(
                    ch,
                    if i % 2 == 0 {
                        LtxCatCode::Letter
                    } else {
                        LtxCatCode::Other
                    },
                );
            }
            black_box(state);
        });
    });

    group.finish();
}

fn bench_catcode_mixed_workload(c: &mut Criterion) {
    let test_cases = vec![
        ("plain_text", "Hello World! This is plain text."),
        (
            "with_commands",
            r"\section{Introduction} \textbf{bold} \textit{italic}",
        ),
        ("with_math", r"$E=mc^2$ and $x^2 + y^2 = z^2$"),
        (
            "mixed",
            r"\documentclass{article} \begin{document} Hello $E=mc^2$ % comment \end{document}",
        ),
    ];

    for (name, content) in test_cases {
        let chars: Vec<char> = content.chars().collect();
        let state = LtxCatCodeState::default_tex();

        c.bench_with_input(
            BenchmarkId::new("mixed_workload", name),
            &chars,
            |b, chars| {
                b.iter(|| {
                    let mut sum = 0u8;
                    let mut is_letter_count = 0;
                    for &ch in chars {
                        let cat = state.get(ch);
                        sum += cat as u8;
                        if state.is_letter(ch) {
                            is_letter_count += 1;
                        }
                    }
                    black_box((sum, is_letter_count));
                });
            },
        );
    }
}

fn bench_catcode_from_u8(c: &mut Criterion) {
    let values: Vec<u8> = (0..16).collect();

    c.bench_function("catcode_from_u8", |b| {
        b.iter(|| {
            let mut cats = Vec::with_capacity(16);
            for &v in &values {
                if let Some(cat) = LtxCatCode::from_u8(v) {
                    cats.push(cat);
                }
            }
            black_box(cats);
        });
    });
}

fn bench_catcode_as_u8(c: &mut Criterion) {
    let cats: Vec<LtxCatCode> = vec![
        LtxCatCode::Escape,
        LtxCatCode::GroupStart,
        LtxCatCode::GroupEnd,
        LtxCatCode::MathShift,
        LtxCatCode::Letter,
        LtxCatCode::Other,
        LtxCatCode::Comment,
        LtxCatCode::WhiteSpace,
    ];

    c.bench_function("catcode_as_u8", |b| {
        b.iter(|| {
            let mut bytes = Vec::with_capacity(8);
            for &cat in &cats {
                bytes.push(cat.as_u8());
            }
            black_box(bytes);
        });
    });
}

criterion_group!(
    benches,
    bench_catcode_get,
    bench_catcode_set,
    bench_catcode_is_letter,
    bench_catcode_reset,
    bench_catcode_default_tex,
    bench_catcode_get_vs_hashmap,
    bench_catcode_bulk_operations,
    bench_catcode_mixed_workload,
    bench_catcode_from_u8,
    bench_catcode_as_u8,
);

criterion_main!(benches);
