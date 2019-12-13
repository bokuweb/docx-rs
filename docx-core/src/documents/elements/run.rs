use super::{Break, RunProperty, Tab, Text};
use crate::documents::BuildXML;
use crate::types::BreakType;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Run {
    pub run_property: RunProperty,
    pub children: Vec<RunChild>,
}

impl Default for Run {
    fn default() -> Self {
        let run_property = RunProperty::new();
        Self {
            run_property,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum RunChild {
    Text(Text),
    Tab(Tab),
    Break(Break),
}

impl Run {
    pub fn new() -> Run {
        Run {
            ..Default::default()
        }
    }

    pub fn add_text(mut self, text: &str) -> Run {
        self.children.push(RunChild::Text(Text::new(text)));
        self
    }

    pub fn add_delete_text(mut self, text: &str) -> Run {
        self.children.push(RunChild::Text(Text::new(text)));
        self
    }

    pub fn add_tab(mut self) -> Run {
        self.children.push(RunChild::Tab(Tab::new()));
        self
    }

    pub fn add_break(mut self, break_type: BreakType) -> Run {
        self.children.push(RunChild::Break(Break::new(break_type)));
        self
    }

    pub fn size(mut self, size: usize) -> Run {
        self.run_property = self.run_property.size(size);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Run {
        self.run_property = self.run_property.color(color);
        self
    }

    pub fn highlight(mut self, color: impl Into<String>) -> Run {
        self.run_property = self.run_property.highlight(color);
        self
    }

    pub fn bold(mut self) -> Run {
        self.run_property = self.run_property.bold();
        self
    }

    pub fn italic(mut self) -> Run {
        self.run_property = self.run_property.italic();
        self
    }

    pub fn underline(mut self, line_type: impl Into<String>) -> Run {
        self.run_property = self.run_property.underline(line_type);
        self
    }
}

impl BuildXML for Run {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_run().add_child(&self.run_property);
        for c in &self.children {
            match c {
                RunChild::Text(t) => b = b.add_child(t),
                RunChild::Tab(t) => b = b.add_child(t),
                RunChild::Break(t) => b = b.add_child(t),
            }
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_build() {
        let b = Run::new().add_text("Hello").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r>"#
        );
    }

    #[test]
    fn test_underline() {
        let b = Run::new().add_text("Hello").underline("single").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r><w:rPr><w:u w:val="single" /></w:rPr><w:t xml:space="preserve">Hello</w:t></w:r>"#
        );
    }
}
