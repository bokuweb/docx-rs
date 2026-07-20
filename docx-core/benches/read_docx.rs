use criterion::{criterion_group, criterion_main, Criterion};
use docx_rs::{read_docx, read_docx_with_options, ReadDocxOptions};
use std::hint::black_box;

fn bench_read_docx(c: &mut Criterion) {
    static DOCX_BYTES: &[u8] = include_bytes!("../../fixtures/hello_world/hello_world.docx");
    static PNG_DOCX_BYTES: &[u8] = include_bytes!("../../fixtures/image/image.docx");
    static JPEG_DOCX_BYTES: &[u8] = include_bytes!("../../fixtures/image_output/image.docx");
    c.bench_function("read_docx_hello", |b| {
        b.iter(|| {
            let buffer = black_box(DOCX_BYTES);
            black_box(read_docx(buffer).expect("failed to read docx"));
        });
    });
    c.bench_function("read_docx_png", |b| {
        b.iter(|| {
            let buffer = black_box(PNG_DOCX_BYTES);
            black_box(read_docx(buffer).expect("failed to read PNG docx"));
        });
    });
    c.bench_function("read_docx_jpeg", |b| {
        b.iter(|| {
            let buffer = black_box(JPEG_DOCX_BYTES);
            black_box(read_docx(buffer).expect("failed to read JPEG docx"));
        });
    });
    c.bench_function("read_docx_jpeg_without_previews", |b| {
        b.iter(|| {
            let buffer = black_box(JPEG_DOCX_BYTES);
            black_box(
                read_docx_with_options(
                    buffer,
                    ReadDocxOptions::default().with_image_previews(false),
                )
                .expect("failed to read JPEG docx"),
            );
        });
    });
}

criterion_group!(docx_benches, bench_read_docx);
criterion_main!(docx_benches);
