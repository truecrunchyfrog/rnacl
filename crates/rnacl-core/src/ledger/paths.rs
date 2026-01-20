use std::path::PathBuf;

use crate::{ledger::ledger::Ledger, node::Node};

pub struct NodeId<'a>(&'a str);

impl<'a> From<&'a str> for NodeId<'a> {
    fn from(value: &'a str) -> Self {
        NodeId(value)
    }
}

impl<'a> From<&'a Node> for NodeId<'a> {
    fn from(value: &'a Node) -> Self {
        NodeId(&value.id)
    }
}

impl Ledger {
    pub fn rnacl_dir(&self) -> PathBuf {
        self.work_dir.join(".rnacl")
    }

    pub fn nodes_dir(&self) -> PathBuf {
        self.rnacl_dir.join("nodes")
    }

    pub fn node_path(&self, id: NodeId) -> PathBuf {
        self.nodes_dir().join(id.0)
    }
}
