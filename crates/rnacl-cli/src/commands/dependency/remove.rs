use crate::{args::dependency::remove::RemoveDependencyArgs, helper::resolve_ledger};

pub(super) fn dispatch(args: RemoveDependencyArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;

    Ok(())
}
