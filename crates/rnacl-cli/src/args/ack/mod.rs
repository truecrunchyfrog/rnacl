use clap::Args;

#[derive(Args)]
pub(crate) struct AckArgs {
    /// ID of node to ack.
    pub(crate) node_id: Vec<String>,

    /// ID of dependency node to ack, instead of acking the node itself.
    #[arg(short, long)]
    pub(crate) dependency: Vec<String>,

    /// Ack all the samples in the batch instead of selecting interactively.
    #[arg(short, long)]
    pub(crate) all: bool,
}
