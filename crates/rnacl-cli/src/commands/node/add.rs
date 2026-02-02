use rnacl_core::node::Node;

use crate::{args::node::add::AddNodeArgs, helper::resolve_ledger, reuse_node};

pub(super) fn dispatch(args: AddNodeArgs) -> anyhow::Result<()> {
    let node = Node::new(
        args.id.map_or(Default::default(), |id| id.into()),
        args.description,
        Default::default(),
        Default::default(),
    );

    let ledger = resolve_ledger()?;
    ledger.add_node(&node)?;

    println!("{}", node.id());

    if !args.no_use {
        reuse_node::set(&ledger, node.id().clone(), None)?;
    }

    Ok(())
}
