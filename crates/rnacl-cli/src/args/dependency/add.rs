use clap::Args;

/// Create one or more dependencies.
#[derive(Args)]
pub(crate) struct AddDependencyArgs {
    /// IDs of nodes to add the dependencies to.
    #[arg(required = true)]
    pub(crate) node_ids: Vec<String>,

    /// IDs of nodes to add as dependencies.
    #[arg(short, long)]
    pub(crate) dependency: Vec<String>,

    /// IDs of nodes to copy all dependencies from to add.
    #[arg(long)]
    pub(crate) from: Vec<String>,

    /// Make the dependents dependencies of each other (when there are more than one dependents).
    #[arg(long)]
    pub(crate) mirror: bool,
}
