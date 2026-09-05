pub fn beautify_xml(input: &str) -> String {
    let mut reader = quick_xml::reader::Reader::from_str(input);
    reader.config_mut().trim_text(true);
    let mut writer =
        quick_xml::writer::Writer::new_with_indent(std::io::Cursor::new(Vec::new()), b' ', 4);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf).unwrap() {
            quick_xml::events::Event::Eof => break,
            event => {
                writer.write_event(event).unwrap();
            }
        }
        buf.clear();
    }
    String::from_utf8(writer.into_inner().into_inner()).unwrap()
}

#[track_caller]
pub fn assert_xml_eq(lhs: &str, rhs: &str) {
    // comparison for humans to look at
    pretty_assertions::assert_eq!(
        beautify_xml(lhs),
        beautify_xml(rhs)
    );

    // to make sure compact serialization is also as expected
    pretty_assertions::assert_eq!(
        lhs,
        rhs
    );
}