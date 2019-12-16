pub(crate) fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('\n', "&#xA;")
        // If \r escape to &#xD, this cause error in libreoffice
        // .replace('\r', "&#xD;")
        .replace('\r', "")
}
