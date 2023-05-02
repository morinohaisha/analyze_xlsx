#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait XlsxArchive {
    fn get_file(&mut self, file_name: &str, buf: &mut String) -> anyhow::Result<String>;
    fn is_empty(&self) -> bool;
}
