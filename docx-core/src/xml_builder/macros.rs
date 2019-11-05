macro_rules! opened_el {
    ($name: ident, $el_name: expr, $attr0: expr) => {
        pub(crate) fn $name(mut self, arg0: &str) -> Self {
            self.writer
                .write(super::XmlEvent::start_element($el_name).attr($attr0, arg0))
                .expect("should write to buf");
            self
        }
    };
}

macro_rules! closed_el {
    ($name: ident, $el_name: expr, $attr0: expr) => {
        pub(crate) fn $name(mut self, arg0: &str) -> Self {
            self.writer
                .write(super::XmlEvent::start_element($el_name).attr($attr0, arg0))
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str) -> Self {
            self.writer
                .write(super::XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1))
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str) -> Self {
            self.writer
                .write(super::XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1).attr($attr2, arg2))
                .expect("should write to buf");
            self.close()
        }
    };
}
