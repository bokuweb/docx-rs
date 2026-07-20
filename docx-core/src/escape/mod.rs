pub(crate) fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            '\n' => escaped.push_str("&#xA;"),
            // Escaping carriage returns causes errors in LibreOffice.
            '\r' => {}
            _ => escaped.push(ch),
        }
    }
    escaped
}

/// Escapes an owned XML value without reallocating plain text.
///
/// The original `String` is returned unchanged when it contains no character
/// that needs escaping or removal.
pub(crate) fn escape_owned(text: String) -> String {
    if text
        .as_bytes()
        .iter()
        .any(|byte| matches!(byte, b'&' | b'<' | b'>' | b'"' | b'\'' | b'\n' | b'\r'))
    {
        escape(&text)
    } else {
        text
    }
}

pub(crate) fn replace_escaped(text: &str) -> String {
    let mut decoded = String::with_capacity(text.len());
    let mut rest = text;

    while let Some(index) = rest.find('&') {
        decoded.push_str(&rest[..index]);
        rest = &rest[index..];
        let (replacement, consumed) = if rest.starts_with("&lt;") {
            (Some("<"), 4)
        } else if rest.starts_with("&gt;") {
            (Some(">"), 4)
        } else if rest.starts_with("&amp;") {
            (Some("&"), 5)
        } else if rest.starts_with("&quot;") {
            (Some("\""), 6)
        } else if rest.starts_with("&#39;") {
            (Some("'"), 5)
        } else if rest.starts_with("&apos;") {
            (Some("'"), 6)
        } else if rest.starts_with("&nbsp;") {
            (Some(" "), 6)
        } else {
            (None, 1)
        };
        decoded.push_str(replacement.unwrap_or("&"));
        rest = &rest[consumed..];
    }
    decoded.push_str(rest);
    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_all_supported_characters_in_one_pass() {
        assert_eq!(
            escape("<&>\"'\n\rplain"),
            "&lt;&amp;&gt;&quot;&apos;&#xA;plain"
        );
    }

    #[test]
    fn preserves_the_allocation_for_plain_owned_text() {
        let text = String::from("plain text");
        let pointer = text.as_ptr();
        let escaped = escape_owned(text);

        assert_eq!(escaped, "plain text");
        assert_eq!(escaped.as_ptr(), pointer);
    }

    #[test]
    fn escapes_special_characters_in_owned_text() {
        assert_eq!(
            escape_owned("<&>\"'\n\r".to_owned()),
            "&lt;&amp;&gt;&quot;&apos;&#xA;"
        );
    }

    #[test]
    fn replaces_entities_without_recursively_decoding_them() {
        assert_eq!(replace_escaped("&lt;&amp;&apos;&nbsp;"), "<&' ");
        assert_eq!(replace_escaped("&amp;lt;"), "&lt;");
        assert_eq!(replace_escaped("plain &unknown;"), "plain &unknown;");
    }
}
