use std::{fs, path::Path};

use log::info;

use crate::ledger::{error::LedgerError, ledger::Ledger};

pub fn init(path: &Path) -> Result<Ledger, LedgerError> {
    let ledger = Ledger {
        work_dir: path.to_path_buf(),
    };

    if ledger.rnacl_dir().try_exists()? {
        if ledger.rnacl_dir().is_dir() {
            return Err(LedgerError::AlreadyExists(ledger.rnacl_dir()));
        }

        return Err(LedgerError::PathNotDirectory(ledger.rnacl_dir()));
    }

    init_ledger(&ledger);

    Ok(ledger)
}

fn init_ledger(ledger: &Ledger) -> Result<(), LedgerError> {
    let rnacl_dir = ledger.rnacl_dir();
    info!("creating {:#?}", rnacl_dir);
    fs::create_dir(rnacl_dir)?;

    let nodes_dir = ledger.nodes_dir();
    info!("creating {:#?}", nodes_dir);
    fs::create_dir(nodes_dir)?;

    Ok(())
}
