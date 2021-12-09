// c:smooth
use super::super::super::BooleanValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Smooth {
    val: BooleanValue,
}
impl Smooth {
    pub fn get_val(&self)-> &bool {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value:bool)-> &mut Smooth {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader:&mut Reader<R>,
        e:&BytesStart
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:smooth
        write_start_tag(writer, "c:smooth", vec![
            ("val", &self.val.get_value_string()),
        ], true);
    }
}
