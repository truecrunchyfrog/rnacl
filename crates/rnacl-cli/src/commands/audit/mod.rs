use rnacl_core::snapshot::diff::SamplePresence;

use crate::{args::audit::AuditArgs, helper::resolve_ledger, ui};

pub(super) fn dispatch(args: AuditArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let audit = ledger.capture_snapshot()?;
    let baseline = ledger.load_baseline_snapshot()?;
    let diffs = audit.diff(baseline);
    let mut unacked_nodes: usize = 0;

    for (node_id, (diff, dependency_diffs)) in diffs {
        let sample_diff = diff.sample_diff();
        let mut attributes = Vec::<String>::new();
        let mut needs_resolve = false;

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
                needs_resolve = true;
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
                needs_resolve = true;
                attributes.push(format!("stale ({})", stale));
            }
        }

        if needs_resolve {
            unacked_nodes += 1;
            println!("{}: {}", node_id, attributes.join(", "));
        }
    }

    match unacked_nodes {
        0 => ui::info(format!("ok!")),
        amount => ui::info(format!("{amount} not ack'd")),
    }

    if !args.dry {
        ledger.save_pending_snapshot(audit)?;
    }

    Ok(())
}
