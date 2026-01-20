use crate::{
    ledger::ledger::Ledger,
    node::{Node, error::NodeError, id::random_node_id},
    pipeline::Pipeline,
};

pub struct AddNodeOptions {
    id: Option<String>,
}

pub fn add(ledger: Ledger, options: AddNodeOptions) -> Result<Node, NodeError> {
    let id = options.id.unwrap_or_else(|| random_node_id(None));

    let node = Node {
        id,
        description: None,
        pipeline: Pipeline(Vec::new()),
    };

    ledger.add_node(&node)?;

    Ok(node)
}
