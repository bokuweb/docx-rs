use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct TableOfContentsItem {
    pub instr: InstrToC,
    pub text: String,
    pub toc_key: String,
    pub level: usize,
}

impl TableOfContentsItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn level(mut self, level: usize) -> Self {
        self.level = level;
        self
    }

    pub fn toc_key(mut self, key: impl Into<String>) -> Self {
        self.toc_key = key.into();
        self
    }
}

impl BuildXML for Vec<TableOfContentsItem> {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new()
            .open_structured_tag()
            .open_structured_tag_property()
            .close()
            .open_structured_tag_content();

        for (i, t) in self.iter().enumerate() {
            let mut p = Paragraph::new().style(&format!("ToC{}", t.level));
            if i == 0 {
                p = p.unshift_run(
                    Run::new()
                        .add_field_char(FieldCharType::Begin, false)
                        .add_instr_text(InstrText::TOC(t.instr.clone()))
                        .add_field_char(FieldCharType::Separate, false),
                );
                p = p
                    .add_tab(
                        Tab::new()
                            .val(TabValueType::Right)
                            .leader(TabLeaderType::Dot)
                            // TODO: for now set 20000
                            .pos(20000),
                    )
                    .add_run(Run::new().add_text(&t.text).add_tab())
                    .add_run(
                        Run::new()
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::PAGEREF(
                                InstrPAGEREF::new(&t.toc_key).hyperlink(),
                            ))
                            .add_field_char(FieldCharType::Separate, false)
                            .add_text("1") // TODO: Add 1 for now
                            .add_field_char(FieldCharType::End, false),
                    );
                b = b.add_child(&p);
            } else {
                let mut p = Paragraph::new().style(&format!("ToC{}", t.level));
                p = p
                    .add_tab(
                        Tab::new()
                            .val(TabValueType::Right)
                            .leader(TabLeaderType::Dot)
                            // TODO: for now set 20000
                            .pos(20000),
                    )
                    .add_run(Run::new().add_text(&t.text).add_tab())
                    .add_run(
                        Run::new()
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::PAGEREF(
                                InstrPAGEREF::new(&t.toc_key).hyperlink(),
                            ))
                            .add_field_char(FieldCharType::Separate, false)
                            .add_text("1") // TODO: Add 1 for now
                            .add_field_char(FieldCharType::End, false),
                    );
                b = b.add_child(&p);
            }

            if i == self.len() - 1 {
                let mut p = Paragraph::new().style(&format!("ToC{}", t.level));
                p = p.add_run(Run::new().add_field_char(FieldCharType::End, false));
                b = b.add_child(&p);
            }
        }
        b.close().close().build()
    }
}
