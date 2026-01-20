use std::path::{Path, PathBuf};

use crate::ledger::{
    error::LedgerError,
    init::{self},
};

pub struct Ledger {
    pub work_dir: PathBuf,
}

impl Ledger {
    pub fn init(path: &Path) -> Result<Self, LedgerError> {
        init::init(path)
    }
}
