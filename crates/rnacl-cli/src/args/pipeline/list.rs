use clap::Args;

/// List a pipeline's stages.
#[derive(Args)]
pub(crate) struct ListPipelineArgs {
    /// ID of node.
    pub(crate) node_id: Option<String>,
}
