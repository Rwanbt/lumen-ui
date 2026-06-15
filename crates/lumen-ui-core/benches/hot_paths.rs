//! Microbenchmarks for the per-frame hot paths: recipe resolution (called once
//! per widget per frame) and the WCAG contrast audit. See `docs/performance.md`
//! for the budgets these guard.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lumen_ui_core::{a11y, ButtonVariant, DarkTheme, TextRole, Theme, UiContext, WidgetState};

fn bench_button_recipe(c: &mut Criterion) {
    let theme = DarkTheme::new();
    let ctx = UiContext::default();
    c.bench_function("button_recipe", |b| {
        b.iter(|| {
            theme.button_recipe(
                black_box(ButtonVariant::Primary),
                black_box(WidgetState::Hovered),
                black_box(&ctx),
            )
        });
    });
}

fn bench_text_recipe(c: &mut Criterion) {
    let theme = DarkTheme::new();
    let ctx = UiContext::default();
    c.bench_function("text_recipe", |b| {
        b.iter(|| theme.text_recipe(black_box(TextRole::Body), black_box(&ctx)));
    });
}

fn bench_audit_colors(c: &mut Criterion) {
    let colors = DarkTheme::new().tokens().colors.clone();
    c.bench_function("audit_colors", |b| {
        b.iter(|| a11y::audit_colors(black_box(&colors)));
    });
}

criterion_group!(
    benches,
    bench_button_recipe,
    bench_text_recipe,
    bench_audit_colors
);
criterion_main!(benches);
