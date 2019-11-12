macro_rules! opened_el {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(mut self) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name))
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr) => {
        pub(crate) fn $name(mut self, arg0: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0))
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1))
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1).attr($attr2, arg2))
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str, arg3: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1).attr($attr2, arg2).attr($attr3, arg3))
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str, arg3: &str, arg4: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1).attr($attr2, arg2).attr($attr3, arg3).attr($attr4, arg4))
                .expect("should write to buf");
            self
        }
    };
}

macro_rules! closed_el_with_child {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(mut self, child: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name))
                .expect("should write to buf");
            self.writer
                .write(child)
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, child: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0))
                .expect("should write to buf");
            self.writer
                .write(child)
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, child: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1))
                .expect("should write to buf");
            self.writer
                .write(child)
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str, child: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1).attr($attr2, arg2))
                .expect("should write to buf");
            self.writer
                .write(child)
                .expect("should write to buf");
            self.close()
        }
    };
}

macro_rules! closed_el {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(mut self) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name))
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr) => {
        pub(crate) fn $name(mut self, arg0: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0))
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1))
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0).attr($attr1, arg1).attr($attr2, arg2))
                .expect("should write to buf");
            self.close()
        }
    };
}

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
                .write(XmlEvent::start_element($el_name).attr("w:val", &format!("{}", val)))
                .expect("should write to buf");
            self.close()
        }
    };
}

macro_rules! closed_w_with_type_el {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(mut self, w: usize, t: WidthType) -> Self {
        self.writer
            .write(
                XmlEvent::start_element($el_name)
                    .attr("w:w", &format!("{}", w))
                    .attr("w:type", &t.to_string()),
            )
            .expect(EXPECT_MESSAGE);
        self.close()
        }
    };
}

macro_rules! closed_border_el {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(mut self, val: BorderType, size: usize, space: usize, color: &str) -> Self {
        self.writer
            .write(
                XmlEvent::start_element($el_name)
                    .attr("w:val", &val.to_string())
                    .attr("w:sz", &format!("{}", size))
                    .attr("w:space", &format!("{}", space))
                    .attr("w:color", color),
            )
            .expect(EXPECT_MESSAGE);
        self.close()
        }
    };
}
