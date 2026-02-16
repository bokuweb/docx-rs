use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use docx_rs::*;
use std::io::Cursor;

fn create_template(paragraph_count: usize, runs_per_paragraph: usize) -> Docx {
    let mut docx = Docx::new();
    for p in 0..paragraph_count {
        let mut paragraph = Paragraph::new();
        for r in 0..runs_per_paragraph {
            paragraph = paragraph.add_run(
                Run::new().add_text(format!("paragraph-{p} run-{r} lorem ipsum dolor sit amet")),
            );
        }
        docx = docx.add_paragraph(paragraph);
    }
    docx
}

fn bench_write_docx(c: &mut Criterion) {
    let template = create_template(200, 5);
    c.bench_function("write_docx_build_pack", |b| {
        b.iter_batched(
            || template.clone(),
            |docx| {
                let mut cursor = Cursor::new(Vec::with_capacity(64 * 1024));
                docx.build()
                    .pack(&mut cursor)
                    .expect("failed to write docx");
                black_box(cursor.into_inner());
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(docx_write_benches, bench_write_docx);
criterion_main!(docx_write_benches);
