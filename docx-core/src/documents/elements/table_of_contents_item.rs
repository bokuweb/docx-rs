use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct TableOfContentsItem {
    pub instr: InstrToC,
    pub text: String,
}

impl TableOfContentsItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
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
            dbg!(i);
            if i == 0 {
                let mut p = Paragraph::new();
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
                            .pos(10000),
                    )
                    .add_run(Run::new().add_text(&t.text).add_tab())
                    // TODO: placeholder
                    .add_run(Run::new().add_text("1"));

                if i == self.len() - 1 {
                    p = p.add_run(Run::new().add_field_char(FieldCharType::End, false));
                }
                dbg!(&p);
                b = b.add_child(&p);
            } else if i == self.len() - 1 {
                let mut p = Paragraph::new();
                p = p.add_run(Run::new().add_field_char(FieldCharType::End, false));
                b = b.add_child(&p);
            } else {
                let mut p = Paragraph::new();
                p = p
                    .add_tab(
                        Tab::new()
                            .val(TabValueType::Right)
                            .leader(TabLeaderType::Dot)
                            .pos(10000),
                    )
                    .add_run(Run::new().add_text(&t.text).add_tab())
                    // TODO: placeholder
                    .add_run(Run::new().add_text("1"));
                b = b.add_child(&p);
            }
        }
        let res = b.close().close().build();
        res
    }
}
