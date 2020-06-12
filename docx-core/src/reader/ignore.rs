use std::io::Read;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};

use super::*;

pub(crate) fn ignore_element<R: Read>(el: XMLElement, ignore: XMLElement, r: &mut EventReader<R>) {
    if ignore == el {
        loop {
            let e = r.next();
            if let Ok(XmlEvent::EndElement { name, .. }) = e {
                let e = XMLElement::from_str(&name.local_name).unwrap();
                if e == ignore {
                    break;
                }
            }
        }
    }
}
