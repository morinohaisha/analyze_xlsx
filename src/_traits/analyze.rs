pub trait Extract {
    fn extract(&self) -> anyhow::Result<Vec<String>>;
}

pub trait Si {
    fn value(&self) -> Option<String>;
}
