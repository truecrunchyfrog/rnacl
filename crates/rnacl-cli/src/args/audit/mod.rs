use clap::Args;

#[derive(Args)]
pub(crate) struct AuditArgs {
    /// Don't set the audit as pending state.
    #[arg(long)]
    pub(crate) dry: bool,
}
