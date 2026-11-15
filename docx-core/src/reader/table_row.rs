use std::io::Read;
use std::str::FromStr;

use crate::HeightRule;

use super::attributes::*;
use super::*;

impl ElementReader for TableRow {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut cells = vec![];
        let mut grid_after = None;
        let mut width_after = None;
        let mut grid_before = None;
        let mut width_before = None;
        let mut row_height = None;
        let mut del = None;
        let mut ins = None;
        let mut height_rule = None;
        let mut cant_split = false;
        loop {
            let e = r.next_event();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    ignore::ignore_element(e.clone(), XMLElement::TableRowPropertyChange, r);

                    match e {
                        XMLElement::CantSplit => {
                            cant_split = read_bool(&attributes);
                        }
                        XMLElement::TableCell => {
                            cells.push(TableCell::read(r, &attributes)?);
                            continue;
                        }
                        XMLElement::GridAfter => {
                            if let Some(v) = read_val(&attributes) {
                                grid_after = Some(u32::from_str(&v)?);
                            }
                        }
                        XMLElement::WidthAfter => {
                            if let Ok(v) = read_width(&attributes) {
                                width_after = Some(v.0 as f32);
                            }
                        }
                        XMLElement::GridBefore => {
                            if let Some(v) = read_val(&attributes) {
                                grid_before = Some(u32::from_str(&v)?);
                            }
                        }
                        XMLElement::WidthBefore => {
                            if let Ok(v) = read_width(&attributes) {
                                width_before = Some(v.0 as f32);
                            }
                        }
                        XMLElement::TableRowHeight => {
                            if let Some(v) = read_val(&attributes) {
                                let h = f32::from_str(&v);
                                if let Ok(h) = h {
                                    row_height = Some(h);
                                }
                            }

                            if let Some(v) = read(&attributes, "hRule") {
                                let h = HeightRule::from_str(&v);
                                if let Ok(h) = h {
                                    height_rule = Some(h);
                                }
                            }
                        }
                        XMLElement::Delete => {
                            if let Ok(d) = Delete::read(r, &attributes) {
                                del = Some(d);
                            }
                        }
                        XMLElement::Insert => {
                            if let Ok(i) = Insert::read(r, &attributes) {
                                ins = Some(i);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableRow {
                        let mut row = TableRow::new(cells);
                        if cant_split {
                            row = row.cant_split();
                        }
                        if let Some(grid_after) = grid_after {
                            row = row.grid_after(grid_after);
                        }

                        if let Some(grid_before) = grid_before {
                            row = row.grid_before(grid_before);
                        }

                        if let Some(row_height) = row_height {
                            row = row.row_height(row_height);
                        }

                        if let Some(width_after) = width_after {
                            row = row.width_after(width_after);
                        }

                        if let Some(width_before) = width_before {
                            row = row.width_before(width_before);
                        }

                        if let Some(height_rule) = height_rule {
                            row = row.height_rule(height_rule);
                        }

                        if let Some(del) = del {
                            row = row.delete(del);
                        }

                        if let Some(ins) = ins {
                            row = row.insert(ins);
                        }

                        return Ok(row);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_row(properties: &str, cells: &str) -> TableRow {
        let xml = format!(
            r#"<w:tr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:trPr>{properties}</w:trPr>{cells}</w:tr>"#
        );
        TableRow::read(&mut EventReader::new(xml.as_bytes()), &[]).unwrap()
    }

    #[test]
    fn cant_split_reader_preserves_the_empty_enabled_element() {
        let row = read_row("<w:cantSplit/>", "<w:tc><w:p/></w:tc>");
        assert_eq!(row.property.cant_split, Some(CantSplit::default()));
        assert_eq!(row.cells.len(), 1);
        assert_eq!(
            serde_json::to_value(&row.property).unwrap()["cantSplit"],
            "cantSplit"
        );
    }

    #[test]
    fn cant_split_reader_recognizes_enabled_boolean_spellings() {
        for value in ["1", "true", "on"] {
            let row = read_row(&format!(r#"<w:cantSplit w:val="{value}"/>"#), "");
            assert_eq!(
                row.property.cant_split,
                Some(CantSplit::default()),
                "{value}"
            );
        }
    }

    #[test]
    fn cant_split_reader_leaves_false_and_absent_constraints_disabled() {
        for value in ["0", "false", "off"] {
            let row = read_row(&format!(r#"<w:cantSplit w:val="{value}"/>"#), "");
            assert_eq!(row.property.cant_split, None, "{value}");
        }
        assert_eq!(read_row("", "").property.cant_split, None);
    }

    #[test]
    fn cant_split_reader_ignores_historical_row_properties() {
        let old = r#"<w:trPrChange w:id="1" w:author="Example"><w:trPr><w:cantSplit/></w:trPr></w:trPrChange>"#;
        assert_eq!(read_row(old, "").property.cant_split, None);
        let current = format!("<w:cantSplit/>{old}");
        assert_eq!(
            read_row(&current, "").property.cant_split,
            Some(CantSplit::default())
        );
    }

    #[test]
    fn cant_split_reader_keeps_nested_row_constraints_local() {
        let row = read_row(
            "",
            "<w:tc><w:tbl><w:tr><w:trPr><w:cantSplit/></w:trPr><w:tc><w:p/></w:tc></w:tr></w:tbl><w:p/></w:tc>",
        );
        assert_eq!(row.property.cant_split, None);
        let TableRowChild::TableCell(cell) = &row.cells[0];
        let TableCellContent::Table(table) = &cell.children[0] else {
            panic!("the nested table must remain present");
        };
        let TableChild::TableRow(nested_row) = &table.rows[0];
        assert_eq!(nested_row.property.cant_split, Some(CantSplit::default()));
    }
}
