use std::io::Read;
use std::{io, path, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;
use structs::Worksheet;
use super::driver::normalize_path;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    target: &str,
    worksheet: &mut Worksheet
)-> result::Result<(), XlsxError>{
    let data = {
        let mut r = io::BufReader::new(arv.by_name(normalize_path(&format!("xl/drawings/{}", target)).to_str().unwrap_or(""))?);
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;
        std::io::Cursor::new(buf)
    };
    let mut reader = Reader::from_reader(data);
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"xdr:wsDr" => {
                        let worksheet_drawing = worksheet.get_worksheet_drawing_mut();
                        worksheet_drawing.set_attributes(&mut reader, e, arv, target);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}
