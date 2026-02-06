use rnacl_core::{sample::Sample, snapshot::diff::SamplePresence};

pub(crate) fn display_sample_diff(sample_diff: &(Sample, SamplePresence)) {
    match sample_diff.1 {
        SamplePresence::OnlyBefore => println!("-"),
        SamplePresence::OnlyAfter => println!("+"),
        SamplePresence::Both => println!("="),
    }
    println!("{}", sample_diff.0.content());
}
