use rnacl_core::snapshot::diff::SamplePresence;

use crate::{args::audit::AuditArgs, helper::resolve_ledger};

pub(super) fn dispatch(args: AuditArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let audit = ledger.capture_snapshot()?;
    let baseline = ledger.load_baseline_snapshot()?;
    let diffs = audit.diff(baseline);

    for (node_id, (diff, dependency_diffs)) in diffs {
        let sample_diff = diff.sample_diff();
        let mut attributes = Vec::<String>::new();

        {
            let dirty = (
                sample_diff
                    .iter()
                    .filter(|(_, p)| p == &SamplePresence::OnlyBefore)
                    .count(),
                sample_diff
                    .iter()
                    .filter(|(_, p)| p == &SamplePresence::OnlyAfter)
                    .count(),
            );

            if dirty != (0, 0) {
                attributes.push(format!("dirty (-{} +{})", dirty.0, dirty.1));
            }
        }

        {
            let stale = dependency_diffs
                .into_iter()
                .map(|(_, diff)| diff.sample_diff())
                .filter(|samples| samples.iter().any(|(_, p)| p != &SamplePresence::Both))
                .count();

            if stale != 0 {
                attributes.push(format!("stale ({})", stale));
            }
        }

        if !attributes.is_empty() {
            println!("{}: {}", node_id, attributes.join(", "));
        }
    }

    if !args.dry {
        ledger.save_pending_snapshot(audit)?;
    }

    Ok(())
}
