macro_rules! open {
    ($name: ident, $el_name: expr) => {
        #[allow(dead_code)]
        pub(crate) fn $name(self) -> crate::xml::writer::Result<Self> {
            self.write(XmlEvent::start_element($el_name))
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr) => {
        pub(crate) fn $name(self, arg0: &str) -> crate::xml::writer::Result<Self> {
            self.write(XmlEvent::start_element($el_name).attr($attr0, arg0))
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr) => {
        pub(crate) fn $name(self, arg0: &str, arg1: &str) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1),
            )
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2),
            )
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2)
                    .attr($attr3, arg3),
            )
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2)
                    .attr($attr3, arg3)
                    .attr($attr4, arg4),
            )
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2)
                    .attr($attr3, arg3)
                    .attr($attr4, arg4)
                    .attr($attr5, arg5),
            )
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2)
                    .attr($attr3, arg3)
                    .attr($attr4, arg4)
                    .attr($attr5, arg5)
                    .attr($attr6, arg6),
            )
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
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
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr, $attr8: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
            arg8: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
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
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr, $attr8: expr, $attr9: expr) => {
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn $name(
            self,
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
        ) -> crate::xml::writer::Result<Self> {
            self.write(
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
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr, $attr8: expr, $attr9: expr, $attr10: expr) => {
        pub(crate) fn $name(
            self,
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
        ) -> crate::xml::writer::Result<Self> {
            self.write(
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
        }
    };
}

macro_rules! closed_with_child {
    ($name: ident, $el_name: expr) => {
        #[allow(dead_code)]
        pub(crate) fn $name(self, child: &str) -> crate::xml::writer::Result<Self> {
            self.write(XmlEvent::start_element($el_name))?
                .write(child)?
                .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr) => {
        pub(crate) fn $name(self, arg0: &str, child: &str) -> crate::xml::writer::Result<Self> {
            self.write(XmlEvent::start_element($el_name).attr($attr0, arg0))?
                .write(child)?
                .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            child: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1),
            )?
            .write(child)?
            .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            child: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2),
            )?
            .write(child)?
            .close()
        }
    };
}

macro_rules! closed {
    ($name: ident, $el_name: expr) => {
        #[allow(clippy::wrong_self_convention)]
        pub(crate) fn $name(self) -> crate::xml::writer::Result<Self> {
            self.write(XmlEvent::start_element($el_name))?.close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr) => {
        #[allow(clippy::wrong_self_convention)]

        pub(crate) fn $name(self, arg0: &str) -> crate::xml::writer::Result<Self> {
            self.write(XmlEvent::start_element($el_name).attr($attr0, arg0))?
                .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr) => {
        #[allow(dead_code)]
        #[allow(clippy::wrong_self_convention)]
        pub(crate) fn $name(self, arg0: &str, arg1: &str) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1),
            )?
            .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2),
            )?
            .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2)
                    .attr($attr3, arg3),
            )?
            .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2)
                    .attr($attr3, arg3)
                    .attr($attr4, arg4),
            )?
            .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr) => {
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2)
                    .attr($attr3, arg3)
                    .attr($attr4, arg4)
                    .attr($attr5, arg5),
            )?
            .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr) => {
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2)
                    .attr($attr3, arg3)
                    .attr($attr4, arg4)
                    .attr($attr5, arg5)
                    .attr($attr6, arg6),
            )?
            .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr) => {
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr($attr0, arg0)
                    .attr($attr1, arg1)
                    .attr($attr2, arg2)
                    .attr($attr3, arg3)
                    .attr($attr4, arg4)
                    .attr($attr5, arg5)
                    .attr($attr6, arg6)
                    .attr($attr7, arg7),
            )?
            .close()
        }
    };
    ($name: ident, $el_name: expr, $attr0: expr, $attr1: expr, $attr2: expr, $attr3: expr, $attr4: expr, $attr5: expr, $attr6: expr, $attr7: expr, $attr8: expr) => {
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn $name(
            self,
            arg0: &str,
            arg1: &str,
            arg2: &str,
            arg3: &str,
            arg4: &str,
            arg5: &str,
            arg6: &str,
            arg7: &str,
            arg8: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
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
            )?
            .close()
        }
    };
}

macro_rules! closed_with_str {
    ($name: ident, $el_name: expr) => {
        #[allow(dead_code)]
        pub(crate) fn $name(self, val: &str) -> crate::xml::writer::Result<Self> {
            self.write(XmlEvent::start_element($el_name).attr("w:val", val))?
                .close()
        }
    };
}

macro_rules! closed_with_usize {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(self, val: usize) -> crate::xml::writer::Result<Self> {
            self.write(XmlEvent::start_element($el_name).attr("w:val", &format!("{}", val)))?
                .close()
        }
    };
}

macro_rules! closed_with_isize {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(self, val: isize) -> crate::xml::writer::Result<Self> {
            self.write(XmlEvent::start_element($el_name).attr("w:val", &format!("{}", val)))?
                .close()
        }
    };
}

macro_rules! closed_w_with_type_el {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(self, w: i32, t: WidthType) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr("w:w", &format!("{}", w))
                    .attr("w:type", &t.to_string()),
            )?
            .close()
        }
    };
}

macro_rules! closed_border_el {
    ($name: ident, $el_name: expr) => {
        pub(crate) fn $name(
            self,
            val: BorderType,
            size: usize,
            space: usize,
            color: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr("w:val", &val.to_string())
                    .attr("w:sz", &format!("{}", size))
                    .attr("w:space", &format!("{}", space))
                    .attr("w:color", color),
            )?
            .close()
        }
    };
}

macro_rules! closed_paragraph_border_el {
    ($name: ident, $ el_name: expr) => {
        pub(crate) fn $name(
            self,
            val: &str,
            space: &str,
            size: &str,
            color: &str,
        ) -> crate::xml::writer::Result<Self> {
            self.write(
                XmlEvent::start_element($el_name)
                    .attr("w:val", val)
                    .attr("w:space", space)
                    .attr("w:sz", size)
                    .attr("w:color", color),
            )?
            .close()
        }
    };
}
