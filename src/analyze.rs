use crate::_structs::_xl::shared_strings;
use crate::_structs::zip::XlsxReader;
use crate::_traits::_xl::shared_strings::Extract;
use crate::_traits::zip::XlsxArchive;
use quick_xml::de::from_str;

pub fn analyze(input_file: &str) -> anyhow::Result<String> {
    let mut reader = XlsxReader::new(input_file)?;

    let mut buf: String = String::new();
    reader.get_file("xl/sharedStrings.xml", &mut buf)?;
    let x: shared_strings::sst = from_str(buf.as_str())?;
    let lst = &x.extract()?;
    let res = &serde_json::to_string(lst)?;
    Ok(res.to_string())
}
