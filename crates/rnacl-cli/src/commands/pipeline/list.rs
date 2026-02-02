use crate::{
    args::pipeline::list::ListPipelineArgs,
    helper::{resolve_explicit_or_reused_node, resolve_ledger},
};

pub(super) fn dispatch(args: ListPipelineArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let node = resolve_explicit_or_reused_node(&ledger, args.node_id)?;

    for stage in node.pipeline().stages() {
        println!("{}  {}", stage.operation_id(), stage.options());
    }

    Ok(())
}
