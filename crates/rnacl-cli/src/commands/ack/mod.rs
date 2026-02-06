use std::io::stdin;

use anyhow::anyhow;
use rnacl_core::snapshot::{SnapshotEntry, diff::SamplePresence};

use crate::{
    args::ack::AckArgs,
    helper::{resolve_explicit_or_reused_node_id, resolve_ledger},
    output, ui,
};

pub(super) fn dispatch(args: AckArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let node_ids = args
        .node_id
        .into_iter()
        .map(|node_id| resolve_explicit_or_reused_node_id(&ledger, Some(node_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let audit = ledger.load_pending_snapshot()?;
    let mut baseline = ledger.load_baseline_snapshot()?;
    let mut diffs = audit.diff(baseline.clone());

    let node_ids = if !node_ids.is_empty() {
        node_ids
    } else {
        audit
            .entries()
            .keys()
            .map(|node_id| node_id.clone())
            .collect::<Vec<_>>()
    };

    for node_id in node_ids {
        let (diff, dependency_diffs) = diffs
            .remove(&node_id)
            .ok_or_else(|| anyhow!("node absent in both pending and baseline snapshot"))?;

        let baseline_entry = baseline
            .entries_mut()
            .entry(node_id)
            .or_insert_with(|| SnapshotEntry::new(Default::default(), Default::default()));

        if args.dependency.is_empty() {
            let working_batch = baseline_entry.batch_mut();

            for sample_diff in diff.sample_diff() {
                if sample_diff.1 == SamplePresence::Both {
                    continue;
                }

                let should_ack = if !args.all {
                    output::display_sample_diff(&sample_diff);
                    should_ack_prompt()?
                } else {
                    true
                };

                if should_ack {
                    match sample_diff {
                        (sample, SamplePresence::OnlyBefore) => {
                            working_batch.samples_mut().retain(|s| s != &sample)
                        }
                        (sample, SamplePresence::OnlyAfter) => {
                            working_batch.samples_mut().push(sample)
                        }
                        _ => (),
                    }
                }
            }
        } else {
            let working_batches = baseline_entry.dependencies_mut();

            for (node_id, batch) in working_batches {
                println!("dependency: {}", node_id);

                for sample_diff in diff.sample_diff() {
                    if sample_diff.1 == SamplePresence::Both {
                        continue;
                    }

                    let should_ack = if !args.all {
                        output::display_sample_diff(&sample_diff);
                        should_ack_prompt()?
                    } else {
                        true
                    };

                    if should_ack {
                        match sample_diff {
                            (sample, SamplePresence::OnlyBefore) => {
                                working_batch.samples_mut().retain(|s| s != &sample)
                            }
                            (sample, SamplePresence::OnlyAfter) => {
                                working_batch.samples_mut().push(sample)
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    ledger.save_baseline_snapshot(baseline)?;

    Ok(())
}

fn prompt_sample_diff(sample_diff: (Sample, SamplePresence)) {}

fn should_ack_prompt() -> anyhow::Result<bool> {
    loop {
        println!("ack? y/n: ");

        let mut choice = String::new();
        stdin().read_line(&mut choice)?;

        match choice.trim() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            input => {
                ui::warn(format!("invalid input '{input}'"));
            }
        }
    }
}
