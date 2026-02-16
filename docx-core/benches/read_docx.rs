use criterion::{black_box, criterion_group, criterion_main, Criterion};
use docx_rs::read_docx;

fn bench_read_docx(c: &mut Criterion) {
    static DOCX_BYTES: &[u8] = include_bytes!("../../fixtures/hello_world/hello_world.docx");
    c.bench_function("read_docx_hello", |b| {
        b.iter(|| {
            let buffer = black_box(DOCX_BYTES);
            black_box(read_docx(buffer).expect("failed to read docx"));
        });
    });
}

criterion_group!(docx_benches, bench_read_docx);
criterion_main!(docx_benches);
