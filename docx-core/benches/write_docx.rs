use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use docx_rs::*;
use std::hint::black_box;
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
    c.bench_function("write_docx_build", |b| {
        b.iter_batched(
            || template.clone(),
            |docx| black_box(docx.build()),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("write_docx_pack", |b| {
        b.iter_batched(
            || template.clone().build(),
            |xml| {
                let mut cursor = Cursor::new(Vec::with_capacity(64 * 1024));
                xml.pack(&mut cursor).expect("failed to write docx");
                black_box(cursor.into_inner());
            },
            BatchSize::SmallInput,
        );
    });

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

    let large_template = create_template(2_000, 5);
    c.bench_function("write_docx_large_build_pack", |b| {
        b.iter_batched(
            || large_template.clone(),
            |docx| {
                let mut cursor = Cursor::new(Vec::with_capacity(512 * 1024));
                docx.build()
                    .pack(&mut cursor)
                    .expect("failed to write large docx");
                black_box(cursor.into_inner());
            },
            BatchSize::LargeInput,
        );
    });

    let image = vec![42; 64 * 1024];
    let mut repeated_image_template = Docx::new();
    for _ in 0..100 {
        repeated_image_template = repeated_image_template.add_paragraph(
            Paragraph::new().add_run(Run::new().add_image(Pic::new_with_dimensions(
                image.clone(),
                1,
                1,
            ))),
        );
    }
    c.bench_function("write_docx_repeated_images", |b| {
        b.iter_batched(
            || repeated_image_template.clone(),
            |docx| {
                let mut cursor = Cursor::new(Vec::with_capacity(128 * 1024));
                docx.build()
                    .pack(&mut cursor)
                    .expect("failed to write image docx");
                black_box(cursor.into_inner());
            },
            BatchSize::LargeInput,
        );
    });
}

criterion_group!(docx_write_benches, bench_write_docx);
criterion_main!(docx_write_benches);
