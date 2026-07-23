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
    c.bench_function("write_docx_construct", |b| {
        b.iter(|| black_box(create_template(200, 5)));
    });

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

    c.bench_function("write_docx_direct_pack", |b| {
        b.iter_batched(
            || template.clone(),
            |docx| {
                let mut cursor = Cursor::new(Vec::with_capacity(64 * 1024));
                docx.pack(&mut cursor).expect("failed to write docx");
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

    let mut many_section_parts_template = Docx::new();
    for number in 0..250 {
        let header = Header::new().add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(format!("header-{number}"))),
        );
        let footer = Footer::new().add_paragraph(
            Paragraph::new().add_run(Run::new().add_text(format!("footer-{number}"))),
        );
        many_section_parts_template =
            many_section_parts_template.add_section(Section::new().header(header).footer(footer));
    }
    c.bench_function("write_docx_many_section_parts", |b| {
        b.iter_batched(
            || many_section_parts_template.clone(),
            |docx| black_box(docx.build()),
            BatchSize::LargeInput,
        );
    });

    let mut colliding_paragraph_ids_template = Docx::new();
    for id in 1_u32..=1_000 {
        colliding_paragraph_ids_template = colliding_paragraph_ids_template
            .add_paragraph(Paragraph::new().id(format!("{id:08x}")));
    }
    for _ in 0..1_000 {
        colliding_paragraph_ids_template =
            colliding_paragraph_ids_template.add_paragraph(Paragraph::new().id("duplicate"));
    }
    c.bench_function("write_docx_colliding_paragraph_ids", |b| {
        b.iter_batched(
            || colliding_paragraph_ids_template.clone(),
            |docx| black_box(docx.build()),
            BatchSize::LargeInput,
        );
    });

    let mut populated_static_toc = TableOfContents::new();
    for index in 0..20 {
        populated_static_toc = populated_static_toc.add_before_paragraph(
            Paragraph::new().add_run(Run::new().add_text(format!("TOC context {index}"))),
        );
    }
    let mut many_static_tocs_template = Docx::new();
    for _ in 0..100 {
        many_static_tocs_template =
            many_static_tocs_template.add_table_of_contents(populated_static_toc.clone());
    }
    c.bench_function("write_docx_many_static_tocs", |b| {
        b.iter_batched(
            || many_static_tocs_template.clone(),
            |docx| black_box(docx.build()),
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

    let small_image = vec![42; 128];
    let mut many_small_images_template = Docx::new();
    for _ in 0..1_000 {
        many_small_images_template =
            many_small_images_template.add_paragraph(Paragraph::new().add_run(
                Run::new().add_image(Pic::new_with_dimensions(small_image.clone(), 1, 1)),
            ));
    }
    c.bench_function("write_docx_many_small_repeated_images", |b| {
        b.iter_batched(
            || many_small_images_template.clone(),
            |docx| {
                let mut cursor = Cursor::new(Vec::with_capacity(256 * 1024));
                docx.build()
                    .pack(&mut cursor)
                    .expect("failed to write small-image docx");
                black_box(cursor.into_inner());
            },
            BatchSize::LargeInput,
        );
    });

    let mut many_unique_images_template = Docx::new();
    for number in 0_u64..1_000 {
        let mut image = vec![42; 128];
        image[..8].copy_from_slice(&number.to_le_bytes());
        many_unique_images_template = many_unique_images_template.add_paragraph(
            Paragraph::new().add_run(Run::new().add_image(Pic::new_with_dimensions(image, 1, 1))),
        );
    }
    c.bench_function("write_docx_many_unique_images", |b| {
        b.iter_batched(
            || many_unique_images_template.clone(),
            |docx| {
                let mut cursor = Cursor::new(Vec::with_capacity(512 * 1024));
                docx.build()
                    .pack(&mut cursor)
                    .expect("failed to write unique-image docx");
                black_box(cursor.into_inner());
            },
            BatchSize::LargeInput,
        );
    });

    let mut colliding_image_ids_template = Docx::new();
    for number in 0_u64..1_000 {
        let mut image = vec![42; 128];
        image[..8].copy_from_slice(&number.to_le_bytes());
        colliding_image_ids_template = colliding_image_ids_template.add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_image(Pic::new_with_dimensions(image, 1, 1).id("shared"))),
        );
    }
    c.bench_function("write_docx_colliding_image_ids", |b| {
        b.iter_batched(
            || colliding_image_ids_template.clone(),
            |docx| {
                let mut cursor = Cursor::new(Vec::with_capacity(512 * 1024));
                docx.build()
                    .pack(&mut cursor)
                    .expect("failed to write colliding-image-ID docx");
                black_box(cursor.into_inner());
            },
            BatchSize::LargeInput,
        );
    });

    let reused_picture = Pic::new_with_dimensions(vec![42; 64 * 1024], 1, 1);
    let mut reused_picture_template = Docx::new();
    for _ in 0..100 {
        reused_picture_template = reused_picture_template
            .add_paragraph(Paragraph::new().add_run(Run::new().add_image(reused_picture.clone())));
    }
    c.bench_function("write_docx_reused_picture", |b| {
        b.iter_batched(
            || reused_picture_template.clone(),
            |docx| {
                let mut cursor = Cursor::new(Vec::with_capacity(128 * 1024));
                docx.build()
                    .pack(&mut cursor)
                    .expect("failed to write reused-picture docx");
                black_box(cursor.into_inner());
            },
            BatchSize::LargeInput,
        );
    });
}

criterion_group!(docx_write_benches, bench_write_docx);
criterion_main!(docx_write_benches);
