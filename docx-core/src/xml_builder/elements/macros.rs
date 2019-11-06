macro_rules! only_str_val_el {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(mut self, val: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr("w:val", val))
                .expect("should write to buf");
            self.close()
        }
    };
}

macro_rules! only_usize_val_el {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(mut self, val: usize) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr("w:val", val))
                .expect("should write to buf");
            self.close()
        }
    };
}
