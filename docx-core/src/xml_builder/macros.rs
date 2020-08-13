macro_rules! open {
    ($name: ident, $el_name: expr) => {
        #[allow(dead_code)]
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
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1),
                )
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2),
                )
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str, arg3: &str) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3),
                )
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr) => {
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4),
                )
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr) => {
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5),
                )
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr) => {
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5)
                        .attr($attr6, arg6),
                )
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr) => {
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5)
                        .attr($attr6, arg6)
                        .attr($attr7, arg7),
                )
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr, $attr8: expr) => {
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
            arg8: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5)
                        .attr($attr6, arg6)
                        .attr($attr7, arg7)
                        .attr($attr8, arg8),
                )
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr, $attr8: expr, $attr9: expr) => {
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
            arg8: &str,
            arg9: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5)
                        .attr($attr6, arg6)
                        .attr($attr7, arg7)
                        .attr($attr8, arg8)
                        .attr($attr9, arg9),
                )
                .expect("should write to buf");
            self
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr, $attr8: expr, $attr9: expr, $attr10: expr) => {
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
            arg8: &str,
            arg9: &str,
            arg10: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5)
                        .attr($attr6, arg6)
                        .attr($attr7, arg7)
                        .attr($attr8, arg8)
                        .attr($attr9, arg9)
                        .attr($attr10, arg10),
                )
                .expect("should write to buf");
            self
        }
    };
}

macro_rules! closed_with_child {
    ($name: ident, $el_name: expr) => {
        #[allow(dead_code)]
        pub(crate) fn $name(mut self, child: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name))
                .expect("should write to buf");
            self.writer.write(child).expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, child: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr($attr0, arg0))
                .expect("should write to buf");
            self.writer.write(child).expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, child: &str) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1),
                )
                .expect("should write to buf");
            self.writer.write(child).expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str, child: &str) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2),
                )
                .expect("should write to buf");
            self.writer.write(child).expect("should write to buf");
            self.close()
        }
    };
}

macro_rules! closed {
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
        #[allow(dead_code)]
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1),
                )
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2),
                )
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr) => {
        pub(crate) fn $name(mut self, arg0: &str, arg1: &str, arg2: &str, arg3: &str) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3),
                )
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr) => {
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4),
                )
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr) => {
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5),
                )
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr) => {
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5)
                        .attr($attr6, arg6),
                )
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr) => {
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5)
                        .attr($attr6, arg6)
                        .attr($attr7, arg7),
                )
                .expect("should write to buf");
            self.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr, $attr8: expr) => {
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn $name(
            mut self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
            arg8: &str,
        ) -> Self {
            self.writer
                .write(
                    XmlEvent::start_element($el_name)
                        .attr($attr0, arg0)
                        .attr($attr1, arg1)
                        .attr($attr2, arg2)
                        .attr($attr3, arg3)
                        .attr($attr4, arg4)
                        .attr($attr5, arg5)
                        .attr($attr6, arg6)
                        .attr($attr7, arg7)
                        .attr($attr8, arg8),
                )
                .expect("should write to buf");
            self.close()
        }
    };
}

macro_rules! closed_with_str {
    ($name: ident, $el_name: expr) => {
        #[allow(dead_code)]
        pub(crate) fn $name(mut self, val: &str) -> Self {
            self.writer
                .write(XmlEvent::start_element($el_name).attr("w:val", val))
                .expect("should write to buf");
            self.close()
        }
    };
}

macro_rules! closed_with_usize {
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
        pub(crate) fn $name(mut self, w: i32, t: WidthType) -> Self {
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
        pub(crate) fn $name(
            mut self,
            val: BorderType,
            size: usize,
            space: usize,
            color: &str,
        ) -> Self {
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
