use std::fs;

use crate::{
    ledger::Ledger,
    node::{Node, error::NodeError},
};

impl Ledger {
    pub fn load_node(&self, id: &str) -> Result<Node, NodeError> {
        let path = self.node_path(id.into());

        if !path.try_exists()? {
            return Err(NodeError::NotFound(id.to_string()));
        }

        let mut node: Node = serde_json::from_str(&fs::read_to_string(path)?)?;
        node.id = id.to_string();

        Ok(node)
    }

    pub fn save_node(&self, node: &Node) -> Result<(), NodeError> {
        let path = self.node_path(node.into());

        if !path.try_exists()? {
            return Err(NodeError::NotFound(node.id.to_string()));
        }

        fs::write(path, serde_json::to_vec_pretty(node)?)?;

        Ok(())
    }

    pub fn add_node(&self, node: &Node) -> Result<(), NodeError> {
        let path = self.node_path(node.into());

        if path.try_exists()? {
            return Err(NodeError::AlreadyExists(node.id.to_string()));
        }

        self.save_node(node)
    }

    pub fn remove_node(&self, node: &Node) -> Result<(), NodeError> {
        let path = self.node_path(node.into());

        if !path.try_exists()? {
            return Err(NodeError::NotFound(node.id.to_string()));
        }

        fs::remove_file(path)?;

        Ok(())
    }
}
