use crate::receipt::{Receipt, TraceEvent};
use std::path::PathBuf;

pub struct Bundle {
    pub path: PathBuf,
    pub events: Vec<TraceEvent>,
    pub receipts: Vec<Receipt>,
}

impl Bundle {
    pub fn init(path: PathBuf) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&path)?;
        Ok(Self {
            path,
            events: Vec::new(),
            receipts: Vec::new(),
        })
    }
}
