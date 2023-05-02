use crate::{_structs::zip::XlsxReader, _traits::zip::XlsxArchive};
use std::{fs::File, io::Read, path::Path};
use zip::{read::ZipFile, ZipArchive};

impl XlsxReader {
    pub fn new(template: &str) -> anyhow::Result<XlsxReader> {
        let path = Path::new(template);
        let zipfile = File::open(path)?;
        let reader = ZipArchive::new(zipfile)?;
        Ok(XlsxReader { reader })
    }
}

impl XlsxArchive for XlsxReader {
    fn get_file(&mut self, file_name: &str, buf: &mut String) -> anyhow::Result<String> {
        for i in 0..self.reader.len() {
            let mut file: ZipFile = self.reader.by_index(i)?;
            let name = file.name();
            if name == file_name {
                let _ = file.read_to_string(buf);
                break;
            }
        }
        Ok("ok".to_string())
    }
    fn is_empty(&self) -> bool {
        self.reader.len() == 0
    }
}
